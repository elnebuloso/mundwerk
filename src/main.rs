#![windows_subsystem = "windows"] // no console window on Windows

use std::ffi::CStr;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample};
use eframe::egui;
use egui::text::{LayoutJob, TextFormat};
use sha2::{Digest, Sha256};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// (file name, purpose, download size)
const MODELS: &[(&str, &str, &str)] = &[
    ("large-v3-turbo-q5_0", "Empfohlen", "0,5 GB"),
    ("large-v3-turbo", "Genauer, langsamer", "1,6 GB"),
    ("base", "Am schnellsten, ungenauer", "0,15 GB"),
];
const LANGS: &[(&str, &str)] = &[("de", "Deutsch"), ("en", "Englisch"), ("auto", "Automatisch")];
const PRESENT: egui::Color32 = egui::Color32::from_rgb(80, 200, 120);
const MISSING: egui::Color32 = egui::Color32::from_rgb(230, 150, 40);
const MIC_HINT: &str = "Mikrofon prüfen – unter Windows: Einstellungen > Datenschutz und Sicherheit > Mikrofon";
// below about -40 dBFS throughout: nothing but noise floor, and Whisper would invent text for it
const SILENCE: f32 = 0.01;

enum Msg {
    /// Some((done, total)) while downloading, None while loading into memory.
    Progress(Option<(u64, u64)>),
    Log(String),
    Loaded(Arc<WhisperContext>, String),
    Text(String),
    Error(String),
}

struct Recording {
    _stream: cpal::Stream,
    samples: Arc<Mutex<Vec<f32>>>,
    rate: u32,
    device: String,
}

struct App {
    model: usize,
    lang: usize,
    ctx: Option<Arc<WhisperContext>>,
    loaded: String,
    loading: bool,
    progress: Option<(u64, u64)>,
    cancel: Arc<AtomicBool>,
    transcribing_since: Option<Instant>,
    audio_secs: f32,
    rec: Option<Recording>,
    text: String,
    cursor: Option<usize>,
    move_cursor: bool,
    last_cleared: Option<String>,
    copied_at: Option<Instant>,
    status: String,
    error: Option<String>,
    started: Instant,
    log: Vec<String>,
    open_log: bool,
    recording_title: bool,
    device: String,
    tx: Sender<Msg>,
    rx: Receiver<Msg>,
}

fn app_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".mundwerk")
}

fn model_path(name: &str) -> PathBuf {
    app_dir().join("models").join(format!("ggml-{name}.bin"))
}

fn model_url(name: &str) -> String {
    format!("https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{name}.bin")
}

/// Names the device whisper.cpp will compute on, and why.
fn compute_device() -> String {
    if !cfg!(feature = "vulkan") {
        return "CPU – ohne GPU-Unterstützung gebaut".into();
    }
    use whisper_rs::whisper_rs_sys as sys;
    // same choice as whisper.cpp's whisper_backend_init_gpu: the first GPU or integrated GPU
    for i in 0..unsafe { sys::ggml_backend_dev_count() } {
        let dev = unsafe { sys::ggml_backend_dev_get(i) };
        let kind = unsafe { sys::ggml_backend_dev_type(dev) };
        if kind == sys::ggml_backend_dev_type_GGML_BACKEND_DEVICE_TYPE_GPU
            || kind == sys::ggml_backend_dev_type_GGML_BACKEND_DEVICE_TYPE_IGPU
        {
            let desc = unsafe { CStr::from_ptr(sys::ggml_backend_dev_description(dev)) }.to_string_lossy();
            let name = unsafe { CStr::from_ptr(sys::ggml_backend_dev_name(dev)) }.to_string_lossy();
            return format!("GPU – {desc} ({name}), automatisch gewählt");
        }
    }
    "CPU – keine Vulkan-fähige GPU gefunden".into()
}

