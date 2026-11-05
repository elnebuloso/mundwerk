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
const MIC_HINT: &str = if cfg!(target_os = "macos") {
    "Mikrofon prüfen – Systemeinstellungen > Datenschutz & Sicherheit > Mikrofon, dort Mundwerk einschalten"
} else {
    "Mikrofon prüfen – unter Windows: Einstellungen > Datenschutz und Sicherheit > Mikrofon"
};
// egui's COMMAND modifier is Cmd on macOS and Ctrl elsewhere.
const RECORD_KEY_LABEL: &str = if cfg!(target_os = "macos") { "⌘R" } else { "Strg+R" };
const COPY_KEY_LABEL: &str = if cfg!(target_os = "macos") { "⌘⇧C" } else { "Strg+Shift+C" };
const GLOBAL_KEY_LABEL: &str = if cfg!(target_os = "macos") { "⌃⌥Leertaste" } else { "Strg+Alt+Leertaste" };
const GPU_API: &str = if cfg!(feature = "metal") { "Metal" } else { "Vulkan" };
// below about -40 dBFS throughout: nothing but noise floor, and Whisper would invent text for it
const SILENCE: f32 = 0.01;
const STOP_AFTER_SPEECH_SECS: f32 = 4.0;
const STOP_WITHOUT_SPEECH_SECS: f32 = 15.0;
// +12 dB over the quietest moment of the recording counts as speech, so steady room noise doesn't
const SPEECH_OVER_FLOOR: f32 = 4.0;
// RMS per 100 ms, -54 dBFS: quiet microphones put speech well below the peak-based SILENCE
const SPEECH_MIN_RMS: f32 = 0.002;

