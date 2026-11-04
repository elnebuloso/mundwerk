# Übergabe: Mundwerk

## Was Mundwerk ist

Lokale Diktier-App: Fenster öffnen, Modell wählen, „● Aufnahme starten“, sprechen, stoppen. Der Text landet in einem editierbaren Textfeld, „In Zwischenablage kopieren“ kopiert ihn. Spracherkennung läuft lokal mit whisper.cpp, es gibt kein Konto und keine Cloud.

**Ziele des Nutzers:**
- Kein Python auf dem Zielrechner. Ausgeliefert wird eine einzelne `.exe` für Windows und ein Binary für Linux, beide über GitHub Releases.
- Unter Linux und WSL (über WSLg) startbar.
- Die GPU wird automatisch genutzt, wenn eine da ist. Der Nutzer hat eine GTX 1070.
- „Keep it simple and stupid“.

Entstanden ist das als Ersatz für OpenWhispr (verlangt ein E-Mail-Konto) und OpenWhisper.org (nur OpenAI-API). Der Vorläufer war ein Python-Skript des Nutzers (faster-whisper, Hotkey, Auto-Paste), das nicht ins Repo gehört.

## Stand

- **Code:** alles in `src/main.rs`. Die Teile:
  - eframe/egui 0.37 für das Fenster
  - cpal 0.18 für das Mikrofon
  - whisper-rs 0.16 als Anbindung an whisper.cpp
  - ureq für den Modell-Download von `huggingface.co/ggerganov/whisper.cpp`
  - dirs für das Modellverzeichnis
- **GPU:** Das Cargo-Feature `vulkan` ist Default. Ob eine GPU genutzt wird, entscheidet whisper.cpp beim Start selbst und fällt sonst auf die CPU zurück. Ohne GPU-Support baut `--no-default-features`.
- **Release:** `.github/workflows/release.yml` baut bei einem Tag `v*` auf `ubuntu-22.04` und `windows-latest`. Die Artefakte `mundwerk-linux-x86_64` und `mundwerk-windows-x86_64.exe` hängt `softprops/action-gh-release` ans Release.
- **`Makefile`:** Targets `help`, `build`, `run`, `test`.
- **Die Dateien sind ungetrackt und nichts ist committet.** Der Umzug kam aus `/tmp/foo/mundwerk`. Das Repo steht noch auf dem Branch `chore/charter-rules`, der mit dieser Arbeit nichts zu tun hat.

**Verifiziert in WSL (Ubuntu 24.04, i5-3570K, GTX 1070):**
- Release-Build mit Vulkan läuft durch.
- `cargo test` läuft durch, das ist ein Unit-Test für `resample`.
- Das Fenster öffnet sich über WSLg.
- Das Modell `large-v3-turbo-q5_0` wird heruntergeladen und geladen.
- `ggml_vulkan: No devices found` → sauberer CPU-Fallback.

**Gemessen** mit einem inzwischen gelöschten Bench-Example, 5 s Stille, `large-v3-turbo-q5_0`, CPU mit 4 Threads:

| Einstellungen | Dauer |
|---|---|
| Greedy, volles 30-s-Fenster | 126 s |
| Greedy, `set_audio_ctx` auf Aufnahmelänge | 17 s (mit und ohne Vulkan-Build gleich) |

Daraufhin sind in der App Greedy und `set_audio_ctx` eingebaut. Beam-Search 5, die vorherige App-Einstellung, wurde nie gemessen. In der Praxis lief es damit nach über 2 Minuten noch.

## Nächste Schritte

