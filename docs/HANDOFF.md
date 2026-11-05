# Übergabe: Mundwerk

## Was Mundwerk ist

Eine lokale Diktier-App in Rust: egui für das Fenster, cpal für Mikrofon und Töne, whisper-rs für whisper.cpp. Man nimmt auf, der erkannte Text landet in einem Textfeld und auf Wunsch gleich in der Zwischenablage. Es gibt kein Konto und keine Cloud, ins Netz geht die App nur für den Modell-Download von Hugging Face. Der ganze Code steht in `src/main.rs`, dazu `scripts/macos-app.sh` für das Mac-Paket. Was die App kann und wie man sie bedient, beschreibt die `README.md` vollständig.

Die Ziele des Nutzers:
- Kein Python auf dem Zielrechner. Ausgeliefert werden Binaries für Windows, Linux (amd64) und Mac (Apple Silicon), alle über GitHub Releases.
- Die GPU wird automatisch genutzt. Der Nutzer hat eine GTX 1070 und einen i5-3570K ohne AVX2.
- „Keep it simple and stupid.“
- Transparenz für Vertrauen: woher die Modelle kommen, wohin sie gespeichert werden, was die App gerade tut.
- Der Hauptablauf des Nutzers: in VS Code (z. B. ins Eingabefeld von Claude Code) per `Strg+Alt+Leertaste` diktieren, während Mundwerk verdeckt im Hintergrund läuft, dann `Strg+V`.

## Stand

- **Funktionen:**
  - Auto-Stopp nach 4 s Stille bzw. 15 s ohne Sprache
  - globales Kürzel `Strg+Alt+Leertaste` (Mac: `⌃⌥Leertaste`)
  - Haken „Automatisch kopieren“ (Standard an)
  - leise Töne bei Start, Stopp und Kopieren, aber nur, wenn außerhalb des Fensters ausgelöst
  - Leeren des Textfelds beim Start per Kürzel
  - Whisper bekommt nur den gesprochenen Teil der Aufnahme
  - luftigere Oberfläche (880×680, mindestens 720×600)
- **Unter Windows vom Nutzer bestätigt (`v0.8.0`):**
  - Die Fußzeile zeigt die GPU.
  - Das Kürzel funktioniert aus VS Code heraus, auch bei minimiertem Fenster.
  - Die Töne und `Strg+V` funktionieren.
  - Zahlen zum GPU-Tempo hat der Nutzer nicht genannt.
- **`v0.9.0` war kaputt:** Unter Windows kam immer „Keine Sprache erkannt“. **`v0.9.1`** behebt das, vom Nutzer unter Windows bestätigt (3,6 s Audio in 0,6 s auf der GPU).
- **Die pegelbasierte Spracherkennung versagt aber auf dem Windows-Mikrofon des Nutzers:** „leisester 0.0077, lautester 0.0178, Sprache ab 0.0306“. Sprache liegt dort nur 2,3-mal (+7 dB) über dem leisesten Abschnitt, Raumrauschen unter WSL ohne Sprache kommt auf das 2,1-fache. Ein Faktor trennt beides nicht. Unter Windows greift deshalb der Zuschnitt auf den Sprachteil nicht (es gibt den Rückfall auf die ganze Aufnahme), und der Auto-Stopp kommt vermutlich erst 15 s nach dem Start statt 4 s nach dem letzten Wort. Die Bestätigung des Nutzers dazu steht aus. Details in TASK-18.
- **Mac:** `Mundwerk.app` erscheint seit `v0.4.0` mit jedem Release, ist aber noch auf keinem echten Mac gelaufen. Der Nutzer hat die App an Kollegen gegeben.
- **Backlog:** Offen sind TASK-5, 9, 10 und 11. Was jeweils fehlt, steht in den Notizen (`backlog task view TASK-n --plain`).

## Nächste Schritte