/// Downloads the model from Hugging Face if missing, then loads it into memory.
fn load_model(name: String, cancel: Arc<AtomicBool>, tx: Sender<Msg>, ui: egui::Context) {
    let send = |m| {
        let _ = tx.send(m);
        ui.request_repaint();
    };
    let result = (|| -> Result<Arc<WhisperContext>, String> {
        let path = model_path(&name);
        if path.exists() {
            send(Msg::Log(format!("Vorhanden: {}", path.display())));
        } else {
            download(&name, &path, &cancel, &send)?;
        }
        send(Msg::Progress(None));
        send(Msg::Log(format!("Lade {name} in den Speicher ...")));
        let ctx = WhisperContext::new_with_params(path.to_str().ok_or("Pfad ungültig")?, WhisperContextParameters::default())
            .map_err(|e| format!("Modell konnte nicht geladen werden: {e}"))?;
        Ok(Arc::new(ctx))
    })();
    send(match result {
        Ok(ctx) => Msg::Loaded(ctx, name),
        Err(e) => Msg::Error(e),
    });
}

fn download(name: &str, path: &Path, cancel: &AtomicBool, send: &impl Fn(Msg)) -> Result<(), String> {
    let url = model_url(name);
    std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    send(Msg::Log(format!("Lade herunter: {url}")));
    // don't follow the redirect: only Hugging Face's own answer carries the file's SHA-256
    let agent: ureq::Agent = ureq::Agent::config_builder().max_redirects(0).build().into();
    let (expected, location) = {
        let first = agent.get(&url).call().map_err(|e| format!("Download fehlgeschlagen: {e}"))?;
        let header = |k: &str| first.headers().get(k).and_then(|v| v.to_str().ok()).map(str::to_owned);
        let expected = header("x-linked-etag").ok_or("Hugging Face hat keine Prüfsumme geliefert")?;
        let location = header("location").ok_or("Hugging Face hat keine Download-Adresse geliefert")?;
        (expected.trim_matches('"').to_string(), location)
    };
    let resp = ureq::get(&location).call().map_err(|e| format!("Download fehlgeschlagen: {e}"))?;
    let total: u64 = resp.headers().get("content-length").and_then(|v| v.to_str().ok()?.parse().ok()).unwrap_or(0);
    let mut body = resp.into_body().into_reader();
    // write to .part first so an aborted download never leaves a broken model behind
    let part = path.with_extension("part");
    let mut file = std::fs::File::create(&part).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    let (mut done, mut buf, mut last, mut logged_pct) = (0u64, vec![0u8; 1 << 16], 0u64, 0u64);
    loop {
        if cancel.load(Ordering::Relaxed) {
            drop(file);
            let _ = std::fs::remove_file(&part);
            return Err("Download abgebrochen".into());
        }
        let n = body.read(&mut buf).map_err(|e| format!("Download abgebrochen: {e}"))?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n]).map_err(|e| e.to_string())?;
        hasher.update(&buf[..n]);
        done += n as u64;
        if done - last > 1 << 20 {
            last = done;
            send(Msg::Progress(Some((done, total))));
        }
        if total > 0 && done * 100 / total >= logged_pct + 10 {
            logged_pct = done * 100 / total / 10 * 10;
            send(Msg::Log(format!("Download {logged_pct} % ({} / {} MB)", done >> 20, total >> 20)));
        }
    }
    drop(file);
    if total > 0 && done != total {
        return Err("Download unvollständig".into());
    }
    let actual: String = hasher.finalize().iter().map(|b| format!("{b:02x}")).collect();
    if actual != expected {
        let _ = std::fs::remove_file(&part);
        return Err(format!("Prüfsumme falsch, Datei verworfen: erwartet {expected}, erhalten {actual}"));
    }
    send(Msg::Log(format!("Prüfsumme SHA-256 stimmt: {actual}")));
    std::fs::rename(&part, path).map_err(|e| e.to_string())?;
    send(Msg::Log(format!("Gespeichert: {}", path.display())));
    Ok(())
}

