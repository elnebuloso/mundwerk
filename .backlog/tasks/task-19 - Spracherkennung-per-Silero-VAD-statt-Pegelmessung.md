---
id: TASK-19
title: Spracherkennung per Silero-VAD statt Pegelmessung
status: To Do
assignee: []
created_date: '2026-11-05 17:41'
updated_date: '2026-11-05 18:53'
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
- [ ] #1 Mundwerk lädt das VAD-Modell nach `~/.mundwerk/models/`, mit Prüfsummen-Vergleich wie beim Whisper-Modell; Quelle und Ziel sind sichtbar, der Download steht im Protokoll
- [ ] #2 Der Auto-Stopp entscheidet per VAD: 4 s nach der letzten erkannten Sprache, 15 s ohne Sprache
- [ ] #3 An Whisper geht nur der per VAD erkannte gesprochene Teil; ohne erkannte Sprache weiterhin die ganze Aufnahme
- [ ] #4 Fehlt das VAD-Modell oder schlägt es fehl, arbeitet Mundwerk mit der bisherigen Pegelmethode weiter und protokolliert das
- [ ] #5 Geprüft mit den Aufnahmen aus TASK-12/17 (Sprachausgabe, Raum- und Lüfterrauschen) und vom Nutzer unter Windows: Auto-Stopp 4 s nach dem letzten Wort
- [ ] #6 README beschreibt die Spracherkennung und das zweite Modell
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Vom Nutzer unter Windows mit `v0.9.1` bestätigt: Der Auto-Stopp kommt erst 15 s nach dem Start, nicht 4 s nach dem letzten Wort. Die Pegelmethode erkennt auf diesem Mikrofon also keine Sprache, die Diagnose aus TASK-18 stimmt.
<!-- SECTION:NOTES:END -->
