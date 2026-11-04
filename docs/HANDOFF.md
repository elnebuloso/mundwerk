# Übergabe: Mundwerk

## Was Mundwerk ist

Eine lokale Diktier-App in Rust, mit egui für das Fenster, cpal für das Mikrofon und whisper-rs für whisper.cpp. Man wählt ein Modell, nimmt auf, und der Text landet in einem Textfeld, aus dem man ihn kopiert. Es gibt kein Konto und keine Cloud. Ins Netz geht die App nur für den Modell-Download von Hugging Face. Der ganze Code steht in `src/main.rs`. Was die App kann und wie man sie bedient, beschreibt die `README.md` vollständig. Sie ist als Ziel-README für den fertigen Zustand geschrieben.

Die Ziele des Nutzers:
- Kein Python auf dem Zielrechner. Ausgeliefert werden Binaries für Windows und Linux amd64, beide über GitHub Releases.
- Die GPU wird automatisch genutzt. Der Nutzer hat eine GTX 1070 und einen i5-3570K ohne AVX2.
- „Keep it simple and stupid.“
- Transparenz für Vertrauen: woher die Modelle kommen, wohin sie gespeichert werden, was die App gerade tut.

## Stand

- **Releases laufen automatisch** über `.github/workflows/ci.yml` und `.releaserc.json`, nach dem Vorbild von `/workspace/github.com/elnebuloso/grimoire`. Ein Push auf `main`, der Code ändert, baut und testet beide Plattformen. Ist seit dem letzten Tag ein `feat` oder `fix` dabei, entsteht ein Tag `vX.Y.Z` mit `mundwerk-linux-x86_64` und `mundwerk-windows-x86_64.exe`. Zuletzt ist `v0.3.0` erschienen.
- **Build-Cache:** Mit gefülltem Cache dauert der Build 11 s unter Linux und 30 s unter Windows, ohne Cache 476 s bzw. 426 s.
- **Unter WSL (CPU) geprüft:**
  - Diktat mit echter Sprache, korrekt erkannt. 3 s Sprache brauchen 15–25 s.
  - Download mit Prüfsumme und Abbrechen
  - alle Zustände der Oberfläche, per Screenshot
  - Dark/Light-Schalter samt Speichern der Wahl
  - Version in der Fußzeile
- **Unter Windows:** Der Nutzer hat ein frühes Windows-Artefakt gestartet. Daher stammt der Wunsch nach dem Dark-Mode. Ob die Fußzeile die GPU zeigt und wie schnell es damit ist, ist **nicht bekannt**.
- **Backlog:** offen sind TASK-5, TASK-6, TASK-8, TASK-9 und TASK-10. Welche Abnahmekriterien noch fehlen und warum, steht jeweils in den Notizen (`backlog task view TASK-n --plain`).

## Nächste Schritte

1. **TASK-8:** Die `.exe` des neuesten Releases auf dem Windows-Rechner des Nutzers testen.
   - Zeigt die Fußzeile `GPU – NVIDIA GeForce GTX 1070 (Vulkan0), automatisch gewählt`?
   - Wie lange dauert ein Diktat (Zeile „Text erkannt: …“ im Protokoll)?
   - Liegt der Ordner unter `%USERPROFILE%\.mundwerk`?
   Damit lassen sich auch TASK-6 (AC 1) und TASK-2 für Windows abschließen.
2. **Was noch nicht live geprüft ist** und in TASK-9, TASK-5 und TASK-10 offen steht:
   - Wechsel auf ein schon vorhandenes Modell (Auto-Laden)
   - eine stumme Aufnahme mit der Meldung „Kein Ton aufgenommen“
   - `Strg+Shift+C`
   - das Protokoll beim allerersten Start
   - eine falsche Prüfsumme
   - ein Build aus einem Pull Request, der nicht released

## Offene Entscheidungen

- **GPU unter WSL:** Vulkan sieht dort keine NVIDIA-Karte. Möglich wäre das nur mit einem CUDA-Build als zweitem Artefakt, dafür bräuchte es das CUDA-Toolkit im Build und die CUDA-Bibliotheken beim Nutzer. Bisher gilt: unter WSL die CPU, für die GPU die Windows-`.exe`.
- **Sprache der Oberfläche:** im Moment Deutsch. Englisch oder umschaltbar wurde nie besprochen.
- **Halluzinationen bei Rauschen:** Ohne Sprache erfindet Whisper kurze Sätze („Vielen Dank.“, „Ja.“). Die Stille-Erkennung fängt nur echte Stille ab. whisper.cpp liefert pro Segment eine No-Speech-Wahrscheinlichkeit, damit ließe sich mehr filtern. Dafür gibt es noch keinen Task.
- **Signieren der `.exe`:** Der Nutzer hat entschieden, das nicht anzugehen. SmartScreen warnt bei jedem Release, „Zulassen“ in den Eigenschaften bzw. `Unblock-File` hilft. Sollte das später doch kommen, wäre der erste Schritt die SignPath Foundation, für Open Source ist sie kostenlos.