fn transcribe(ctx: &WhisperContext, audio: &[f32], lang: &str) -> Result<String, String> {
    let mut state = ctx.create_state().map_err(|e| e.to_string())?;
    let mut p = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    p.set_language(Some(lang));
    p.set_n_threads(std::thread::available_parallelism().map_or(4, |n| n.get()) as i32);
    p.set_no_context(true);
    // size the audio context to the recording instead of the fixed 30 s window: measured 126 s -> 17 s for 5 s of audio on an i5-3570K
    p.set_audio_ctx(((audio.len() * 1500 / (16000 * 30)) + 64).min(1500) as i32);
    p.set_suppress_blank(true);
    p.set_print_progress(false);
    p.set_print_realtime(false);
    p.set_print_timestamps(false);
    state.full(p, audio).map_err(|e| e.to_string())?;
    let text: String = state.as_iter().filter_map(|s| s.to_str_lossy().ok().map(|t| t.into_owned())).collect();
    Ok(text.trim().to_string())
}

/// Resamples mono audio to the 16 kHz Whisper expects.
// shortcut: linear interpolation without a low-pass filter, good enough for speech; switch to rubato if quality suffers
fn resample(input: &[f32], from: u32) -> Vec<f32> {
    if from == 16000 || input.is_empty() {
        return input.to_vec();
    }
    let ratio = from as f64 / 16000.0;
    let len = (input.len() as f64 / ratio) as usize;
    (0..len)
        .map(|i| {
            let pos = i as f64 * ratio;
            let j = pos as usize;
            let frac = (pos - j as f64) as f32;
            let a = input[j];
            let b = *input.get(j + 1).unwrap_or(&a);
            a + (b - a) * frac
        })
        .collect()
}

fn is_silent(audio: &[f32]) -> bool {
    audio.iter().all(|s| s.abs() < SILENCE)
}

