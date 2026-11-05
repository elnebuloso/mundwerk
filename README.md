<p align="center">
  <img src="logo.png" alt="Mundwerk" width="128">
</p>

<h1 align="center">Mundwerk</h1>

<p align="center">
  Lokales Diktieren mit Whisper: sprechen, Text kopieren. Offline, ohne Konto.
</p>

<p align="center">
  <a href="https://github.com/elnebuloso/mundwerk/releases">Releases</a>
</p>

Ein kleines Fenster: **Aufnahme starten → sprechen → stoppen**. Der erkannte Text landet im Textfeld, wo du ihn korrigierst und in die Zwischenablage kopierst. Die Spracherkennung läuft komplett auf deinem Rechner ([whisper.cpp](https://github.com/ggml-org/whisper.cpp)). Ins Netz geht Mundwerk nur, um Modelle herunterzuladen – nie mit deinem Audio.

## Installation

Unter [Releases](../../releases) die passende Datei herunterladen und starten. Python oder andere Installationen sind nicht nötig.

| System | Datei | Voraussetzung |
|---|---|---|
| Windows 10/11 (64 Bit) | `mundwerk-windows-x86_64.exe` | – |
| Linux x86_64 | `mundwerk-linux-x86_64` | glibc 2.35 oder neuer (z. B. Ubuntu 22.04), `libasound2`, `libvulkan1` |
| macOS mit Apple-Chip (M1 und neuer) | `mundwerk-macos-arm64.zip` | macOS 13 Ventura oder neuer |

Unter Windows und Linux muss der Prozessor AVX und F16C können: Intel ab Ivy Bridge, AMD ab Piledriver bzw. Jaguar, also etwa ab Baujahr 2012.

**Windows:** Die `.exe` ist nicht signiert. SmartScreen warnt beim ersten Start; „Weitere Informationen“ › „Trotzdem ausführen“ startet sie.

**Linux:** Vor dem ersten Start ausführbar machen:

```sh
chmod +x mundwerk-linux-x86_64
./mundwerk-linux-x86_64
```

**Mac:** siehe den nächsten Abschnitt.

## Mac

Mundwerk läuft auf Macs mit Apple-Chip (M1, M2, M3, M4 …). Welchen Chip dein Mac hat, steht im Apple-Menü (Apfel oben links) › „Über diesen Mac“ in der Zeile „Chip“. Steht dort „Prozessor … Intel“, läuft Mundwerk auf diesem Mac nicht.

### Herunterladen und auspacken

1. Unter [Releases](../../releases) beim neuesten Release `mundwerk-macos-arm64.zip` herunterladen.
2. Im Finder unter „Downloads“ die ZIP-Datei doppelklicken. Daneben erscheint **Mundwerk** (`Mundwerk.app`).
3. **Mundwerk** in den Ordner „Programme“ ziehen. Das ist keine Installation, die App ist nur diese eine Datei. Aus „Programme“ heraus merkt sich macOS die Freigaben aus den nächsten Schritten zuverlässig.

### Erster Start: „Trotzdem öffnen“

Mundwerk ist nicht bei Apple registriert (das kostet 99 $ im Jahr). Deshalb blockiert macOS den ersten Start, einmalig:

1. **Mundwerk** in „Programme“ doppelklicken. macOS meldet „„Mundwerk“ wurde nicht geöffnet“ (ältere macOS-Versionen: „kann nicht geöffnet werden, da Apple darin nicht nach Schadsoftware suchen kann“). Auf **Fertig** bzw. **OK** klicken, *nicht* auf „In den Papierkorb legen“.
2. Apple-Menü › **Systemeinstellungen** › **Datenschutz & Sicherheit** öffnen und ganz nach unten scrollen bis zum Abschnitt „Sicherheit“.
3. Dort steht „„Mundwerk“ wurde blockiert …“. Auf **Trotzdem öffnen** klicken und mit Passwort oder Touch ID bestätigen.
4. In der folgenden Rückfrage noch einmal **Trotzdem öffnen** wählen.

Ab jetzt startet Mundwerk normal per Doppelklick. Nach einem Update auf ein neues Release fragt macOS noch einmal, dann die Schritte wiederholen.

Unter macOS 13 und 14 geht es auch kürzer: Mundwerk mit **Rechtsklick** (oder Ctrl-Klick) › **Öffnen** › **Öffnen**. Ab macOS 15 gibt es diesen Weg nicht mehr.

**Meldet macOS „Mundwerk ist beschädigt“** oder fehlt der Knopf „Trotzdem öffnen“, hilft das Terminal (Programme › Dienstprogramme › Terminal). Diesen Befehl eingeben und Enter drücken:

```sh
xattr -dr com.apple.quarantine /Applications/Mundwerk.app
```

Er entfernt nur die Download-Markierung, die macOS an die App geheftet hat. Danach startet Mundwerk per Doppelklick.

### Mikrofon freigeben

Bei der ersten Aufnahme fragt macOS: „„Mundwerk“ möchte auf das Mikrofon zugreifen.“ Auf **Erlauben** klicken.

Wurde das versehentlich abgelehnt, meldet Mundwerk „Kein Ton aufgenommen“. Dann unter Apple-Menü › **Systemeinstellungen** › **Datenschutz & Sicherheit** › **Mikrofon** den Schalter bei **Mundwerk** einschalten und Mundwerk neu starten.

### Auf dem Mac anders

- Tastenkürzel mit **⌘** statt Strg: `⌘R` startet und stoppt die Aufnahme, `⌘⇧C` kopiert den Text. Das Kürzel für alle Fenster ist `⌃⌥Leertaste` (Control, Option, Leertaste), eingefügt wird mit `⌘V`.
- Die Grafikeinheit des Apple-Chips wird über Metal genutzt. Die Fußzeile zeigt sie mit „GPU – …“.
- Die Daten liegen im Ordner `.mundwerk` in deinem Benutzerordner. Der Finder blendet Ordner mit Punkt aus; `⌘⇧.` im Finder zeigt sie.
- Zum Deinstallieren **Mundwerk** aus „Programme“ in den Papierkorb ziehen und den Ordner `.mundwerk` löschen.

## Erster Start

1. Modell auswählen – voreingestellt ist das empfohlene `large-v3-turbo-q5_0`.
2. „Herunterladen (0,5 GB)“ klicken. Vorher holt Mundwerk einmalig den Sprachfilter (0,9 MB, siehe [Modelle](#modelle)). Der Download lässt sich mit „Abbrechen“ stoppen.
3. Sobald „✓ geladen“ erscheint: „Aufnahme starten“, sprechen, „Aufnahme stoppen“.

Ab dem nächsten Start lädt Mundwerk das Modell selbst.

## Bedienung

| Taste | Wirkung |
|---|---|
| `Strg+Alt+Leertaste` (Mac: `⌃⌥Leertaste`) | Aufnahme starten / stoppen – aus **jedem** Fenster |
| `Strg+R` (Mac: `⌘R`) | Aufnahme starten / stoppen, wenn Mundwerk im Vordergrund ist |
| `Strg+Shift+C` (Mac: `⌘⇧C`) | ganzen Text in die Zwischenablage kopieren, wenn Mundwerk im Vordergrund ist |

### In anderen Programmen diktieren

Mit **„Automatisch kopieren“** (Haken unter dem Textfeld, von Anfang an gesetzt) landet nach jedem Diktat der ganze Text aus dem Textfeld in der Zwischenablage. Zusammen mit dem Kürzel `Strg+Alt+Leertaste` diktierst du so in jedes Programm, ohne das Fenster zu wechseln:

1. Im Zielprogramm an die gewünschte Stelle klicken, z. B. ins Eingabefeld in VS Code.
2. `Strg+Alt+Leertaste` drücken und sprechen. Mundwerk kann dabei im Hintergrund liegen; die Taskleiste zeigt „● Aufnahme – Mundwerk“.
3. `Strg+Alt+Leertaste` noch einmal drücken oder einfach still sein – nach 4 Sekunden stoppt die Aufnahme von selbst.
4. Sobald der Text erkannt ist, mit `Strg+V` einfügen.

Startest du per Tastenkürzel, leert Mundwerk vorher das Textfeld – der letzte Text ist dann ja meist schon eingefügt. So steht jedes Diktat für sich, und in der Zwischenablage landet nur das neue. „Rückgängig“ holt den vorigen Text zurück. Startest du im Fenster (Button oder `Strg+R`), bleibt der Text stehen und das Diktat kommt an die Cursorposition.

Weil Mundwerk dabei meist verdeckt ist, meldet es sich mit leisen, kurzen Tönen:

| Ton | Wann |
|---|---|
| zwei Töne aufsteigend | Aufnahme per Tastenkürzel gestartet |
| zwei Töne absteigend | Aufnahme per Tastenkürzel oder wegen Stille gestoppt |
| zwei kurze gleiche, hohe Töne | Text erkannt und in der Zwischenablage – jetzt `Strg+V` (nur mit gesetztem Haken) |

Was du im Mundwerk-Fenster selbst startest und stoppst (Button, `Strg+R`), bleibt stumm – das siehst du ja.

Der Haken überschreibt, was vorher in der Zwischenablage lag. Ohne Haken kopierst du wie bisher selbst. Die Wahl gilt auch beim nächsten Start.

Das Kürzel wirkt unter Windows, auf dem Mac und unter Linux mit X11. Unter Linux mit Wayland lässt das System kein Kürzel zu, das aus anderen Fenstern heraus wirkt, und unter WSL erreicht es keine Windows-Programme – dort gilt nur `Strg+R` im Mundwerk-Fenster. Ist das Kürzel schon von einem anderen Programm belegt (oder läuft Mundwerk zweimal), steht das im Protokoll, und der Aufnahme-Button nennt `Strg+R` statt `Strg+Alt+Leertaste`.

- Die Aufnahme stoppt von selbst, wenn du 4 Sekunden lang nichts sagst. Kommt nach dem Start gar keine Sprache, stoppt sie nach 15 Sekunden. In den letzten 3 Sekunden zählt die Zeile unter dem Button herunter („Stille, stoppt in 3 s“). Was bis dahin gesprochen wurde, wird ganz normal erkannt. Weiterreden nach dem Stopp: einfach neu starten, der Text kommt an die Cursorposition.
- Neuer Text wird an der Cursorposition eingefügt. Ohne gesetzten Cursor kommt er ans Ende.
- „Leeren“ löscht den ganzen Text. „Rückgängig“ holt ihn zurück, bis du im Textfeld etwas änderst; was du inzwischen diktiert hast, bleibt dahinter stehen.
- Die Sprache stellst du rechts neben dem Aufnahme-Button ein: „Deutsch“, „Englisch“ oder „Automatisch“.
- Mundwerk startet dunkel, unabhängig von der Systemeinstellung. Rechts in der Fußzeile schaltest du mit ☀ auf hell und mit 🌙 (ganz rechts) zurück auf dunkel; die Wahl gilt auch beim nächsten Start.
- Whisper erfindet bei Stille gern kurze Sätze wie „Vielen Dank.“ oder „Ja.“. Mundwerk gibt deshalb nur den Teil der Aufnahme weiter, in dem gesprochen wurde, und lässt Stille davor und danach weg. Ob gesprochen wird, erkennt der Sprachfilter (Silero-VAD) am Klang, nicht an der Lautstärke: Leise eingestellte Mikrofone funktionieren, Lüfter- und Raumrauschen zählen nicht als Sprache. Wie viel Sprache er gefunden hat, steht nach jeder Aufnahme im Protokoll („Sprache laut VAD: …“). Findet Mundwerk gar keine Sprache, geht vorsichtshalber die ganze Aufnahme an Whisper – dann kann doch einmal ein erfundener Satz kommen, aber kein Diktat geht verloren.
- Fehlt der Sprachfilter (etwa weil der Download scheiterte), entscheidet Mundwerk nach der Lautstärke und schreibt das ins Protokoll. Als Sprache zählt dann, was deutlich lauter ist als die leiseste Stelle der Aufnahme; Klopfen und Tippen zählen mit, und bei leisen Mikrofonen kann der Auto-Stopp erst nach 15 Sekunden kommen. Beim nächsten Laden eines Modells versucht Mundwerk den Download erneut.

## Modelle

| Modell | Größe | Hinweis |
|---|---|---|
| `large-v3-turbo-q5_0` | 0,5 GB | Empfohlen |
| `large-v3-turbo` | 1,6 GB | Genauer, langsamer, braucht mehr Speicher |
| `base` | 0,15 GB | Am schnellsten, ungenauer |

In der Auswahl steht ● „vorhanden“ für heruntergeladene Modelle und ○ „noch nicht heruntergeladen“ für die übrigen. Ein vorhandenes Modell wird beim Auswählen sofort geladen. Unter der Auswahl stehen die Download-Adresse und der Speicherort des gewählten Modells.

Geladen wird von `https://huggingface.co/ggerganov/whisper.cpp`. Dazu kommt einmalig der Sprachfilter `ggml-silero-v5.1.2.bin` (0,9 MB) von `https://huggingface.co/ggml-org/whisper-vad`. Er erkennt, wann gesprochen wird, für den Auto-Stopp und den Zuschnitt der Aufnahme. Speicherort und Quelle stehen in der Zeile „Sprachfilter“ unter der Modellauswahl.

Nach jedem Download vergleicht Mundwerk die SHA-256-Prüfsumme mit der, die Hugging Face veröffentlicht, und verwirft die Datei, wenn sie nicht stimmt.

## GPU oder CPU

Mundwerk nutzt automatisch eine vorhandene Grafikkarte und rechnet sonst auf der CPU: unter Windows und Linux über Vulkan (NVIDIA, AMD, Intel), auf dem Mac über Metal. Die Fußzeile „Berechnung läuft auf:“ zeigt, was genutzt wird, und bei CPU den Grund:

- „keine Vulkan-fähige GPU gefunden“ – Grafiktreiber aktualisieren. Unter Windows bringt der normale Treiber Vulkan mit, unter Linux muss `libvulkan1` samt Treiber installiert sein.
- „ohne GPU-Unterstützung gebaut“ – ein selbst gebautes Binary mit `--no-default-features`.

Auf der CPU dauert die Erkennung ein Vielfaches der Aufnahme: Auf einem i5-3570K brauchten 9 s Sprache mit `large-v3-turbo-q5_0` 65–90 s. Mit `base` geht es deutlich schneller.

## Dateien

Alles liegt in einem Ordner: `%USERPROFILE%\.mundwerk\` unter Windows, `~/.mundwerk/` unter Linux und macOS.

- `models/` – die heruntergeladenen Modelle und der Sprachfilter
- `mundwerk.log` – das Protokoll, dieselben Zeilen wie unter „Protokoll“ im Fenster, mit Sekunden seit dem Programmstart
- `theme` – `light`, wenn du auf hell umgeschaltet hast

Zum Deinstallieren die Programmdatei und diesen Ordner löschen.

## Wenn etwas nicht klappt

Fehler stehen rot mit ⚠ unter dem Aufnahme-Button und zusätzlich im Protokoll. Die Version steht rechts in der Fußzeile („Version 0.2.0“; selbst gebaut: „Version dev“).

**„Kein Ton aufgenommen“** – Das Mikrofon liefert nur Stille. Unter Windows: `Einstellungen > Datenschutz und Sicherheit > Mikrofon`, dort „Mikrofonzugriff“ und „Desktop-Apps den Zugriff auf Ihr Mikrofon erlauben“ einschalten. Auf dem Mac: siehe „Mikrofon freigeben“ im Abschnitt Mac. Mundwerk nimmt über das Standard-Aufnahmegerät des Systems auf; welches das ist, steht im Protokoll („Aufnahme über Mikrofon: …“).

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

Auf dem Mac (Apple-Chip) braucht es nur Rust und die Xcode Command Line Tools (`xcode-select --install`), dann `cargo build --release --no-default-features --features metal`. `scripts/macos-app.sh target/release/mundwerk dev Mundwerk.zip` packt das Binary in eine `Mundwerk.app`.

Unter Windows überschreitet der Build von whisper.cpp unter `.\target` die Pfadgrenze von 260 Zeichen. Mit einem kurzen Build-Verzeichnis geht es, z. B. `set CARGO_TARGET_DIR=C:\t` vor `cargo build --release`.

## Releases

Jeder Push auf `main`, der Code ändert (`src/`, `scripts/`, `Cargo.toml`, `Cargo.lock`, die CI-Konfiguration), wird für Linux, Windows und macOS gebaut und getestet. Enthält er seit dem letzten Release einen Commit `feat:` oder `fix:` nach [Conventional Commits](https://www.conventionalcommits.org/), erzeugt semantic-release den Tag `vX.Y.Z` und ein GitHub-Release mit allen drei Dateien. `chore:`, `docs:`, `ci:`, `refactor:` und `test:` lösen kein Release aus; Änderungen nur an README oder Doku starten gar keinen Build.

## Lizenz

MIT. Whisper-Modelle: MIT (OpenAI), whisper.cpp: MIT.
