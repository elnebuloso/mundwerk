#![windows_subsystem = "windows"] // no console window on Windows

use std::ffi::CStr;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample};
use eframe::egui;
use sha2::{Digest, Sha256};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

const MODELS: &[(&str, &str)] = &[
    ("large-v3-turbo-q5_0", "beste Wahl, 0,5 GB"),
    ("large-v3-turbo", "1,6 GB"),
    ("small", "schnell, 0,5 GB"),
    ("base", "sehr schnell, 0,15 GB"),
];
const LANGS: &[(&str, &str)] = &[("de", "Deutsch"), ("en", "Englisch"), ("auto", "Automatisch")];
const PRESENT: egui::Color32 = egui::Color32::from_rgb(80, 200, 120);
const MISSING: egui::Color32 = egui::Color32::from_rgb(230, 150, 40);

enum Msg {
    Status(String),
    Log(String),
    Loaded(Arc<WhisperContext>, String),
    Text(String),
    Error(String),
}

struct Recording {
    _stream: cpal::Stream,
    samples: Arc<Mutex<Vec<f32>>>,
    rate: u32,
}

struct App {
    model: usize,
    lang: usize,
    ctx: Option<Arc<WhisperContext>>,
    loaded: String,
    busy: bool,
    transcribing_since: Option<Instant>,
    audio_secs: f32,
    rec: Option<Recording>,
    text: String,
    status: String,
    started: Instant,
    log: Vec<String>,
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
fn load_model(name: String, tx: Sender<Msg>, ui: egui::Context) {
    let send = |m| {
        let _ = tx.send(m);
        ui.request_repaint();
    };
    let result = (|| -> Result<Arc<WhisperContext>, String> {
        let path = model_path(&name);
        if path.exists() {
            send(Msg::Log(format!("Vorhanden: {}", path.display())));
        } else {
            download(&name, &path, &send)?;
        }
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

fn download(name: &str, path: &Path, send: &impl Fn(Msg)) -> Result<(), String> {
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
        let n = body.read(&mut buf).map_err(|e| format!("Download abgebrochen: {e}"))?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n]).map_err(|e| e.to_string())?;
        hasher.update(&buf[..n]);
        done += n as u64;
        if done - last > 5 << 20 {
            last = done;
            send(Msg::Status(match total {
                0 => format!("Lade {name} herunter: {} MB", done >> 20),
                t => format!("Lade {name} herunter: {} %", done * 100 / t),
            }));
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
    Ok(Recording { _stream: stream, samples, rate: cfg.sample_rate })
}

impl App {
    fn new() -> Self {
        let (tx, rx) = channel();
        let mut app = App {
            model: 0,
            lang: 0,
            ctx: None,
            loaded: String::new(),
            busy: false,
            transcribing_since: None,
            audio_secs: 0.0,
            rec: None,
            text: String::new(),
            status: String::new(),
            started: Instant::now(),
            log: Vec::new(),
            device: compute_device(),
            tx,
            rx,
        };
        let dir = app_dir();
        if !dir.exists() {
            match std::fs::create_dir_all(&dir) {
                Ok(()) => app.log(format!("Ordner angelegt: {}", dir.display())),
                Err(e) => app.log(format!("Ordner {} konnte nicht angelegt werden: {e}", dir.display())),
            }
        }
        app.log(format!("Mundwerk {} gestartet, Daten in {}", env!("CARGO_PKG_VERSION"), dir.display()));
        app.log(format!("Rechenwerk: {}", app.device));
        app.status = "Modell wählen und laden".into();
        app
    }

    /// Shows a line in the window's log, appends it to `~/.mundwerk/mundwerk.log` and mirrors it in the status.
    // shortcut: seconds since start instead of wall-clock time, saves a time-zone crate; add one if the log file needs dates
    fn log(&mut self, line: String) {
        let entry = format!("{:>7.1} s  {line}", self.started.elapsed().as_secs_f32());
        let file = std::fs::OpenOptions::new().create(true).append(true).open(app_dir().join("mundwerk.log"));
        // a log file that can't be written must not stop dictation
        if let Ok(mut f) = file {
            let _ = writeln!(f, "{entry}");
        }
        self.log.push(entry);
        self.status = line;
    }

    fn toggle(&mut self, ui_ctx: &egui::Context) {
        match self.rec.take() {
            None => match start_recording() {
                Ok(r) => {
                    self.rec = Some(r);
                    self.status = "Aufnahme läuft ...".into();
                }
                Err(e) => self.log(format!("Fehler: {e}")),
            },
            Some(r) => {
                drop(r._stream);
                let audio = resample(&r.samples.lock().unwrap(), r.rate);
                let (ctx, tx, ui, lang) = (self.ctx.clone().unwrap(), self.tx.clone(), ui_ctx.clone(), LANGS[self.lang].0);
                self.busy = true;
                self.audio_secs = audio.len() as f32 / 16000.0;
                self.transcribing_since = Some(Instant::now());
                std::thread::spawn(move || {
                    let _ = tx.send(match transcribe(&ctx, &audio, lang) {
                        Ok(t) => Msg::Text(t),
                        Err(e) => Msg::Error(format!("Transkription fehlgeschlagen: {e}")),
                    });
                    ui.request_repaint();
                });
            }
        }
    }
}

fn model_label(name: &str, info: &str) -> egui::RichText {
    if model_path(name).exists() {
        egui::RichText::new(format!("● {name} ({info}) – vorhanden")).color(PRESENT)
    } else {
        egui::RichText::new(format!("○ {name} ({info}) – nicht heruntergeladen")).color(MISSING)
    }
}

impl eframe::App for App {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ui_ctx = &root.ctx().clone();
        while let Ok(m) = self.rx.try_recv() {
            match m {
                Msg::Status(s) => self.status = s,
                Msg::Log(s) => self.log(s),
                Msg::Loaded(c, name) => {
                    self.ctx = Some(c);
                    self.log(format!("Bereit: {name}"));
                    self.loaded = name;
                    self.busy = false;
                }
                Msg::Text(t) => {
                    self.busy = false;
                    let secs = self.transcribing_since.take().map_or(0.0, |t| t.elapsed().as_secs_f32());
                    self.log(format!("Transkribiert: {:.1} s Audio in {secs:.1} s", self.audio_secs));
                    if t.is_empty() {
                        self.status = "Keine Sprache erkannt".into();
                    } else {
                        if !self.text.is_empty() && !self.text.ends_with(char::is_whitespace) {
                            self.text.push(' ');
                        }
                        self.text.push_str(&t);
                        self.status.clear();
                    }
                }
                Msg::Error(e) => {
                    self.busy = false;
                    self.transcribing_since = None;
                    self.log(e);
                }
            }
        }

        if let Some(t) = self.transcribing_since {
            self.status = format!("Transkribiere ... {} s", t.elapsed().as_secs());
        }
        let (name, info) = MODELS[self.model];
        let present = model_path(name).exists();
        let style = root.style().clone();
        let padded = |frame: egui::Frame| frame.inner_margin(egui::Margin::symmetric(14, 10));

        egui::Panel::top("bar").frame(padded(egui::Frame::side_top_panel(&style))).show(root, |ui| {
            ui.add_enabled_ui(!self.busy && self.rec.is_none(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Modell");
                    egui::ComboBox::from_id_salt("model").width(340.0).selected_text(model_label(name, info)).show_ui(ui, |ui| {
                        for (i, (name, info)) in MODELS.iter().enumerate() {
                            ui.selectable_value(&mut self.model, i, model_label(name, info));
                        }
                    });
                    if name == self.loaded {
                        ui.label(egui::RichText::new("✓ geladen").color(PRESENT));
                    } else if ui.button(if present { "Modell laden" } else { "Herunterladen und laden" }).clicked() {
                        self.busy = true;
                        let (tx, ui) = (self.tx.clone(), ui_ctx.clone());
                        std::thread::spawn(move || load_model(name.into(), tx, ui));
                    }
                });
            });
            ui.add_space(4.0);
            egui::Grid::new("model_info").num_columns(2).spacing([10.0, 2.0]).show(ui, |ui| {
                ui.label(egui::RichText::new("Quelle").small().weak());
                ui.label(egui::RichText::new(model_url(name)).small().monospace());
                ui.end_row();
                ui.label(egui::RichText::new("Ziel").small().weak());
                ui.label(egui::RichText::new(model_path(name).display().to_string()).small().monospace());
                ui.end_row();
            });
        });

        egui::Panel::bottom("footer").frame(padded(egui::Frame::side_top_panel(&style))).show(root, |ui| {
            let color = if self.device.starts_with("GPU") { PRESENT } else { MISSING };
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("●").color(color));
                ui.label(egui::RichText::new(format!("Rechenwerk: {}", self.device)).small());
            });
        });

        egui::Panel::bottom("log").resizable(true).default_size(120.0).frame(padded(egui::Frame::side_top_panel(&style))).show(root, |ui| {
            ui.label(egui::RichText::new("Protokoll").small().weak());
            egui::ScrollArea::vertical().stick_to_bottom(true).auto_shrink(false).show(ui, |ui| {
                for line in &self.log {
                    ui.label(egui::RichText::new(line).small().monospace());
                }
            });
        });

        egui::CentralPanel::default().frame(padded(egui::Frame::central_panel(&style))).show(root, |ui| {
            ui.horizontal(|ui| {
                let (label, fill) = match &self.rec {
                    Some(r) => {
                        let secs = r.samples.lock().unwrap().len() as u32 / r.rate.max(1);
                        (format!("■  Aufnahme stoppen · {secs} s"), egui::Color32::from_rgb(185, 28, 28))
                    }
                    None => ("●  Aufnahme starten".to_string(), egui::Color32::from_rgb(37, 99, 235)),
                };
                let can_record = self.ctx.is_some() && !self.busy;
                let text = egui::RichText::new(label).size(18.0);
                let rec_btn = if can_record {
                    egui::Button::new(text.color(egui::Color32::WHITE)).fill(fill)
                } else {
                    egui::Button::new(text)
                };
                if ui.add_enabled(can_record, rec_btn.min_size(egui::vec2(240.0, 44.0)).corner_radius(8.0)).clicked() {
                    self.toggle(ui_ctx);
                }
                ui.add_space(8.0);
                let hint = if self.ctx.is_none() && !self.busy { "Erst ein Modell laden" } else { self.status.as_str() };
                ui.label(egui::RichText::new(hint).size(15.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_enabled_ui(!self.busy && self.rec.is_none(), |ui| {
                        egui::ComboBox::from_id_salt("lang").selected_text(LANGS[self.lang].1).show_ui(ui, |ui| {
                            for (i, (_, label)) in LANGS.iter().enumerate() {
                                ui.selectable_value(&mut self.lang, i, *label);
                            }
                        });
                        ui.label("Sprache");
                    });
                });
            });
            ui.add_space(8.0);
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
                ui.horizontal(|ui| {
                    if ui.add_enabled(!self.text.is_empty(), egui::Button::new("Leeren")).clicked() {
                        self.text.clear();
                    }
                    if ui.add_enabled(!self.text.is_empty(), egui::Button::new("In Zwischenablage kopieren")).clicked() {
                        ui_ctx.copy_text(self.text.clone());
                        self.status = "Kopiert ✓".into();
                    }
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut self.text)
                            .hint_text("Hier erscheint der diktierte Text. Du kannst ihn vor dem Kopieren bearbeiten.")
                            .margin(egui::Margin::same(10))
                            .font(egui::TextStyle::Body),
                    );
                });
            });
        });
        if self.rec.is_some() || self.transcribing_since.is_some() {
            ui_ctx.request_repaint_after(std::time::Duration::from_millis(200));
        }
    }
}

fn main() -> eframe::Result {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([760.0, 600.0])
            .with_min_inner_size([560.0, 420.0])
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
                style.text_styles.insert(TextStyle::Small, FontId::new(12.0, FontFamily::Proportional));
                style.text_styles.insert(TextStyle::Monospace, FontId::new(12.5, FontFamily::Monospace));
                style.spacing.item_spacing = egui::vec2(8.0, 6.0);
                style.spacing.button_padding = egui::vec2(10.0, 5.0);
            });
            Ok(Box::new(App::new()))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::resample;

    #[test]
    fn resample_48k_to_16k() {
        let input: Vec<f32> = (0..48000).map(|i| i as f32).collect();
        let out = resample(&input, 48000);
        assert_eq!(out.len(), 16000);
        assert_eq!(out[1], 3.0);
        assert_eq!(resample(&input[..10], 16000).len(), 10);
    }
}