/// Input level of the samples as 0..1 on a -60..0 dBFS scale.
fn level(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let rms = (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt();
    ((20.0 * rms.max(1e-6).log10() + 60.0) / 60.0).clamp(0.0, 1.0)
}

/// Inserts dictated text at a char position, adding a space on either side where words would touch.
/// Returns the char position right after the inserted text.
fn insert_dictation(text: &mut String, at_char: usize, new: &str) -> usize {
    let at = text.char_indices().nth(at_char).map_or(text.len(), |(i, _)| i);
    let before = text[..at].chars().next_back();
    let after = text[at..].chars().next();
    let mut piece = String::new();
    if before.is_some_and(|c| !c.is_whitespace()) {
        piece.push(' ');
    }
    piece.push_str(new);
    if after.is_some_and(|c| !c.is_whitespace()) {
        piece.push(' ');
    }
    text.insert_str(at, &piece);
    text[..at + piece.len()].chars().count()
}

fn build_stream<T: SizedSample>(
    dev: &cpal::Device,
    cfg: &cpal::StreamConfig,
    samples: Arc<Mutex<Vec<f32>>>,
) -> Result<cpal::Stream, cpal::Error>
where
    f32: FromSample<T>,
{
    let ch = cfg.channels as usize;
    dev.build_input_stream(
        cfg.clone(),
        move |data: &[T], _: &_| {
            let mut s = samples.lock().unwrap();
            // average channels down to mono
            s.extend(data.chunks(ch).map(|f| f.iter().map(|x| x.to_sample::<f32>()).sum::<f32>() / ch as f32));
        },
        |e| eprintln!("audio stream error: {e}"),
        None,
    )
}

fn start_recording() -> Result<Recording, String> {
    let dev = cpal::default_host().default_input_device().ok_or("Kein Mikrofon gefunden")?;
    let device = dev.description().map_or_else(|_| "unbekannt".into(), |d| d.name().to_string());
    let sup = dev.default_input_config().map_err(|e| e.to_string())?;
    let cfg: cpal::StreamConfig = sup.config();
    let samples = Arc::new(Mutex::new(Vec::new()));
    let s = samples.clone();
    let stream = match sup.sample_format() {
        cpal::SampleFormat::F32 => build_stream::<f32>(&dev, &cfg, s),
        cpal::SampleFormat::I16 => build_stream::<i16>(&dev, &cfg, s),
        cpal::SampleFormat::I32 => build_stream::<i32>(&dev, &cfg, s),
        cpal::SampleFormat::U16 => build_stream::<u16>(&dev, &cfg, s),
        f => return Err(format!("Audioformat {f} wird nicht unterstützt")),
    }
    .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok(Recording { _stream: stream, samples, rate: cfg.sample_rate, device })
}

/// `with_state` spells out whether the model is on disk; the closed combo box leaves that to the dot and the button beside it.
fn model_label(style: &egui::Style, i: usize, with_state: bool) -> LayoutJob {
    let (name, purpose, size) = MODELS[i];
    let font = egui::TextStyle::Button.resolve(style);
    let (dot, color, state) =
        if model_path(name).exists() { ("●", PRESENT, "vorhanden") } else { ("○", MISSING, "noch nicht heruntergeladen") };
    let mut job = LayoutJob::default();
    job.append(dot, 0.0, TextFormat::simple(font.clone(), color));
    let text = if with_state { format!("{purpose} · {name} · {size} – {state}") } else { format!("{purpose} · {name} · {size}") };
    job.append(&text, 6.0, TextFormat::simple(font, style.visuals.text_color()));
    job
}

fn decimal(secs: f32) -> String {
    format!("{secs:.1}").replace('.', ",")
}

impl App {
    fn new(ui_ctx: &egui::Context) -> Self {
        let (tx, rx) = channel();
        let mut app = App {
            model: 0,
            lang: 0,
            ctx: None,
            loaded: String::new(),
            loading: false,
            progress: None,
            cancel: Arc::new(AtomicBool::new(false)),
            transcribing_since: None,
            audio_secs: 0.0,
            rec: None,
            text: String::new(),
            cursor: None,
            move_cursor: false,
            last_cleared: None,
            copied_at: None,
            status: String::new(),
            error: None,
            started: Instant::now(),
            log: Vec::new(),
            open_log: false,
            recording_title: false,
            device: compute_device(),
            tx,
            rx,
        };
        let dir = app_dir();
        if !dir.exists() {
            app.open_log = true;
            match std::fs::create_dir_all(&dir) {
                Ok(()) => app.log(format!("Ordner angelegt: {}", dir.display())),
                Err(e) => app.log(format!("Ordner {} konnte nicht angelegt werden: {e}", dir.display())),
            }
        }
        app.log(format!("Mundwerk {} gestartet, Daten in {}", env!("CARGO_PKG_VERSION"), dir.display()));
        app.log(format!("Berechnung läuft auf: {}", app.device));
        if model_path(MODELS[0].0).exists() {
            app.start_load(ui_ctx);
        } else {
            app.status = "Modell herunterladen, um loszulegen".into();
        }
        app
    }

    /// Appends a line to the window's log and, with time since start, to `~/.mundwerk/mundwerk.log`.
    // shortcut: seconds since start instead of wall-clock time, saves a time-zone crate; add one if the log file needs dates
    fn log(&mut self, line: String) {
        let file = std::fs::OpenOptions::new().create(true).append(true).open(app_dir().join("mundwerk.log"));
        // a log file that can't be written must not stop dictation
        if let Ok(mut f) = file {
            let _ = writeln!(f, "{:>7.1} s  {line}", self.started.elapsed().as_secs_f32());
        }
        self.log.push(line);
    }

    fn fail(&mut self, error: String) {
        self.log(format!("Fehler: {error}"));
        self.error = Some(error);
        self.open_log = true;
    }

    fn selected(&self) -> &'static str {
        MODELS[self.model].0
    }

    fn can_record(&self) -> bool {
        self.ctx.is_some() && self.loaded == self.selected() && !self.loading && self.transcribing_since.is_none()
    }

    fn start_load(&mut self, ui_ctx: &egui::Context) {
        let name = self.selected();
        self.error = None;
        self.loading = true;
        self.progress = None;
        self.cancel.store(false, Ordering::Relaxed);
        if !model_path(name).exists() {
            self.open_log = true;
        }
        let (cancel, tx, ui) = (self.cancel.clone(), self.tx.clone(), ui_ctx.clone());
        std::thread::spawn(move || load_model(name.into(), cancel, tx, ui));
    }

    fn toggle(&mut self, ui_ctx: &egui::Context) {
        self.error = None;
        match self.rec.take() {
            None => match start_recording() {
                Ok(r) => {
                    self.log(format!("Aufnahme über Mikrofon: {}", r.device));
                    self.rec = Some(r);
                    self.status = "Aufnahme läuft …".into();
                }
                Err(e) => self.fail(format!("{e}. {MIC_HINT}")),
            },
            Some(r) => {
                drop(r._stream);
                let audio = resample(&r.samples.lock().unwrap(), r.rate);
                self.status.clear();
                if is_silent(&audio) {
                    self.fail(format!("Kein Ton aufgenommen. {MIC_HINT}"));
                    return;
                }
                let (ctx, tx, ui, lang) = (self.ctx.clone().unwrap(), self.tx.clone(), ui_ctx.clone(), LANGS[self.lang].0);
                self.audio_secs = audio.len() as f32 / 16000.0;
                self.transcribing_since = Some(Instant::now());
                std::thread::spawn(move || {
                    let _ = tx.send(match transcribe(&ctx, &audio, lang) {
                        Ok(t) => Msg::Text(t),
                        Err(e) => Msg::Error(format!("Texterkennung fehlgeschlagen: {e}")),
                    });
                    ui.request_repaint();
                });
            }
        }
    }

    fn copy(&mut self, ui_ctx: &egui::Context) {
        self.error = None;
        ui_ctx.copy_text(self.text.clone());
        self.copied_at = Some(Instant::now());
    }

    fn handle(&mut self, m: Msg) {
        match m {
            Msg::Progress(p) => self.progress = p,
            Msg::Log(s) => self.log(s),
            Msg::Loaded(c, name) => {
                self.ctx = Some(c);
                self.log(format!("Bereit: {name}"));
                self.status = "Bereit".into();
                self.loaded = name;
                self.loading = false;
            }
            Msg::Text(t) => {
                let secs = self.transcribing_since.take().map_or(0.0, |t| t.elapsed().as_secs_f32());
                self.log(format!("Text erkannt: {} s Audio in {} s", decimal(self.audio_secs), decimal(secs)));
                if t.is_empty() {
                    self.status = "Keine Sprache erkannt".into();
                } else {
                    let at = self.cursor.unwrap_or(self.text.chars().count());
                    self.cursor = Some(insert_dictation(&mut self.text, at, &t));
                    self.move_cursor = true;
                    self.last_cleared = None;
                    self.status = format!("Text eingefügt ({} s Audio, {} s)", decimal(self.audio_secs), decimal(secs));
                }
            }
            Msg::Error(e) => {
                self.loading = false;
                self.transcribing_since = None;
                self.status.clear();
                self.fail(e);
            }
        }
    }

    fn model_row(&mut self, ui: &mut egui::Ui, ui_ctx: &egui::Context) {
        let style = ui.style().clone();
        ui.horizontal(|ui| {
            ui.add_enabled_ui(!self.loading && self.rec.is_none() && self.transcribing_since.is_none(), |ui| {
                ui.label("Modell");
                let before = self.model;
                let width = (ui.available_width() - 200.0).clamp(220.0, 400.0);
                egui::ComboBox::from_id_salt("model").width(width).selected_text(model_label(&style, self.model, false)).show_ui(
                    ui,
                    |ui| {
                        for i in 0..MODELS.len() {
                            ui.selectable_value(&mut self.model, i, model_label(&style, i, true));
                        }
                    },
                );
                let (name, _, size) = MODELS[self.model];
                if self.model != before && model_path(name).exists() {
                    self.start_load(ui_ctx);
                }
                if self.loaded == name {
                    ui.label(egui::RichText::new("✓ geladen").color(PRESENT));
                } else if !model_path(name).exists() && ui.button(format!("Herunterladen ({size})")).clicked() {
                    self.start_load(ui_ctx);
                }
            });
        });
        ui.add_space(4.0);
        let name = self.selected();
        egui::Grid::new("model_info").num_columns(2).spacing([10.0, 2.0]).show(ui, |ui| {
            for (label, value) in [("Quelle", model_url(name)), ("Ziel", model_path(name).display().to_string())] {
                ui.label(egui::RichText::new(label).weak());
                ui.add(egui::Label::new(egui::RichText::new(&value).monospace()).truncate()).on_hover_text(&value);
                ui.end_row();
            }
        });
    }

    fn action_row(&mut self, ui: &mut egui::Ui, ui_ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.set_min_height(44.0);
            if self.loading {
                match self.progress {
                    Some((done, total)) if total > 0 => {
                        let bar = egui::ProgressBar::new(done as f32 / total as f32)
                            .text(format!("Download {} / {} MB", done >> 20, total >> 20))
                            .desired_width(300.0);
                        ui.add(bar);
                    }
                    Some((done, _)) => {
                        ui.spinner();
                        ui.label(format!("Download {} MB", done >> 20));
                    }
                    None => {
                        ui.spinner();
                        ui.label("Lade Modell …");
                    }
                }
                if self.progress.is_some() && ui.button("Abbrechen").clicked() {
                    self.cancel.store(true, Ordering::Relaxed);
                }
            } else if let Some(t) = self.transcribing_since {
                ui.spinner();
                ui.label(egui::RichText::new(format!("Text wird erkannt … {} s", t.elapsed().as_secs())).size(15.0));
            } else {
                let (label, fill) = match &self.rec {
                    Some(r) => {
                        let secs = r.samples.lock().unwrap().len() as u32 / r.rate.max(1);
                        (format!("Aufnahme stoppen · {secs} s"), egui::Color32::from_rgb(185, 28, 28))
                    }
                    None => ("Aufnahme starten  (Strg+R)".to_string(), egui::Color32::from_rgb(37, 99, 235)),
                };
                let text = egui::RichText::new(label).size(17.0);
                let can_record = self.can_record();
                let button = if can_record {
                    egui::Button::new(text.color(egui::Color32::WHITE)).fill(fill)
                } else {
                    egui::Button::new(text)
                };
                if ui.add_enabled(can_record, button.min_size(egui::vec2(250.0, 44.0)).corner_radius(8.0)).clicked() {
                    self.toggle(ui_ctx);
                }
                if let Some(r) = &self.rec {
                    let samples = r.samples.lock().unwrap();
                    let recent = &samples[samples.len().saturating_sub(r.rate as usize / 10)..];
                    ui.add(egui::ProgressBar::new(level(recent)).desired_width(120.0)).on_hover_text("Eingangspegel");
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_enabled_ui(!self.loading && self.rec.is_none() && self.transcribing_since.is_none(), |ui| {
                    egui::ComboBox::from_id_salt("lang").selected_text(LANGS[self.lang].1).show_ui(ui, |ui| {
                        for (i, (_, label)) in LANGS.iter().enumerate() {
                            ui.selectable_value(&mut self.lang, i, *label);
                        }
                    });
                    ui.label("Sprache");
                });
            });
        });
        match &self.error {
            Some(e) => ui.add(egui::Label::new(egui::RichText::new(format!("⚠ {e}")).color(ui.visuals().error_fg_color)).wrap()),
            None => {
                let idle = !self.loading && self.rec.is_none() && self.transcribing_since.is_none();
                let status = if idle && self.loaded != self.selected() { "Dieses Modell erst herunterladen" } else { &self.status };
                ui.add(egui::Label::new(status).truncate())
            }
        };
    }

    fn text_area(&mut self, ui: &mut egui::Ui, ui_ctx: &egui::Context) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            ui.horizontal(|ui| {
                if let Some(old) = &self.last_cleared {
                    if ui.button("Rückgängig").clicked() {
                        self.text = old.clone();
                        self.cursor = None;
                        self.last_cleared = None;
                    }
                } else if ui.add_enabled(!self.text.is_empty(), egui::Button::new("Leeren")).clicked() {
                    self.error = None;
                    self.last_cleared = Some(std::mem::take(&mut self.text));
                    self.cursor = None;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let copied = self.copied_at.is_some_and(|t| t.elapsed() < Duration::from_secs(2));
                    let label = if copied { "Kopiert ✓" } else { "In Zwischenablage kopieren  (Strg+Shift+C)" };
                    if ui.add_enabled(!self.text.is_empty(), egui::Button::new(label)).clicked() {
                        self.copy(ui_ctx);
                    }
                });
            });
            let id = egui::Id::unique("dictation");
            if std::mem::take(&mut self.move_cursor)
                && let Some(at) = self.cursor
            {
                // otherwise the text field reports its old cursor and the next dictation lands before this one
                let mut state = egui::TextEdit::load_state(ui_ctx, id).unwrap_or_default();
                let cursor = egui::text::CCursor::new(at);
                state.cursor.set_char_range(Some(egui::text::CCursorRange::one(cursor)));
                state.store(ui_ctx, id);
            }
            egui::ScrollArea::vertical().show(ui, |ui| {
                let out = egui::TextEdit::multiline(&mut self.text)
                    .id(id)
                    .hint_text("Hier erscheint der diktierte Text, an der Stelle des Cursors. Du kannst ihn vor dem Kopieren bearbeiten.")
                    .margin(egui::Margin::same(10))
                    .font(egui::TextStyle::Body)
                    .desired_width(f32::INFINITY)
                    .min_size(ui.available_size())
                    .show(ui);
                if let Some(range) = out.cursor_range {
                    self.cursor = Some(range.primary.index.0);
                }
                if out.response.changed() {
                    self.last_cleared = None;
                }
            });
        });
    }
}