1. **Spracherkennung per Silero-VAD** (vorgeschlagen, vom Nutzer noch nicht beauftragt): die Pegelmethode beim Auto-Stopp und beim Zuschnitt durch das VAD von whisper.cpp ersetzen. `whisper-rs` 0.16 bringt `WhisperVadContext` mit. Das Modell ist `ggml-org/whisper-vad`, Datei `ggml-silero-v5.1.2.bin`, 885098 Bytes, SHA-256 `29940d98d42b91fbd05ce489f3ecf7c72f0a42f027e4875919a28fb4c04ea2cf`, und würde mit dem Whisper-Modell mitgeladen. Vorher beim Nutzer nachfragen, ob der Auto-Stopp unter Windows nach 4 s oder erst nach 15 s kommt.
2. **TASK-11:** Rückmeldung der Mac-Kollegen einholen:
   - macOS-Version
   - Lief „Trotzdem öffnen“ wie im README beschrieben?
   - Kam die Mikrofon-Abfrage?
   - Text der Fußzeile, Zeile „Text erkannt“
   - Funktioniert `⌃⌥Leertaste`?
3. **Reste**, die noch nicht live geprüft sind:
   - TASK-9: Wechsel auf ein vorhandenes Modell, `Strg+Shift+C`, Protokoll beim allerersten Start
   - TASK-5: eine falsche Prüfsumme
   - TASK-10: ein Build aus einem Pull Request

## Offene Entscheidungen

- **Silero-VAD oder Pegel?** Siehe Nächste Schritte, Punkt 1. Empfohlen ist das VAD, entschieden ist noch nichts.
- **Erkennung einstellbar machen?** Der Nutzer hat gefragt, ob man die Pegel in der Oberfläche einstellen sollte. Empfohlen: keine Zahlen-Regler. Mit dem VAD entfällt die Frage weitgehend. Bleibt es bei der Pegelmethode, höchstens ein Strich im Pegelbalken für die Schwelle.
- **GPU unter WSL:** Vulkan sieht dort keine NVIDIA-Karte, das ginge nur mit einem CUDA-Build als zweitem Artefakt. Bisher gilt: unter WSL die CPU, für die GPU die Windows-`.exe`.
- **Sprache der Oberfläche:** im Moment Deutsch. Englisch oder umschaltbar wurde nie besprochen.
- **Signieren:** Der Nutzer hat entschieden, die `.exe` nicht zu signieren. Die Mac-App ist ebenfalls nur ad-hoc signiert und nicht notarisiert. Beide Wege zum Öffnen („Trotzdem ausführen“ bzw. „Trotzdem öffnen“) stehen im README. Für Windows-Signaturen wäre die SignPath Foundation der erste Schritt, für Open Source kostenlos. Für den Mac kostet das Apple-Konto 99 $ im Jahr.
- **Direktes Einfügen** (Stufe 2: Mundwerk drückt selbst `Strg+V`): besprochen, aber nicht beauftragt. Der Nutzer hat sich vorerst für Kopieren und Einfügen von Hand entschieden.

## Stolperfallen

**Releases**
- **Release-Texte:** Nach jedem grünen Release kommt ein deutscher Text für Nutzer über die generierte Commit-Liste (`gh release edit <tag> --notes-file …`). Die Regel steht in `.claude/rules/release-notes.md`, frühere Releases dienen als Vorbild. `gh release edit` antwortet gelegentlich mit HTTP 500/502, dann einfach wiederholen und am Ende prüfen.
- **Release nur halb erschienen:** Der Tag ist da, das GitHub-Release fehlt. Ein Neustart des Laufs hilft dann nicht, weil semantic-release die Version für veröffentlicht hält. Lösung: `gh release create <tag> --verify-tag --latest` mit den Artefakten des Laufs (`gh run download <id>`), vorher die Version in den Binaries prüfen. So geschehen bei `v0.6.0`, beschrieben in TASK-10. Die dabei fehlende Git-Notiz `refs/notes/semantic-release-…` stört nicht.
- **Mac-Runner:** Sie warten bei GitHub teils über 10 Minuten. Einmal wurde ein Job nach 15 Minuten ohne einen einzigen Schritt abgebrochen, dann hilft `gh run rerun <id> --failed`.
- **Berechtigungen:** `git push` und `gh release create` blockiert der Berechtigungsschutz, solange der Nutzer sie nicht ausdrücklich erlaubt („push erlaubt“, „ist erlaubt“). Ein „ja“ auf eine Frage mit mehreren Teilen hat einmal nicht gereicht.
- **Startpunkt für semantic-release:** Den Tag `v0.0.0` und das Release „placeholder release for semantic release“ hat der Nutzer angelegt. Nicht löschen und keinen Release-Text dafür schreiben. Unter 1.0 gilt `v0-commits.md`: keine Breaking-Change-Marker, sonst entsteht `1.0.0`.
- **Version in `Cargo.toml`:** Sie bleibt bei `0.1.0` und hat keine Bedeutung. Die Release-Version kommt beim Build über `MUNDWERK_VERSION` herein, lokale Builds zeigen „dev“. Nicht hochzählen.