## Stolperfallen

- **Startpunkt für semantic-release:** Der Tag `v0.0.0` und das Release „placeholder release for semantic release“ hat der Nutzer angelegt. Sie sind der Startpunkt und dürfen nicht gelöscht werden. Ohne sie würde semantic-release mit `1.0.0` beginnen. Unter 1.0 gilt `v0-commits.md`: keine Breaking-Change-Marker, sonst entsteht ebenfalls `1.0.0`.
- **Version in `Cargo.toml`:** Sie bleibt bei `0.1.0` und hat keine Bedeutung. Die Release-Version kommt beim Build über `MUNDWERK_VERSION` herein, lokale Builds zeigen „dev“. Nicht hochzählen.
- **CI startet nur bei Code-Pfaden:** `src/**`, `Cargo.toml`, `Cargo.lock`, `ci.yml` und `.releaserc.json`. Ist ein Build rot und wird danach nur Doku gepusht, läuft nichts erneut. Dann von Hand starten (`gh workflow run ci.yml`).
- **Cache-Schlüssel:** Er setzt sich aus Betriebssystem, Rust-Version und `Cargo.lock` zusammen. Ändert sich eins davon, baut der nächste Lauf alles neu, das dauert etwa 8 Minuten.
- **Diese Einstellungen nicht „optimieren“:**
  - `GGML_AVX2=OFF`, weil der i5-3570K sonst mit „illegal instruction“ abstürzt
  - `CARGO_TARGET_DIR=C:/t` unter Windows, weil whisper.cpp sonst über das Pfadlimit von 260 Zeichen kommt (Fehler `FTK1011`)
  - `libvulkan-dev` im Linux-Build, sonst fehlt beim Linken `-lvulkan`
- **Linux-Abhängigkeiten im CI:** Einmal hat „Linux dependencies“ 10 Minuten gedauert, weil der Azure-Mirror nur 54 kB/s lieferte, sonst sind es etwa 20 s. Vorinstallierte Pakete streichen spart wenig und ist ungeprüft.
- **Schreibrechte im Job `version`:** Ob semantic-release im Probelauf Schreibrechte braucht, ist eine ungeprüfte Annahme. Mit Schreibrechten funktioniert es.
- **Einfügen am Cursor:** Bei einem Test des Nutzers landete der Text einmal am Ende statt an der Cursorposition. Er hatte mit `make run` unter Wayland gestartet. Unter X11 konnte ich das nicht nachstellen, und der Nutzer hat es nicht weiter verfolgt. Ungeklärt.
- **Screenshot-Tests unter WSL:**
  - Die App mit `env -u WAYLAND_DISPLAY ./target/release/mundwerk` starten.
  - Screenshot mit `import -window Mundwerk out.png`. Während einer Aufnahme heißt das Fenster „● Aufnahme – Mundwerk“.
  - Klicks mit `xdotool`: erst `windowfocus`, dann `mousemove --window`, dann `mousedown 1` und `mouseup 1`. Ein einfaches `click` kommt nicht an, und `xdotool type` kommt auch nicht an. Tasten gehen mit `xdotool key --window <id> ctrl+r`.
  - `xdotool` wurde per apt installiert.
  - Laufende Instanzen vorher beenden (`pkill -x mundwerk`), sonst landen die Tasten in der falschen Instanz.
- **Playwright** funktioniert nicht, Mundwerk ist ein natives Fenster.
- **Mikrofon unter WSL:** Es braucht zwei Dinge: die ALSA-Weiterleitung an PulseAudio (README, Abschnitt „WSL“) und die Windows-Freigabe „Desktop-Apps den Zugriff auf Ihr Mikrofon erlauben“. Ohne die Freigabe kommt nur Stille an. Prüfen lässt sich das mit `arecord -d 5 -f S16_LE -r 16000 -c 1 <datei>.wav` und dem Spitzenwert: bei Stille 1, bei Sprache etwa 15000.
- **Unbelegte Angaben in der README:** Die CPU-Generationen für AVX und F16C (Ivy Bridge, Piledriver, Jaguar) stammen aus dem Gedächtnis.
- **Zwischenablage unter Linux/X11:** Dass der Inhalt nach dem Beenden von Mundwerk verschwinden kann, ist weiterhin nur eine Vermutung.
- **Entwicklungsrechner:** Unter WSL liegt `large-v3-turbo-q5_0` (0,5 GB) in `~/.mundwerk/models/`, das spart den Download beim Testen. Die Rust-Toolchain `1.99.0` ist als Default gesetzt.
- **Außerhalb des Repos:** `/tmp/foo/whisper 1.zip` ist das Original des Python-Vorläufers des Nutzers. Behalten, und es gehört nicht ins Repo.
