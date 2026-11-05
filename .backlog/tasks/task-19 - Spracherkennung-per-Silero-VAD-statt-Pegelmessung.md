---
id: TASK-19
title: Spracherkennung per Silero-VAD statt Pegelmessung
status: Done
assignee:
  - '@claude'
created_date: '2026-11-05 17:41'
updated_date: '2026-11-05 19:38'
labels: []
dependencies: []
references:
  - 'https://huggingface.co/ggml-org/whisper-vad'
priority: high
type: enhancement
ordinal: 19000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Auto-Stopp (4 s nach dem letzten Wort) und der Zuschnitt auf den gesprochenen Teil entscheiden bisher über Pegel: Sprache ist, was um `SPEECH_OVER_FLOOR` (×4) über dem leisesten 100-ms-Fenster liegt. Auf dem Windows-Mikrofon des Nutzers liegt Sprache nur 2,3-mal darüber („leisester 0.0077, lautester 0.0178“), WSL-Raumrauschen ohne Sprache 2,1-mal – ein Faktor trennt beides nicht (TASK-18). Unter Windows greift der Zuschnitt deshalb nicht, und der Auto-Stopp sieht vermutlich nie Sprache. Silero-VAD erkennt Sprache am Klang statt an der Lautstärke. whisper.cpp bringt es mit, `whisper-rs` 0.16 bietet `WhisperVadContext`; das Modell liegt auf Hugging Face unter `ggml-org/whisper-vad` als `ggml-silero-v5.1.2.bin` (885098 Bytes, SHA-256 `29940d98d42b91fbd05ce489f3ecf7c72f0a42f027e4875919a28fb4c04ea2cf`, per `x-linked-etag` geprüft). Was bleiben soll: kein Verwerfen von Aufnahmen ohne erkannte Sprache, Countdown, Töne, Pegelanzeige.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Mundwerk lädt das VAD-Modell nach `~/.mundwerk/models/`, mit Prüfsummen-Vergleich wie beim Whisper-Modell; Quelle und Ziel sind sichtbar, der Download steht im Protokoll
- [x] #2 Der Auto-Stopp entscheidet per VAD: 4 s nach der letzten erkannten Sprache, 15 s ohne Sprache
- [x] #3 An Whisper geht nur der per VAD erkannte gesprochene Teil; ohne erkannte Sprache weiterhin die ganze Aufnahme
- [x] #4 Fehlt das VAD-Modell oder schlägt es fehl, arbeitet Mundwerk mit der bisherigen Pegelmethode weiter und protokolliert das
- [x] #5 Geprüft mit den Aufnahmen aus TASK-12/17 (Sprachausgabe, Raum- und Lüfterrauschen) und vom Nutzer unter Windows: Auto-Stopp 4 s nach dem letzten Wort
- [x] #6 README beschreibt die Spracherkennung und das zweite Modell
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. `download` nimmt URL statt Modellname, damit Whisper- und VAD-Modell denselben Weg mit Prüfsumme gehen
2. `load_model` lädt beim ersten Mal auch das VAD-Modell (`ggml-silero-v5.1.2.bin` aus `ggml-org/whisper-vad`) nach `~/.mundwerk/models/`; Fehler dort nur ins Protokoll, dann Pegelmethode
3. Sprache je Fenster als `Vec<bool>`: aus Pegeln (100 ms) oder aus VAD-Wahrscheinlichkeiten (512 Samples = 32 ms bei 16 kHz); `secs_until_auto_stop` und `speech_span` rechnen auf dieser Liste
4. Während der Aufnahme: neue Abschnitte (ab 0,5 s) resamplen und per VAD auswerten, Ergebnis an die Liste der Aufnahme hängen
5. Beim Stopp: VAD über die ganze Aufnahme für den Zuschnitt
6. Zeile im Modellbereich zeigt Ziel und Quelle des VAD-Modells
7. Messen mit `espeak-ng`-Sprache, Raum- und Lüfterrauschen (Probe-Test), live unter WSL; README
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Vom Nutzer unter Windows mit `v0.9.1` bestätigt: Der Auto-Stopp kommt erst 15 s nach dem Start, nicht 4 s nach dem letzten Wort. Die Pegelmethode erkennt auf diesem Mikrofon also keine Sprache, die Diagnose aus TASK-18 stimmt.