1. **Ende-zu-Ende mit echter Sprache bestätigen lassen.** Der Nutzer hat noch nie einen erfolgreich transkribierten Text gesehen. Beim ersten Versuch hing es scheinbar bei „Transkribiere …“, das war aber nur die langsame CPU. Danach kamen erst Greedy, `audio_ctx` und die Sekundenanzeige dazu.
2. **Vor dem ersten Commit den Branch klären** (siehe offene Entscheidungen), dann in Conventional Commits auf Englisch committen. Pushen nur, wenn der Nutzer es ausdrücklich sagt.
3. **Workflow einmal laufen lassen** (`workflow_dispatch` existiert), sobald gepusht ist. Er ist noch nie gelaufen, die Risiken stehen unter den Stolperfallen.
4. **Die Windows-`.exe` auf dem Rechner des Nutzers testen.** Das ist der eigentliche Zielpfad mit GTX 1070 über Vulkan. Völlig ungetestet.
5. **Offene Punkte als Backlog-Tasks anlegen.** Das Repo nutzt Backlog.md, siehe `CLAUDE.md`: zuerst `backlog instructions overview`, Tasks auf Deutsch, immer mit `--type` und `--priority`. Bisher gibt es keine Tasks.

## Offene Entscheidungen

- **Branch:** Die Arbeit liegt ungetrackt auf `chore/charter-rules`. Eigener Feature-Branch von `main`, oder erst `chore/charter-rules` mergen? Das entscheidet der Nutzer.
- **Release-Mechanik:** `.claude/rules/agent-plugin-charter/releases.md` setzt voraus, dass ein Release-Tooling die Version aus Conventional Commits ableitet. Ein solches Tooling gibt es noch nicht, der Workflow reagiert nur auf manuell gesetzte Tags `v*`. Release-please oder Ähnliches einführen, oder bei Tags bleiben? `Cargo.toml` steht auf `0.1.0`. Unter 1.0 gilt `v0-commits.md`: keine Breaking-Change-Marker.
- **GPU-Anzeige in der UI:** Der Nutzer wollte, dass die App die GPU „ausliest“. Die Auswahl passiert automatisch, sichtbar ist sie aber nur auf stderr, nicht im Fenster. Ob GPU oder CPU, steht nur in den whisper.cpp-Logzeilen (`whisper_backend_init_gpu`). Eine Anzeige bräuchte Log-Hooks (`whisper-rs` Feature `log_backend`) oder ggml-Device-Abfragen.
- **GPU unter WSL:** Vulkan sieht dort keine NVIDIA, es gibt nur `lvp` und CUDA (`/usr/lib/wsl/lib/libcuda.so`). GPU-Tempo in WSL ginge nur mit einem CUDA-Build. Der braucht das CUDA-Toolkit im Build und CUDA-Libs beim Nutzer, also ein zweites Artefakt. Bisher bewusst nicht gemacht: „unter WSL CPU, für GPU die Windows-`.exe`“.
- **Sprache der UI:** Die UI-Texte sind deutsch, Code und Kommentare englisch (Regel `base.md`). Für ein Open-Source-Release eventuell Englisch oder umschaltbar machen. Bisher nicht besprochen.

## Stolperfallen

- **Rust-Toolchain:** rustup hat keine Default-Toolchain, nur `1.99.0` ist installiert. `make build` und `cargo` scheitern mit „rustup could not choose a version“. Lokal mit `RUSTUP_TOOLCHAIN=1.99.0 make build` arbeiten, oder den Nutzer fragen, ob `rustup default 1.99.0` gesetzt werden darf.
- **Dauer des ersten Builds:** In einem frischen `target/` dauert der Build etwa 10 Minuten (whisper.cpp und Vulkan-Shader). Im Repo lief beim Schreiben dieser Übergabe der erste `make test`-Build noch. Ob er grün durchlief, ist nicht bestätigt, also einmal `RUSTUP_TOOLCHAIN=1.99.0 make test` ausführen. Mit gleichem Quellcode lief er in `/tmp/foo/mundwerk` grün.
- **CPU-Flags im Release:** Der Workflow setzt `GGML_NATIVE=OFF`, `GGML_AVX2=OFF` und `GGML_FMA=OFF`. Der Rechner des Nutzers (i5-3570K, Ivy Bridge) hat **kein AVX2**, ein AVX2-Build stürzt dort mit „illegal instruction“ ab. Diese Flags nicht „optimieren“. Die `GGML_*`-Env-Variablen reicht `whisper-rs-sys` an CMake durch, das ist im Quellcode `build.rs` geprüft.
- **Ungetestete Annahmen im Workflow:**
  - Inputs von `jakoch/install-vulkan-sdk-action@v1`
  - `RUSTFLAGS=-C target-feature=+crt-static` zusammen mit dem CMake-Build von whisper.cpp unter MSVC. Mögliche Folge: CRT-Mismatch beim Linken. Notfalls das Flag entfernen, dann braucht der Nutzer die VC++-Runtime.
  - libclang für bindgen auf `windows-latest`