impl eframe::App for App {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ui_ctx = &root.ctx().clone();
        while let Ok(m) = self.rx.try_recv() {
            self.handle(m);
        }

        let record_key = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::R);
        let copy_key = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND | egui::Modifiers::SHIFT, egui::Key::C);
        if ui_ctx.input_mut(|i| i.consume_shortcut(&record_key)) && self.can_record() {
            self.toggle(ui_ctx);
        }
        if ui_ctx.input_mut(|i| i.consume_shortcut(&copy_key)) && !self.text.is_empty() {
            self.copy(ui_ctx);
        }

        let style = root.style().clone();
        let padded = |frame: egui::Frame| frame.inner_margin(egui::Margin::symmetric(14, 10));

        egui::Panel::top("bar").frame(padded(egui::Frame::side_top_panel(&style))).show(root, |ui| self.model_row(ui, ui_ctx));

        egui::Panel::bottom("footer").frame(padded(egui::Frame::side_top_panel(&style))).show(root, |ui| {
            let color = if self.device.starts_with("GPU") { PRESENT } else { ui.visuals().weak_text_color() };
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("●").color(color));
                ui.label(format!("Berechnung läuft auf: {}", self.device));
            });
        });

        egui::Panel::bottom("log").frame(padded(egui::Frame::side_top_panel(&style))).show(root, |ui| {
            let open = std::mem::take(&mut self.open_log).then_some(true);
            egui::CollapsingHeader::new(format!("Protokoll ({})", self.log.len())).id_salt("log").open(open).show(ui, |ui| {
                egui::ScrollArea::vertical().max_height(110.0).stick_to_bottom(true).auto_shrink([false, true]).show(ui, |ui| {
                    for line in &self.log {
                        ui.label(egui::RichText::new(line).monospace());
                    }
                });
            });
        });

        egui::CentralPanel::default().frame(padded(egui::Frame::central_panel(&style))).show(root, |ui| {
            self.action_row(ui, ui_ctx);
            ui.add_space(6.0);
            self.text_area(ui, ui_ctx);
        });

        if self.rec.is_some() != self.recording_title {
            self.recording_title = self.rec.is_some();
            let title = if self.recording_title { "● Aufnahme – Mundwerk" } else { "Mundwerk" };
            ui_ctx.send_viewport_cmd(egui::ViewportCommand::Title(title.into()));
        }
        if self.rec.is_some() || self.transcribing_since.is_some() || self.loading {
            ui_ctx.request_repaint_after(Duration::from_millis(100));
        } else if self.copied_at.is_some_and(|t| t.elapsed() < Duration::from_secs(2)) {
            ui_ctx.request_repaint_after(Duration::from_millis(250));
        }
    }
}