**CI**
- **CI startet nur bei Code-Pfaden:** `src/**`, `scripts/**`, `Cargo.toml`, `Cargo.lock`, `ci.yml` und `.releaserc.json`. Pushes, die nur Doku oder Backlog ändern, bauen nichts. Ist ein Build rot, von Hand starten: `gh workflow run ci.yml`.
- **Cache-Schlüssel:** Er setzt sich aus Betriebssystem, Rust-Version und `Cargo.lock` zusammen. Jede neue Abhängigkeit heißt etwa 8–10 Minuten Neubau, der Mac-Build braucht ohne Cache gut 3 Minuten.
- **Diese Einstellungen nicht „optimieren“:**
  - `GGML_AVX2=OFF`, weil der i5-3570K sonst mit „illegal instruction“ abstürzt. Die `GGML_AVX*`-Schalter greifen nur auf x86, auf dem Mac sind sie wirkungslos.
  - `CARGO_TARGET_DIR=C:/t` unter Windows (Pfadlimit von 260 Zeichen, Fehler `FTK1011`)
  - `libvulkan-dev` im Linux-Build, sonst fehlt `-lvulkan`
  - Mac: `--no-default-features --features metal`, `MACOSX_DEPLOYMENT_TARGET=13.0`, kein Vulkan SDK. `scripts/macos-app.sh` signiert ad-hoc und packt mit `ditto`, weil `zip` die Signatur beschädigen kann.
- **Linux-Abhängigkeiten im CI:** Einmal dauerten sie 10 Minuten, weil der Azure-Mirror langsam war, sonst etwa 20 s.
- **Schreibrechte im Job `version`:** Ob semantic-release im Probelauf Schreibrechte braucht, ist eine ungeprüfte Annahme. Mit Schreibrechten funktioniert es.

**Spracherkennung**
- **`SILENCE` (0,01) ist eine Grenze für Spitzen** und gilt nur für `is_silent`, also „alle Samples darunter“. Sie darf nicht als Mindestwert für den RMS je 100 ms dienen. Genau das war der Fehler in `v0.9.0`. Für Sprache gilt `SPEECH_MIN_RMS` (0,002). Als Sprache zählt, was um `SPEECH_OVER_FLOOR` (×4, +12 dB) über dem leisesten 100-ms-Fenster der Aufnahme liegt. Diese Werte sind nur unter WSL und mit Sprachausgabe gemessen.
- **Keine Aufnahme verwerfen:** Ohne erkannte Sprache geht die ganze Aufnahme an Whisper. Das ist eine bewusste Entscheidung nach `v0.9.0`: Ein falsch eingeschätztes Mikrofon darf kein Diktat kosten.
- **`no_speech_probability()` von whisper.cpp ist bei `large-v3-turbo-q5_0` immer 0,000,** egal ob Rauschen oder Sprache. Als Filter ungeeignet.
- **Grenzen der Erkennung:** Klopfen und Tippen zählen als Sprache. Anmerkungen von Whisper wie „*Signal*“ oder „[Musik]“ werden nicht gefiltert.
- **„Rückgängig“ verhält sich anders als früher:** Es übersteht jetzt ein Diktat und setzt den geleerten Text **vor** das neue (`restore_cleared`).

**Testen unter WSLg**
- **App starten:** mit `setsid env -u WAYLAND_DISPLAY ./target/release/mundwerk … &`. Ohne `setsid` stirbt der Prozess, sobald der Shell-Befehl endet. Laufende Instanzen vorher beenden (`pkill -x mundwerk`).
- **Fokus:** Der X-Fokus folgt dem Windows-Fokus. Ist ein Windows-Programm aktiv, hat kein X-Fenster den Fokus, und X11 verwirft Tasten, auch das globale Kürzel. Zum Testen des Kürzels ein frisches Fenster öffnen (`setsid xmessage … &`), das bekommt kurz den Fokus. In dieser Zeit `xdotool key ctrl+alt+space` senden.
- **Klicks:** erst `mousemove --window <id> x y`, dann `mousedown 1` und `mouseup 1`. `xdotool windowfocus` scheitert oft. `Strg+V` per `xdotool key --window` kam einmal nicht an, einfach wiederholen.
- **Wo was liegt bei 880×680:**
  - Aufnahme-Button: etwa (180, 154)
  - Textfeld: etwa (400, 350)
  - Zeile mit „Leeren“ und „Rückgängig“: y ≈ 551
  - Fußzeile: y ≈ 650