Umgesetzt wie geplant. Messungen auf dem i5-3570K (Probe-Test, danach entfernt) mit `espeak-ng`-Sprache, WSL-Raumrauschen (RMS 0,016) und rosa Rauschen:
- Sprache mit 1 s Pause davor und 8 s danach: Sprache 1,0–9,3 s erkannt, Auto-Stopp bei 13,4 s, Zuschnitt 0,72–9,8 s. Mit Lüfterrauschen RMS 0,01 und als leise Sprache über Rauschen (wie das Windows-Mikrofon des Nutzers) praktisch gleich.
- Raumrauschen 9,5 s, Lüfterrauschen 20 s, digitale Stille: keine Sprache, Auto-Stopp nach 15 s.
- Bei 44,1 kHz dasselbe Ergebnis wie bei 16 kHz.
- Schrittweise Auswertung (0,5 s, 16 Frames Vorlauf) gegenüber einem Durchlauf über alles: höchstens 10 von 551 Frames anders.
- `n_threads` muss 1 sein: 2 s Audio in 23 ms mit 1 Thread, 75 ms mit 2, 3,9 s mit 4. Mit 1 Thread höchstens 13 ms je Schritt im UI-Thread.
Live unter WSL: VAD-Modell geladen, Prüfsumme stimmt. Mit `pactl set-default-source RDPSink.monitor` (das WSL-Mikrofon hört den Lautsprecher nicht) Auto-Stopp 4,1 s nach Ende der Sprachausgabe, „Sprache laut VAD: 6,8 s von 13,6 s“, 9,1 s an Whisper. Ohne Sprache: Stopp nach 15 s, ganze Aufnahme an Whisper. Kaputte VAD-Datei: „Spracherkennung per VAD nicht verfügbar, Mundwerk erkennt Sprache am Pegel: …“, Mundwerk wird trotzdem bereit. Nebenbei: Downloads unter 10 MB schreiben keine Prozentzeilen mehr ins Protokoll.
Beobachtet, nicht Teil dieses Tasks: Whisper gab den 9,1-s-Ausschnitt mit dem Satz doppelt zurück, obwohl er in der Aufnahme nur einmal vorkommt. Offen bleibt AC #5: die Bestätigung des Nutzers unter Windows.

Vom Nutzer unter Windows mit `v0.10.0` bestätigt (GTX 1070): „Aufnahme nach 4 s Stille automatisch beendet“, „Sprache laut VAD: 1,7 s von 6,5 s“, „Text erkannt: 2,5 s Audio in 1,7 s“.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Ob gesprochen wird, entscheidet jetzt Silero-VAD (`ggml-silero-v5.1.2.bin`, 0,9 MB) statt der Pegelmessung, für den Auto-Stopp und für den Zuschnitt der Aufnahme. Das Modell wird mit Prüfsumme nach `~/.mundwerk/models/` geladen, die Zeile „Sprachfilter“ zeigt Ziel und Quelle. Fehlt es oder scheitert es, nutzt Mundwerk die Pegelmethode und protokolliert das. Das VAD läuft schrittweise alle 0,5 s im UI-Thread mit einem Thread (höchstens 13 ms auf dem i5-3570K, mit 4 Threads wäre es über 150-mal langsamer). Geprüft mit Testaufnahmen, live unter WSL und vom Nutzer unter Windows: Der Auto-Stopp kommt 4 s nach dem letzten Wort, auch auf seinem leisen Mikrofon. Ausgeliefert in `v0.10.0`.
<!-- SECTION:FINAL_SUMMARY:END -->
