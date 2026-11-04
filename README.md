# Mundwerk

Lokales Diktieren mit Whisper: sprechen, Text kopieren. Offline, ohne Konto.

Ein kleines Fenster: **Aufnahme starten → sprechen → stoppen**. Der erkannte Text landet im Textfeld, wo du ihn korrigierst und in die Zwischenablage kopierst. Die Spracherkennung läuft komplett auf deinem Rechner ([whisper.cpp](https://github.com/ggml-org/whisper.cpp)). Ins Netz geht Mundwerk nur, um ein Modell herunterzuladen – nie mit deinem Audio.

## Installation

Unter [Releases](../../releases) die passende Datei herunterladen und starten. Python oder andere Installationen sind nicht nötig.

| System | Datei | Voraussetzung |
|---|---|---|
| Windows 10/11 (64 Bit) | `mundwerk-windows-x86_64.exe` | – |
| Linux x86_64 | `mundwerk-linux-x86_64` | glibc 2.35 oder neuer (z. B. Ubuntu 22.04), `libasound2`, `libvulkan1` |

Der Prozessor muss AVX und F16C können: Intel ab Ivy Bridge, AMD ab Piledriver bzw. Jaguar, also etwa ab Baujahr 2012.

**Windows:** Die `.exe` ist nicht signiert. SmartScreen warnt beim ersten Start; „Weitere Informationen“ › „Trotzdem ausführen“ startet sie.

**Linux:** Vor dem ersten Start ausführbar machen:

```sh
chmod +x mundwerk-linux-x86_64
./mundwerk-linux-x86_64
```

## Erster Start

1. Modell auswählen – voreingestellt ist das empfohlene `large-v3-turbo-q5_0`.
2. „Herunterladen (0,5 GB)“ klicken. Der Download lässt sich mit „Abbrechen“ stoppen.
3. Sobald „✓ geladen“ erscheint: „Aufnahme starten“, sprechen, „Aufnahme stoppen“.

Ab dem nächsten Start lädt Mundwerk das Modell selbst.

## Bedienung

| Taste | Wirkung |
|---|---|
| `Strg+R` | Aufnahme starten / stoppen |
| `Strg+Shift+C` | ganzen Text in die Zwischenablage kopieren |

Die Tasten wirken nur, solange das Mundwerk-Fenster im Vordergrund ist.

- Neuer Text wird an der Cursorposition eingefügt. Ohne gesetzten Cursor kommt er ans Ende.
- „Leeren“ löscht den ganzen Text. „Rückgängig“ holt ihn zurück, aber nur bis zum nächsten Diktat oder zur nächsten Änderung im Textfeld.
- Die Sprache stellst du rechts neben dem Aufnahme-Button ein: „Deutsch“, „Englisch“ oder „Automatisch“.
- Mundwerk startet dunkel, unabhängig von der Systemeinstellung. Rechts in der Fußzeile schaltest du auf „☀ Hell“ um; die Wahl gilt auch beim nächsten Start.
- Bei Rauschen ohne Sprache erfindet Whisper manchmal kurze Sätze wie „Vielen Dank.“ oder „Ja.“ – einfach löschen.

## Modelle

| Modell | Größe | Hinweis |
|---|---|---|
| `large-v3-turbo-q5_0` | 0,5 GB | Empfohlen |
| `large-v3-turbo` | 1,6 GB | Genauer, langsamer, braucht mehr Speicher |
| `base` | 0,15 GB | Am schnellsten, ungenauer |

In der Auswahl steht ● „vorhanden“ für heruntergeladene Modelle und ○ „noch nicht heruntergeladen“ für die übrigen. Ein vorhandenes Modell wird beim Auswählen sofort geladen. Unter der Auswahl stehen die Download-Adresse und der Speicherort des gewählten Modells.

Geladen wird von `https://huggingface.co/ggerganov/whisper.cpp`. Nach dem Download vergleicht Mundwerk die SHA-256-Prüfsumme mit der, die Hugging Face veröffentlicht, und verwirft die Datei, wenn sie nicht stimmt.

## GPU oder CPU

Mundwerk nutzt über Vulkan automatisch eine vorhandene Grafikkarte (NVIDIA, AMD, Intel) und rechnet sonst auf der CPU. Die Fußzeile „Berechnung läuft auf:“ zeigt, was genutzt wird, und bei CPU den Grund:

- „keine Vulkan-fähige GPU gefunden“ – Grafiktreiber aktualisieren. Unter Windows bringt der normale Treiber Vulkan mit, unter Linux muss `libvulkan1` samt Treiber installiert sein.
- „ohne GPU-Unterstützung gebaut“ – ein selbst gebautes Binary mit `--no-default-features`.

Auf der CPU dauert die Erkennung ein Vielfaches der Aufnahme: Auf einem i5-3570K brauchten 3 s Sprache mit `large-v3-turbo-q5_0` 15–25 s. Mit `base` geht es deutlich schneller.

## Dateien

Alles liegt in einem Ordner: `%USERPROFILE%\.mundwerk\` unter Windows, `~/.mundwerk/` unter Linux.

- `models/` – die heruntergeladenen Modelle
- `mundwerk.log` – das Protokoll, dieselben Zeilen wie unter „Protokoll“ im Fenster, mit Sekunden seit dem Programmstart
- `theme` – `light`, wenn du auf hell umgeschaltet hast

Zum Deinstallieren die Programmdatei und diesen Ordner löschen.

## Wenn etwas nicht klappt

Fehler stehen rot mit ⚠ unter dem Aufnahme-Button und zusätzlich im Protokoll.

**„Kein Ton aufgenommen“** – Das Mikrofon liefert nur Stille. Unter Windows: `Einstellungen > Datenschutz und Sicherheit > Mikrofon`, dort „Mikrofonzugriff“ und „Desktop-Apps den Zugriff auf Ihr Mikrofon erlauben“ einschalten. Mundwerk nimmt über das Standard-Aufnahmegerät des Systems auf; welches das ist, steht im Protokoll („Aufnahme über Mikrofon: …“).

**„Kein Mikrofon gefunden“** – Es gibt kein Standard-Aufnahmegerät. Mikrofon anschließen bzw. in den Sound-Einstellungen als Eingabegerät wählen.

**„Download fehlgeschlagen“ / „Download unvollständig“** – Verbindung zu `huggingface.co` prüfen und erneut auf „Herunterladen“ klicken. Ein abgebrochener Download hinterlässt keine kaputte Datei.

**„Prüfsumme falsch, Datei verworfen“** – Die geladene Datei war nicht die von Hugging Face veröffentlichte. Sie wurde gelöscht; erneut herunterladen.

**„Hugging Face hat keine Prüfsumme geliefert“** – Hugging Face hat anders geantwortet als erwartet, Mundwerk lädt dann nichts. Später erneut versuchen.

## WSL

Mundwerk läuft auch unter WSL2 mit WSLg, dort aber immer auf der CPU – Vulkan sieht in WSL keine Grafikkarte. Für GPU-Tempo die Windows-Version nutzen.

Für das Mikrofon muss ALSA an den PulseAudio-Server von WSLg weitergeleitet werden:

```sh
sudo apt install libasound2-plugins
printf 'pcm.!default { type pulse }\nctl.!default { type pulse }\n' > ~/.asoundrc
```

Außerdem muss Windows Desktop-Apps den Mikrofonzugriff erlauben, siehe „Kein Ton aufgenommen“.

## Selbst bauen

Benötigt Rust, CMake, Clang und das Vulkan SDK; unter Linux zusätzlich `libasound2-dev`, unter Windows die Visual Studio Build Tools mit „Desktopentwicklung mit C++“.

```sh
make build   # Release-Binary nach target/release/
make run     # bauen und starten
make test    # Unit-Tests
```

Ohne GPU-Unterstützung: `cargo build --release --no-default-features`

Unter Windows überschreitet der Build von whisper.cpp unter `.\target` die Pfadgrenze von 260 Zeichen. Mit einem kurzen Build-Verzeichnis geht es, z. B. `set CARGO_TARGET_DIR=C:\t` vor `cargo build --release`.

## Releases

Jeder Push auf `main`, der Code ändert (`src/`, `Cargo.toml`, `Cargo.lock`, die CI-Konfiguration), wird für Linux und Windows gebaut und getestet. Enthält er seit dem letzten Release einen Commit `feat:` oder `fix:` nach [Conventional Commits](https://www.conventionalcommits.org/), erzeugt semantic-release den Tag `vX.Y.Z` und ein GitHub-Release mit beiden Binaries. `chore:`, `docs:`, `ci:`, `refactor:` und `test:` lösen kein Release aus; Änderungen nur an README oder Doku starten gar keinen Build.

## Lizenz

MIT. Whisper-Modelle: MIT (OpenAI), whisper.cpp: MIT.