- **Screenshots:** `import -window Mundwerk out.png`. Während einer Aufnahme heißt das Fenster „● Aufnahme – Mundwerk“.
- **Ton und Zwischenablage:** Den Ton schneidet `parec -d RDPSink.monitor --format=s16le --rate=16000 --channels=1 --raw` mit, die Zwischenablage zeigt `xclip -o -selection clipboard`.
- **Messen mit echten Aufnahmen:** ein vorübergehendes `#[cfg(test)] mod probe` mit `#[ignore]`-Test, der f32-Rohdaten aus Dateien liest (Pfade per Umgebungsvariable). Nach der Messung wieder entfernen.
- **Mikrofon unter WSL:** Es braucht zwei Dinge: die ALSA-Weiterleitung an PulseAudio (README, Abschnitt „WSL“) und die Windows-Freigabe „Desktop-Apps den Zugriff auf Ihr Mikrofon erlauben“. Das WSL-Mikrofon ist sehr laut eingestellt (Spitzen etwa 0,45). Pegel von dort taugen nicht als Maßstab für andere Rechner. Auf der CPU braucht die Erkennung von 15 s Raumrauschen 45–70 s.
- **Playwright** funktioniert nicht, Mundwerk ist ein natives Fenster.

**Mac**
- Noch komplett ungetestet.
- `⌃⌥Leertaste` ist auf dem Mac womöglich schon als „Nächste Eingabequelle“ belegt, das ist ungeprüft.
- Die Dialogtexte im README-Abschnitt „Mac“ („wurde nicht geöffnet“, „Trotzdem öffnen“) stammen aus dem Gedächtnis.
- Laut Quelltext von `global-hotkey` braucht das Kürzel keine Freigabe unter „Bedienungshilfen“ (`RegisterEventHotKey`).
- Ob macOS bei einer nur ad-hoc signierten App nach dem Mikrofon fragt, ist offen.

**Sonstiges**
- **Ungeklärt:** Bei einem Test des Nutzers landete der Text einmal am Ende statt an der Cursorposition. Er hatte mit `make run` unter Wayland gestartet. Unter X11 nicht nachstellbar.
- **Unbelegte Angaben im README:** die CPU-Generationen für AVX und F16C stammen aus dem Gedächtnis.
- **Zwischenablage unter Linux/X11:** Dass der Inhalt nach dem Beenden von Mundwerk verschwinden kann, ist weiterhin nur eine Vermutung.

## Entwicklungsrechner

- **Modell:** Unter WSL liegt `large-v3-turbo-q5_0` in `~/.mundwerk/models/`. Die Rust-Toolchain `1.99.0` ist als Default gesetzt.
- **Per apt nachinstalliert:** `xdotool`, `espeak-ng`, `xclip`, `x11-utils` und `pulseaudio-utils`.
- **WSL-Speicher:** Die `.wslconfig` des Nutzers setzt `memory=12GB`, `swap=8GB` und `swapFile=D:\\wsl\\swap.vhdx`. `swapFile` war beim Schreiben dieser Übergabe noch nicht aktiv, das greift erst nach `wsl --shutdown`. Danach sollte `D:\wsl\swap.vhdx` existieren.
- **Platz auf C:** In `%TEMP%` lagen etwa 22 GB alte Auslagerungsdateien von WSL (zwei GUID-Ordner mit je rund 8 GB) und `wsl-crashes` (6 GB). Löschen erst nach `wsl --shutdown`.
- **RAM:** Viele alte Claude-Code- und Playwright-Prozesse sowie die `grimoire`-Dev-Server belegten den Arbeitsspeicher, die Auslagerung war zu 5,6 von 8 GB voll.
- **Außerhalb des Repos:** `/tmp/foo/whisper 1.zip` ist das Original des Python-Vorläufers des Nutzers. Behalten, und es gehört nicht ins Repo.
