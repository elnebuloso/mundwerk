# Mundwerk

Lokales Diktieren mit Whisper: sprechen, Text kopieren. Offline, ohne Konto.

Ein kleines Fenster: **Aufnahme starten → sprechen → stoppen**. Der Text landet im Textfeld, wo du ihn korrigieren und mit einem Klick in die Zwischenablage kopieren kannst. Die Spracherkennung läuft komplett auf deinem Rechner ([whisper.cpp](https://github.com/ggml-org/whisper.cpp)), es geht kein Audio ins Netz.

## Installation

Windows 10/11 (64 Bit): Unter [Releases](../../releases) `mundwerk-windows-x86_64.exe` herunterladen und starten. Python oder andere Installationen sind nicht nötig. Für Linux gibt es vorerst kein fertiges Binary, siehe [Selbst bauen](#selbst-bauen).

Mundwerk legt alles in einem Ordner ab: `%USERPROFILE%\.mundwerk\` unter Windows, `~/.mundwerk/` unter Linux.

- `models/`: die Modelle. Beim ersten Laden wird ein Modell einmalig von Hugging Face heruntergeladen (0,15–1,6 GB). Nach dem Download prüft Mundwerk die SHA-256-Prüfsumme gegen die von Hugging Face und verwirft die Datei, wenn sie nicht stimmt.
- `mundwerk.log`: das Protokoll, dieselben Zeilen wie im Fenster unten.

Die Modellauswahl zeigt für jedes Modell, ob es schon da ist (● grün, „vorhanden“) oder noch heruntergeladen wird (○ orange, „nicht heruntergeladen“), und für das gewählte Modell die Download-Adresse und den Zielpfad.

## Modelle

| Modell | Größe | Hinweis |
|---|---|---|
| `large-v3-turbo-q5_0` | 0,5 GB | beste Wahl mit GPU |
| `large-v3-turbo` | 1,6 GB | etwas genauer, braucht mehr Speicher |
| `small` | 0,5 GB | guter Kompromiss ohne GPU |
| `base` | 0,15 GB | sehr schnell, ungenauer |

## GPU

Mundwerk nutzt über **Vulkan** automatisch eine vorhandene Grafikkarte (NVIDIA, AMD, Intel). Ohne passende GPU rechnet es auf der CPU, das geht auch, ist aber bei großen Modellen deutlich langsamer. Die Fußzeile `Rechenwerk:` zeigt, was genutzt wird, und bei CPU den Grund.

Unter Windows bringt der normale Grafiktreiber Vulkan mit. Unter Linux wird `libvulkan1` benötigt (auf Desktop-Systemen meist vorhanden).

## Linux / WSL

- Benötigt `libasound2` und `libvulkan1`.
- **WSL:** Das Fenster öffnet sich über WSLg. Für das Mikrofon muss ALSA an den PulseAudio-Server von WSLg weitergeleitet werden:

  ```sh
  sudo apt install libasound2-plugins
  printf 'pcm.!default { type pulse }\nctl.!default { type pulse }\n' > ~/.asoundrc
  ```

  In WSL gibt es kein Vulkan für die Grafikkarte, Mundwerk rechnet dort auf der CPU. Für GPU-Tempo die Windows-Version nutzen.

## Selbst bauen

Rust, CMake, Clang und das Vulkan SDK (bzw. unter Linux `libvulkan-dev glslc libasound2-dev`):

```sh
cargo build --release
```

Ohne GPU-Unterstützung: `cargo build --release --no-default-features`

## Lizenz

MIT. Whisper-Modelle: MIT (OpenAI), whisper.cpp: MIT.