enum Msg {
    /// Some((done, total)) while downloading, None while loading into memory.
    Progress(Option<(u64, u64)>),
    Log(String),
    Loaded(Arc<WhisperContext>, String),
    Text(String),
    Error(String),
    /// The global shortcut was pressed, possibly while another window has the focus.
    Hotkey,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cue {
    Start,
    Stop,
    Copied,
}

/// Soft sine notes for a cue: start rises, stop falls, copied is two short equal notes.
fn cue_samples(cue: Cue, rate: u32) -> Vec<f32> {
    // about -26 dBFS: audible next to other sounds, never loud
    const VOLUME: f32 = 0.05;
    let notes: &[(f32, f32)] = match cue {
        Cue::Start => &[(660.0, 0.09), (880.0, 0.12)],
        Cue::Stop => &[(880.0, 0.09), (660.0, 0.12)],
        Cue::Copied => &[(1175.0, 0.06), (1175.0, 0.06)],
    };
    let rate_f = rate as f32;
    let gap = vec![0.0; (rate_f * 0.04) as usize];
    let mut out = Vec::new();
    for (i, &(freq, secs)) in notes.iter().enumerate() {
        if i > 0 {
            out.extend(&gap);
        }
        let n = (rate_f * secs) as usize;
        // 10 ms fade in and out, so the note neither clicks on nor off
        let fade = (rate_f * 0.01) as usize;
        out.extend((0..n).map(|k| {
            let envelope = (k.min(n - 1 - k) as f32 / fade as f32).min(1.0);
            VOLUME * envelope * (std::f32::consts::TAU * freq * k as f32 / rate_f).sin()
        }));
    }
    out
}

fn build_output<T: SizedSample + FromSample<f32>>(
    dev: &cpal::Device,
    cfg: &cpal::StreamConfig,
    samples: Vec<f32>,
) -> Result<cpal::Stream, cpal::Error> {
    let ch = cfg.channels as usize;
    let mut next = samples.into_iter();
    dev.build_output_stream(
        *cfg,
        move |data: &mut [T], _: &_| {
            for frame in data.chunks_mut(ch) {
                let v = T::from_sample(next.next().unwrap_or(0.0));
                frame.fill(v);
            }
        },
        |e| eprintln!("audio output error: {e}"),
        None,
    )
}

/// Plays a cue on the default output device without blocking the UI.
fn play_cue(cue: Cue, tx: Sender<Msg>) {
    std::thread::spawn(move || {
        let result = (|| -> Result<(), String> {
            let dev = cpal::default_host().default_output_device().ok_or("kein Ausgabegerät")?;
            let sup = dev.default_output_config().map_err(|e| e.to_string())?;
            let cfg: cpal::StreamConfig = sup.config();
            let samples = cue_samples(cue, cfg.sample_rate);
            let secs = samples.len() as f32 / cfg.sample_rate as f32;
            let stream = match sup.sample_format() {
                cpal::SampleFormat::F32 => build_output::<f32>(&dev, &cfg, samples),
                cpal::SampleFormat::I16 => build_output::<i16>(&dev, &cfg, samples),
                cpal::SampleFormat::I32 => build_output::<i32>(&dev, &cfg, samples),
                cpal::SampleFormat::U16 => build_output::<u16>(&dev, &cfg, samples),
                f => return Err(format!("Audioformat {f} wird nicht unterstützt")),
            }
            .map_err(|e| e.to_string())?;
            stream.play().map_err(|e| e.to_string())?;
            // the stream stops when dropped; leave room for the device's buffer to drain
            std::thread::sleep(Duration::from_secs_f32(secs + 0.15));
            Ok(())
        })();
        if let Err(e) = result {
            let _ = tx.send(Msg::Log(format!("Ton konnte nicht abgespielt werden: {e}")));
        }
    });
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
    auto_copy: bool,
    /// The running transcription was stopped outside the window, so the copy is announced by a cue.
    cue_on_copy: bool,
    /// Keeps the global shortcut registered; None where the system refused it.
    hotkeys: Option<global_hotkey::GlobalHotKeyManager>,
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
    if !cfg!(any(feature = "vulkan", feature = "metal")) {
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
    format!("CPU – keine {GPU_API}-fähige GPU gefunden")
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
    let bytes: Vec<u8> = state.as_iter().filter_map(|s| s.to_bytes().ok().map(<[u8]>::to_vec)).flatten().collect();
    Ok(decode_text(&bytes))
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

fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt()
}

/// Input level of the samples as 0..1 on a -60..0 dBFS scale.
fn level(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    ((20.0 * rms(samples).max(1e-6).log10() + 60.0) / 60.0).clamp(0.0, 1.0)
}

fn window_levels(samples: &[f32], rate: u32) -> Vec<f32> {
    samples.chunks_exact((rate.max(10) / 10) as usize).map(rms).collect()
}

/// The quietest window that isn't digital zero, and the level above which a window counts as speech.
fn speech_threshold(levels: &[f32]) -> (f32, f32) {
    // below 1e-4 is digital zero (stream start, muted device), not the room's noise floor
    let floor = levels.iter().copied().filter(|&r| r > 1e-4).fold(f32::INFINITY, f32::min);
    let floor = if floor.is_finite() { floor } else { 0.0 };
    (floor, (floor * SPEECH_OVER_FLOOR).max(SPEECH_MIN_RMS))
}

/// For each 100 ms window of the samples, whether it holds speech.
fn speech_windows(samples: &[f32], rate: u32) -> Vec<bool> {
    let levels = window_levels(samples, rate);
    let (_, threshold) = speech_threshold(&levels);
    levels.iter().map(|&r| r > threshold).collect()
}

/// One log line with the levels the speech detection decided on, to tune it on other microphones.
fn level_report(samples: &[f32], rate: u32) -> String {
    let levels = window_levels(samples, rate);
    let (floor, threshold) = speech_threshold(&levels);
    let loudest = levels.iter().copied().fold(0.0, f32::max);
    format!("Pegel (RMS je 100 ms): leisester {floor:.4}, lautester {loudest:.4}, Sprache ab {threshold:.4}")
}

/// Seconds left until the recording should stop for lack of speech, and whether speech was heard.
fn secs_until_auto_stop(samples: &[f32], rate: u32) -> (f32, bool) {
    let rate = rate.max(10);
    // only the last 30 s: older speech would have ended the recording long ago, and the cost stays flat
    let tail = &samples[samples.len().saturating_sub(rate as usize * 30)..];
    let speech = speech_windows(tail, rate);
    match speech.iter().rposition(|&s| s) {
        Some(last) => (STOP_AFTER_SPEECH_SECS - (speech.len() - 1 - last) as f32 / 10.0, true),
        None => (STOP_WITHOUT_SPEECH_SECS - samples.len() as f32 / rate as f32, false),
    }
}

/// The stretch of the recording from the first to the last speech, with a margin, or None without speech.
/// Whisper invents phrases like "Vielen Dank." for silence, so it only gets the spoken part.
fn speech_span(audio: &[f32], rate: u32) -> Option<&[f32]> {
    let rate = rate.max(10) as usize;
    let speech = speech_windows(audio, rate as u32);
    let first = speech.iter().position(|&s| s)?;
    let last = speech.iter().rposition(|&s| s)?;
    let start = (first * rate / 10).saturating_sub(rate * 3 / 10);
    let end = ((last + 1) * rate / 10 + rate / 2).min(audio.len());
    Some(&audio[start..end])
}

/// Whisper's text as bytes, decoded once: a character can be split across segments, and bytes
/// that never form a character are dropped instead of showing up as "�".
fn decode_text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).replace(char::REPLACEMENT_CHARACTER, "").trim().to_string()
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

/// Undo for a cleared text field: the cleared text comes back, anything dictated since stays after it.
fn restore_cleared(cleared: &str, current: &str) -> String {
    let mut text = cleared.to_string();
    if !current.is_empty() {
        insert_dictation(&mut text, cleared.chars().count(), current);
    }
    text
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
        if model_path(name).exists() { ("●", present_color(&style.visuals), "vorhanden") } else { ("○", missing_color(&style.visuals), "noch nicht heruntergeladen") };
    let mut job = LayoutJob::default();
    job.append(dot, 0.0, TextFormat::simple(font.clone(), color));
    let text = if with_state { format!("{purpose} · {name} · {size} – {state}") } else { format!("{purpose} · {name} · {size}") };
    job.append(&text, 6.0, TextFormat::simple(font, style.visuals.text_color()));
    job
}

/// The version CI releases under; "dev" for local builds, since Cargo.toml's version is never bumped.
fn version() -> &'static str {
    match option_env!("MUNDWERK_VERSION") {
        Some(v) if !v.is_empty() => v,
        _ => "dev",
    }
}

