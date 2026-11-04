# Mundwerk

Lokales Diktieren mit Whisper: sprechen, Text kopieren. Offline, ohne Konto.

Ein kleines Fenster: **Aufnahme starten → sprechen → stoppen**. Der Text landet im Textfeld, wo du ihn korrigieren und mit einem Klick in die Zwischenablage kopieren kannst. Die Spracherkennung läuft komplett auf deinem Rechner ([whisper.cpp](https://github.com/ggml-org/whisper.cpp)), es geht kein Audio ins Netz.

## Installation

Windows 10/11 (64 Bit): Unter [Releases](../../releases) `mundwerk-windows-x86_64.exe` herunterladen und starten. Python oder andere Installationen sind nicht nötig. Für Linux gibt es vorerst kein fertiges Binary, siehe [Selbst bauen](#selbst-bauen).

Mundwerk legt alles in einem Ordner ab: `%USERPROFILE%\.mundwerk\` unter Windows, `~/.mundwerk/` unter Linux.

- `models/`: die Modelle. Ein Modell wird einmalig mit „Herunterladen“ von Hugging Face geholt (0,15–1,6 GB); der Download lässt sich abbrechen. Nach dem Download prüft Mundwerk die SHA-256-Prüfsumme gegen die von Hugging Face und verwirft die Datei, wenn sie nicht stimmt.
- `mundwerk.log`: das Protokoll mit Sekunden seit dem Programmstart, dieselben Zeilen wie unter „Protokoll“ im Fenster.

Die Modellauswahl zeigt für jedes Modell, ob es schon da ist (● „vorhanden“) oder nicht (○ „noch nicht heruntergeladen“), und für das gewählte Modell die Download-Adresse und den Zielpfad. Ein vorhandenes Modell wird beim Start und beim Auswählen automatisch geladen.

## Bedienung

| Taste | Wirkung |
|---|---|
| `Strg+R` | Aufnahme starten / stoppen |
| `Strg+Shift+C` | ganzen Text in die Zwischenablage kopieren |

Die Tasten wirken nur, solange das Mundwerk-Fenster im Vordergrund ist. Neuer Text wird an der Cursorposition im Textfeld eingefügt. „Leeren“ lässt sich bis zum nächsten Diktat oder zur nächsten Änderung mit „Rückgängig“ zurücknehmen.

Kommt bei einer Aufnahme gar kein Ton an, meldet Mundwerk „Kein Ton aufgenommen“ statt Text zu erraten. Bei leisem Rauschen ohne Sprache erfindet Whisper manchmal kurze Sätze wie „Vielen Dank.“ – dann einfach löschen.

## Modelle

| Modell | Größe | Hinweis |
|---|---|---|
| `large-v3-turbo-q5_0` | 0,5 GB | Empfohlen |
| `large-v3-turbo` | 1,6 GB | Genauer, langsamer, braucht mehr Speicher |
| `base` | 0,15 GB | Am schnellsten, ungenauer |

## GPU

Mundwerk nutzt über **Vulkan** automatisch eine vorhandene Grafikkarte (NVIDIA, AMD, Intel). Ohne passende GPU rechnet es auf der CPU, das geht auch, ist aber bei großen Modellen deutlich langsamer. Die Fußzeile „Berechnung läuft auf:“ zeigt, was genutzt wird, und bei CPU den Grund.

Unter Windows bringt der normale Grafiktreiber Vulkan mit. Unter Linux wird `libvulkan1` benötigt (auf Desktop-Systemen meist vorhanden).

## Linux / WSL

- Benötigt `libasound2` und `libvulkan1`.
- **WSL:** Das Fenster öffnet sich über WSLg. Für das Mikrofon muss ALSA an den PulseAudio-Server von WSLg weitergeleitet werden:

  ```sh
  sudo apt install libasound2-plugins
  printf 'pcm.!default { type pulse }\nctl.!default { type pulse }\n' > ~/.asoundrc
  ```

  Kommt trotzdem nur Stille an (Whisper erkennt dann oft „Vielen Dank“), muss Windows den Zugriff erlauben: `Einstellungen > Datenschutz und Sicherheit > Mikrofon`, dort „Mikrofonzugriff“ und „Desktop-Apps den Zugriff auf Ihr Mikrofon erlauben“ einschalten.

  In WSL gibt es kein Vulkan für die Grafikkarte, Mundwerk rechnet dort auf der CPU. Für GPU-Tempo die Windows-Version nutzen.

## Selbst bauen

Rust, CMake, Clang und das Vulkan SDK (bzw. unter Linux `libvulkan-dev glslc libasound2-dev`):

```sh
cargo build --release
```

Ohne GPU-Unterstützung: `cargo build --release --no-default-features`

## Lizenz

MIT. Whisper-Modelle: MIT (OpenAI), whisper.cpp: MIT.