fn main() -> eframe::Result {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([780.0, 600.0])
            .with_min_inner_size([640.0, 480.0])
            .with_title("Mundwerk"),
        ..Default::default()
    };
    eframe::run_native(
        "Mundwerk",
        opts,
        Box::new(|cc| {
            cc.egui_ctx.all_styles_mut(|style| {
                use egui::{FontFamily, FontId, TextStyle};
                style.text_styles.insert(TextStyle::Body, FontId::new(15.0, FontFamily::Proportional));
                style.text_styles.insert(TextStyle::Button, FontId::new(15.0, FontFamily::Proportional));
                style.text_styles.insert(TextStyle::Small, FontId::new(13.0, FontFamily::Proportional));
                style.text_styles.insert(TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace));
                style.spacing.item_spacing = egui::vec2(8.0, 6.0);
                style.spacing.button_padding = egui::vec2(10.0, 5.0);
            });
            Ok(Box::new(App::new(&cc.egui_ctx)))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resample_48k_to_16k() {
        let input: Vec<f32> = (0..48000).map(|i| i as f32).collect();
        let out = resample(&input, 48000);
        assert_eq!(out.len(), 16000);
        assert_eq!(out[1], 3.0);
        assert_eq!(resample(&input[..10], 16000).len(), 10);
    }

    #[test]
    fn silence_is_noise_floor_only() {
        // peaks measured on the WSL microphone: 1/32768 when muted by Windows, 14900/32768 when speaking
        assert!(is_silent(&[1.0 / 32768.0, -1.0 / 32768.0]));
        assert!(!is_silent(&[0.0, 14900.0 / 32768.0]));
        assert!(is_silent(&[]));
    }

    #[test]
    fn level_maps_dbfs_to_unit_range() {
        assert_eq!(level(&[]), 0.0);
        assert_eq!(level(&[0.0; 10]), 0.0);
        assert_eq!(level(&[1.0, -1.0]), 1.0);
        assert!((level(&[0.1, -0.1]) - 2.0 / 3.0).abs() < 1e-4); // -20 dBFS
    }

    #[test]
    fn insert_dictation_spaces_and_cursor() {
        let mut t = String::new();
        assert_eq!(insert_dictation(&mut t, 0, "Hallo"), 5);
        assert_eq!(t, "Hallo");

        let mut t = "Hallo Welt".to_string();
        let end = insert_dictation(&mut t, 10, "wie geht's");
        assert_eq!(t, "Hallo Welt wie geht's");
        assert_eq!(end, t.chars().count());

        let mut t = "Grüße an".to_string();
        assert_eq!(insert_dictation(&mut t, 5, "liebe"), 11);
        assert_eq!(t, "Grüße liebe an");

        let mut t = "ÄÖ".to_string();
        assert_eq!(insert_dictation(&mut t, 1, "x"), 4);
        assert_eq!(t, "Ä x Ö");
        assert_eq!(insert_dictation(&mut t, 99, "y"), 7);
        assert_eq!(t, "Ä x Ö y");
    }
}