/// Green for "present"/GPU, readable on both backgrounds.
fn present_color(visuals: &egui::Visuals) -> egui::Color32 {
    if visuals.dark_mode { egui::Color32::from_rgb(80, 200, 120) } else { egui::Color32::from_rgb(21, 128, 61) }
}

/// Orange for "not downloaded", readable on both backgrounds.
fn missing_color(visuals: &egui::Visuals) -> egui::Color32 {
    if visuals.dark_mode { egui::Color32::from_rgb(230, 150, 40) } else { egui::Color32::from_rgb(180, 83, 9) }
}

fn theme_path() -> PathBuf {
    app_dir().join("theme")
}

/// Dark unless the user switched to light; the system theme is deliberately not followed.
fn load_theme() -> egui::Theme {
    match std::fs::read_to_string(theme_path()) {
        Ok(s) if s.trim() == "light" => egui::Theme::Light,
        _ => egui::Theme::Dark,
    }
}

fn save_theme(theme: egui::Theme) {
    let value = if theme == egui::Theme::Light { "light" } else { "dark" };
    // losing the choice only means dark at the next start
    let _ = std::fs::write(theme_path(), value);
}

fn auto_copy_path() -> PathBuf {
    app_dir().join("auto-copy")
}

fn load_auto_copy() -> bool {
    std::fs::read_to_string(auto_copy_path()).map_or(true, |s| s.trim() != "off")
}

fn save_auto_copy(on: bool) {
    // losing the choice only means copying is on at the next start
    let _ = std::fs::write(auto_copy_path(), if on { "on" } else { "off" });
}