- **Linux-Binary:** Es ist gegen `libvulkan.so.1` gelinkt und startet ohne `libvulkan1` gar nicht. Das steht in der README. Unter WSL ist die Bibliothek da.
- **WSL-Mikrofon:** cpal nutzt ALSA, und in WSL ist ALSA nicht auf den PulseAudio-Server von WSLg umgeleitet. Fehlerbild: „The requested audio device is not available“. Lösung: `libasound2-plugins` installieren und `~/.asoundrc` anlegen. Beides ist auf diesem Rechner erledigt und steht in der README. Ob danach echte Sprache ankam, ist offen: Eine Testaufnahme mit `arecord` war stumm, möglicherweise hat nur niemand gesprochen.
- **`set_audio_ctx`:** In whisper.cpp ist das als EXPERIMENTAL markiert. Ein kleineres Fenster kann bei Whisper zu Wiederholungen oder schlechterer Qualität führen. Mit echter Sprache ist das nicht geprüft. Greedy statt Beam-Search ist auf Qualität ebenfalls nicht geprüft.
- **Modellverzeichnis unter Windows:** `dirs::data_dir()` zeigt auf `%APPDATA%`, also das **Roaming**-Profil. Dort liegen dann 0,5 bis 1,6 GB, die in Firmenumgebungen mitsynchronisiert werden. `dirs::data_local_dir()` (`%LOCALAPPDATA%`) wäre richtig. Ist noch nicht umgestellt, die README nennt `%APPDATA%`.
- **Zwischenablage unter Linux/X11:** Bei egui `copy_text` gehört der Inhalt dem Prozess und kann verschwinden, wenn Mundwerk beendet wird. Das ist eine Vermutung zum X11-Verhalten und nicht geprüft.
- **Unterschied zum Python-Vorläufer:** Neuer Text wird ans Ende angehängt, nicht an der Cursorposition. Bei der Fenster-Variante in Python war es die Cursorposition.
- **Download:** Es gibt keine Prüfsumme. Ein abgebrochener Download bleibt als `.part` liegen und wird beim nächsten Versuch überschrieben, ein kaputtes Modell entsteht also nicht.
- **Windows SmartScreen:** Die `.exe` ist unsigniert, SmartScreen wird warnen. Das steht noch nicht in der README.
- **Lizenz:** Es gilt die vorhandene `LICENSE` im Repo (Copyright elnebuloso). In `/tmp/foo/mundwerk/LICENSE` liegt eine abweichende Fassung mit einem anderen Namen, die nicht übernommen wurde und nicht übernommen werden soll.
- **Chat-Regel des Repos:** Jede finale Antwort beginnt mit `[mundwerk]` (`chat.md`).

## Außerhalb des Repos (Entsorgung nur nach Zustimmung des Nutzers)

- `/tmp/foo/mundwerk/`: die alte Arbeitskopie, inklusive mehrerer GB in `target/`
- `/tmp/foo/whisper/`: das entpackte Python-Vorläuferprojekt mit `diktat.py`/`diktat.bat`. Daneben liegt `/tmp/foo/whisper 1.zip`, das Original des Nutzers.
- `~/.local/share/mundwerk/ggml-large-v3-turbo-q5_0.bin`: das Modell, 0,5 GB. Behalten spart den Download beim Testen.
- Systemänderungen dieser Session, als Info:
  - per apt installiert: `cmake clang libclang-dev libasound2-dev libvulkan-dev glslc libasound2-plugins alsa-utils`
  - `~/.asoundrc` angelegt