/// Registers the shortcut that toggles recording from any window; the manager must live on the
/// thread running the event loop (Windows, macOS).
fn register_hotkey(tx: Sender<Msg>, ui_ctx: egui::Context) -> Result<global_hotkey::GlobalHotKeyManager, String> {
    use global_hotkey::hotkey::{Code, HotKey, Modifiers};
    use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
    let manager = GlobalHotKeyManager::new().map_err(|e| e.to_string())?;
    manager.register(HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::Space)).map_err(|e| e.to_string())?;
    GlobalHotKeyEvent::set_event_handler(Some(move |e: GlobalHotKeyEvent| {
        if e.state == HotKeyState::Pressed {
            let _ = tx.send(Msg::Hotkey);
            // the window may be in the background, where nothing else wakes it up
            ui_ctx.request_repaint();
        }
    }));
    Ok(manager)
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
            auto_copy: load_auto_copy(),
            cue_on_copy: false,
            hotkeys: None,
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
        app.log(format!("Mundwerk {} gestartet, Daten in {}", version(), dir.display()));
        app.log(format!("Berechnung läuft auf: {}", app.device));
        match register_hotkey(app.tx.clone(), ui_ctx.clone()) {
            Ok(m) => {
                app.hotkeys = Some(m);
                app.log(format!("Tastenkürzel {GLOBAL_KEY_LABEL} startet und stoppt die Aufnahme aus jedem Fenster"));
            }
            Err(e) => app.log(format!("Tastenkürzel {GLOBAL_KEY_LABEL} nicht verfügbar: {e}")),
        }
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

    fn auto_stop(&mut self, ui_ctx: &egui::Context) {
        let Some(r) = &self.rec else { return };
        let (left, heard) = secs_until_auto_stop(&r.samples.lock().unwrap(), r.rate);
        if left > 0.0 {
            self.status = match left {
                l if l <= 3.0 => format!("Aufnahme läuft … Stille, stoppt in {} s", l.ceil()),
                _ => "Aufnahme läuft …".into(),
            };
            return;
        }
        self.log(if heard {
            format!("Aufnahme nach {STOP_AFTER_SPEECH_SECS} s Stille automatisch beendet")
        } else {
            format!("Aufnahme nach {STOP_WITHOUT_SPEECH_SECS} s ohne erkannte Sprache automatisch beendet")
        });
        self.toggle(ui_ctx, true);
    }

    /// `audible`: started or stopped outside the window (global shortcut, silence), where only a cue tells the user.
    fn toggle(&mut self, ui_ctx: &egui::Context, audible: bool) {
        self.error = None;
        match self.rec.take() {
            None => match start_recording() {
                Ok(r) => {
                    if audible {
                        play_cue(Cue::Start, self.tx.clone());
                        // dictating from another window: the last text was most likely pasted there already
                        if !self.text.is_empty() {
                            self.last_cleared = Some(std::mem::take(&mut self.text));
                            self.cursor = None;
                        }
                    }
                    self.log(format!("Aufnahme über Mikrofon: {}", r.device));
                    self.rec = Some(r);
                    self.status = "Aufnahme läuft …".into();
                }
                Err(e) => self.fail(format!("{e}. {MIC_HINT}")),
            },
            Some(r) => {
                drop(r._stream);
                if audible {
                    play_cue(Cue::Stop, self.tx.clone());
                }
                self.cue_on_copy = audible;
                let audio = resample(&r.samples.lock().unwrap(), r.rate);
                self.status.clear();
                if is_silent(&audio) {
                    self.fail(format!("Kein Ton aufgenommen. {MIC_HINT}"));
                    return;
                }
                self.log(level_report(&audio, 16000));
                // Without detected speech the whole recording still goes to Whisper: a microphone the
                // detection misjudges must not cost a dictation, an invented "Vielen Dank." is cheaper.
                let audio = match speech_span(&audio, 16000) {
                    Some(speech) => speech.to_vec(),
                    None => {
                        self.log("Keine Sprache erkannt, die ganze Aufnahme geht an die Texterkennung".into());
                        audio
                    }
                };
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

    fn handle(&mut self, m: Msg, ui_ctx: &egui::Context) {
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
                    self.status = format!("Text eingefügt ({} s Audio, {} s)", decimal(self.audio_secs), decimal(secs));
                    if self.auto_copy {
                        self.copy(ui_ctx);
                        self.status.push_str(", kopiert");
                        if self.cue_on_copy {
                            play_cue(Cue::Copied, self.tx.clone());
                        }
                    }
                }
            }
            Msg::Error(e) => {
                self.loading = false;
                self.transcribing_since = None;
                self.status.clear();
                self.fail(e);
            }
            Msg::Hotkey => {
                if self.can_record() {
                    self.toggle(ui_ctx, true);
                }
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
                    ui.label(egui::RichText::new("✓ geladen").color(present_color(ui.visuals())));
                } else if !model_path(name).exists() && ui.button(format!("Herunterladen ({size})")).clicked() {
                    self.start_load(ui_ctx);
                }
            });
        });
        ui.add_space(8.0);
        let name = self.selected();
        egui::Grid::new("model_info").num_columns(2).spacing([12.0, 4.0]).min_row_height(18.0).show(ui, |ui| {
            for (label, value) in [("Quelle", model_url(name)), ("Ziel", model_path(name).display().to_string())] {
                ui.label(egui::RichText::new(label).weak());
                ui.add(egui::Label::new(egui::RichText::new(&value).monospace()).truncate()).on_hover_text(&value);
                ui.end_row();
            }
        });
    }

    fn action_row(&mut self, ui: &mut egui::Ui, ui_ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.set_min_height(48.0);
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
                    None => {
                        let key = if self.hotkeys.is_some() { GLOBAL_KEY_LABEL } else { RECORD_KEY_LABEL };
                        (format!("Aufnahme starten  ({key})"), egui::Color32::from_rgb(37, 99, 235))
                    }
                };
                let text = egui::RichText::new(label).size(17.0);
                let can_record = self.can_record();
                let button = if can_record {
                    egui::Button::new(text.color(egui::Color32::WHITE)).fill(fill)
                } else {
                    egui::Button::new(text)
                };
                if ui.add_enabled(can_record, button.min_size(egui::vec2(280.0, 48.0)).corner_radius(8.0)).clicked() {
                    self.toggle(ui_ctx, false);
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
        ui.add_space(4.0);
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
                        self.text = restore_cleared(old, &self.text);
                        self.cursor = None;
                        self.last_cleared = None;
                    }
                } else if ui.add_enabled(!self.text.is_empty(), egui::Button::new("Leeren")).clicked() {
                    self.error = None;
                    self.last_cleared = Some(std::mem::take(&mut self.text));
                    self.cursor = None;
                }
                if ui.checkbox(&mut self.auto_copy, "Automatisch kopieren").on_hover_text("Legt nach jedem Diktat den ganzen Text in die Zwischenablage").changed() {
                    save_auto_copy(self.auto_copy);
                }
                ui.add_space(8.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let copied = self.copied_at.is_some_and(|t| t.elapsed() < Duration::from_secs(2));
                    let label = if copied { "Kopiert ✓".to_string() } else { format!("In Zwischenablage kopieren  ({COPY_KEY_LABEL})") };
                    if ui.add_enabled(!self.text.is_empty(), egui::Button::new(label)).clicked() {
                        self.copy(ui_ctx);
                    }
                });
            });
            ui.add_space(10.0);
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
                // TextEdit's line height is the font's row height plus this; the default 0 sets dictated paragraphs tightly
                ui.spacing_mut().extra_text_line_spacing = 8.0;
                let out = egui::TextEdit::multiline(&mut self.text)
                    .id(id)
                    .hint_text("Hier erscheint der diktierte Text, an der Stelle des Cursors. Du kannst ihn vor dem Kopieren bearbeiten.")
                    .margin(egui::Margin::symmetric(16, 14))
                    .font(egui::FontId::proportional(16.0))
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
            self.handle(m, ui_ctx);
        }

        let record_key = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::R);
        let copy_key = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND | egui::Modifiers::SHIFT, egui::Key::C);
        if ui_ctx.input_mut(|i| i.consume_shortcut(&record_key)) && self.can_record() {
            self.toggle(ui_ctx, false);
        }
        if ui_ctx.input_mut(|i| i.consume_shortcut(&copy_key)) && !self.text.is_empty() {
            self.copy(ui_ctx);
        }

        let style = root.style().clone();
        let padded = |frame: egui::Frame| frame.inner_margin(egui::Margin::symmetric(20, 14));

        egui::Panel::top("bar").frame(padded(egui::Frame::side_top_panel(&style))).show(root, |ui| self.model_row(ui, ui_ctx));

        egui::Panel::bottom("footer").frame(padded(egui::Frame::side_top_panel(&style))).show(root, |ui| {
            let color = if self.device.starts_with("GPU") { present_color(ui.visuals()) } else { ui.visuals().weak_text_color() };
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mut theme = ui_ctx.theme();
                    // right-to-left layout: the first one added sits rightmost
                    let dark = ui.selectable_value(&mut theme, egui::Theme::Dark, "🌙").on_hover_text("Dunkel");
                    let light = ui.selectable_value(&mut theme, egui::Theme::Light, "☀").on_hover_text("Hell");
                    if light.changed() || dark.changed() {
                        ui_ctx.set_theme(theme);
                        save_theme(theme);
                    }
                    ui.label(egui::RichText::new(format!("Version {}", version())).weak());
                    ui.add_space(12.0);
                    // laid out last so it gets the space the right side leaves and truncates instead of overlapping
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("●").color(color));
                        let device = format!("Berechnung läuft auf: {}", self.device);
                        ui.add(egui::Label::new(&device).truncate()).on_hover_text(&device);
                    });
                });
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

        self.auto_stop(ui_ctx);
        egui::CentralPanel::default().frame(padded(egui::Frame::central_panel(&style))).show(root, |ui| {
            self.action_row(ui, ui_ctx);
            ui.add_space(12.0);
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
            .with_inner_size([880.0, 680.0])
            .with_min_inner_size([720.0, 600.0])
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
                style.spacing.item_spacing = egui::vec2(10.0, 8.0);
                style.spacing.button_padding = egui::vec2(14.0, 7.0);
                style.spacing.interact_size.y = 30.0;
            });
            cc.egui_ctx.set_theme(load_theme());
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

    /// A square wave at 1 kHz with one amplitude per 100 ms window.
    fn audio(windows: &[f32]) -> Vec<f32> {
        windows.iter().flat_map(|&a| [a, -a].repeat(50)).collect()
    }

    #[test]
    fn auto_stop_counts_from_last_speech() {
        let noise = 0.003;
        let mut a = audio(&[noise; 5]);
        a.extend(audio(&[0.2; 10]));
        a.extend(audio(&[noise; 10])); // 1 s pause
        let (left, heard) = secs_until_auto_stop(&a, 1000);
        assert!(heard);
        assert!((left - 3.0).abs() < 1e-4, "{left}");
        a.extend(audio(&[noise; 30]));
        assert!(secs_until_auto_stop(&a, 1000).0 <= 0.0);
    }

    #[test]
    fn auto_stop_ignores_steady_room_noise() {
        // room noise above the absolute silence threshold still is no speech
        let a = audio(&[0.03; 160]);
        let (left, heard) = secs_until_auto_stop(&a, 1000);
        assert!(!heard);
        assert!(left < 0.0);
    }

    #[test]
    fn auto_stop_without_speech_waits_longer() {
        let a = audio(&[0.0; 50]); // digital zero, e.g. a muted device
        assert_eq!(secs_until_auto_stop(&a, 1000), (STOP_WITHOUT_SPEECH_SECS - 5.0, false));
    }

    #[test]
    fn auto_stop_hears_speech_from_the_first_moment() {
        let mut a = audio(&[0.2, 0.25, 0.15]);
        a.extend(audio(&[0.002; 2]));
        let (left, heard) = secs_until_auto_stop(&a, 1000);
        assert!(heard);
        assert!((left - 3.8).abs() < 1e-4, "{left}");
    }

    /// Dominant frequency of a stretch of a sine, from its zero crossings.
    fn pitch(samples: &[f32], rate: u32) -> f32 {
        let crossings = samples.windows(2).filter(|w| w[0] < 0.0 && w[1] >= 0.0).count();
        crossings as f32 * rate as f32 / samples.len() as f32
    }

    #[test]
    fn cues_are_quiet_and_click_free() {
        for cue in [Cue::Start, Cue::Stop, Cue::Copied] {
            let s = cue_samples(cue, 48000);
            assert!(s.iter().all(|v| v.abs() <= 0.05), "{cue:?} too loud");
            assert!(s[0].abs() < 1e-3 && s[s.len() - 1].abs() < 1e-2, "{cue:?} clicks");
            assert!(s.len() < 48000 / 2, "{cue:?} too long");
        }
    }

    #[test]
    fn start_cue_rises_and_stop_cue_falls() {
        let rate = 48000;
        let note = (rate as f32 * 0.08) as usize;
        let start = cue_samples(Cue::Start, rate);
        let stop = cue_samples(Cue::Stop, rate);
        assert!(pitch(&start[..note], rate) < pitch(&start[start.len() - note..], rate));
        assert!(pitch(&stop[..note], rate) > pitch(&stop[stop.len() - note..], rate));
    }

    #[test]
    fn restore_cleared_keeps_new_dictation_after_old_text() {
        assert_eq!(restore_cleared("Alter Text.", ""), "Alter Text.");
        assert_eq!(restore_cleared("Alter Text.", "Neues Diktat."), "Alter Text. Neues Diktat.");
        assert_eq!(restore_cleared("", "Neu."), "Neu.");
    }

    #[test]
    fn speech_span_trims_silence_with_margin() {
        // at 1 kHz: 1 s noise, 0.5 s speech, 2 s noise
        let mut a = audio(&[0.003; 10]);
        a.extend(audio(&[0.2; 5]));
        a.extend(audio(&[0.003; 20]));
        let span = speech_span(&a, 1000).unwrap();
        assert_eq!(span.len(), 300 + 500 + 500);
        assert_eq!(span.as_ptr(), a[700..].as_ptr());
    }

    #[test]
    fn quiet_microphone_speech_counts() {
        // speech at RMS 0.006 (-44 dBFS) over a noise floor of 0.0005: below the old 0.01 limit
        let mut a = audio(&[0.0005; 10]);
        a.extend(audio(&[0.006; 10]));
        a.extend(audio(&[0.0005; 10]));
        assert_eq!(speech_span(&a, 1000).unwrap().len(), 300 + 1000 + 500);
        assert!(secs_until_auto_stop(&a, 1000).1);
    }

    #[test]
    fn speech_span_is_none_without_speech() {
        assert!(speech_span(&audio(&[0.03; 50]), 1000).is_none());
        assert!(speech_span(&[], 16000).is_none());
    }

    #[test]
    fn speech_span_keeps_margins_inside_the_recording() {
        let mut a = audio(&[0.2; 2]);
        a.extend(audio(&[0.003; 2]));
        assert_eq!(speech_span(&a, 1000).unwrap().len(), a.len());
    }

    #[test]
    fn decode_text_joins_split_characters_and_drops_broken_ones() {
        let mut bytes = b" Gr".to_vec();
        bytes.extend_from_slice(&"ö".as_bytes()[..1]);
        bytes.extend_from_slice(&"ö".as_bytes()[1..]);
        assert_eq!(decode_text(&bytes), "Grö");
        assert_eq!(decode_text(b"B\xF0\x9F. Dispatch. "), "B. Dispatch.");
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
