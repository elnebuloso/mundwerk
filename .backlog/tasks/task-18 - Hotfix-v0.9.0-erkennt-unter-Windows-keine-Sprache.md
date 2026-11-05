---
id: TASK-18
title: 'Hotfix: v0.9.0 erkennt unter Windows keine Sprache'
status: Done
assignee:
  - '@claude'
created_date: '2026-11-05 17:19'
updated_date: '2026-11-05 17:40'
labels: []
dependencies: []
priority: high
type: bug
ordinal: 18000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Nutzer meldet: `v0.9.0` sagt unter Windows bei jedem Diktat „Keine Sprache erkannt“. Vermutete Ursache: Die Sprachschwelle hat als Untergrenze `SILENCE` (0,01), gedacht als Spitzenwert-Grenze, aber angewendet auf den RMS je 100 ms. Das WSL-Mikrofon ist sehr laut eingestellt (Spitzen ~0,45), ein normales Windows-Mikrofon liegt mit dem RMS von Sprache darunter. Pegel des Windows-Mikrofons sind nicht bekannt.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Die Untergrenze der Sprachschwelle liegt bei RMS 0,002 (−54 dBFS) statt 0,01; ein Unit-Test deckt leise Sprache ab
- [x] #2 Wird keine Sprache erkannt, geht die ganze Aufnahme an die Texterkennung (kein Verwerfen); echte Stille meldet weiter „Kein Ton aufgenommen“
- [x] #3 Jeder Stopp schreibt Grundrauschen, lautesten Abschnitt und Sprachschwelle ins Protokoll
- [x] #4 Unter Windows vom Nutzer geprüft: Diktat wird erkannt, Pegelzeile liegt vor
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Regressionstest `quiet_microphone_speech_counts` (Sprache RMS 0,006 über Rauschen 0,0005) schlägt mit der alten Untergrenze 0,01 fehl. Probe mit echten Aufnahmen nach dem Fix, Entscheidungen unverändert: nur Sprache → ganz; Sprache+Stille → 0,7–16,7 s; Raum 10 s (leisester 0,0010, lautester 0,0021, Schwelle 0,0038) und Lüfter (0,0097/0,0107/0,0387) → keine Sprache. Live WSL: Pegelzeile im Protokoll, ohne erkannte Sprache „die ganze Aufnahme geht an die Texterkennung“, Text erkannt. Offen: AC 4 unter Windows.

Windows (`v0.9.1`, Nutzer, mit Sprechen): „Pegel (RMS je 100 ms): leisester 0.0077, lautester 0.0178, Sprache ab 0.0306“ → „Keine Sprache erkannt, die ganze Aufnahme geht an die Texterkennung“ → „Text erkannt: 3,6 s Audio in 0,6 s“ (GPU). Erkennung funktioniert dank Rückfall, aber die Pegel-Spracherkennung versagt auf diesem Mikrofon: lautester Abschnitt nur 2,3× (+7 dB) über dem leisesten, Schwelle verlangt ×4. Zum Vergleich WSL-Raumrauschen ohne Sprache: 2,1×. Ein Faktor trennt beides nicht zuverlässig. Folge unter Windows: Zuschnitt auf den Sprachteil greift nicht, Auto-Stopp sieht keine Sprache und stoppt erst nach 15 s statt 4 s nach dem letzten Wort. Möglicher Ersatz: Silero-VAD von whisper.cpp (`WhisperVadContext` in whisper-rs 0.16), Modell `ggml-org/whisper-vad` `ggml-silero-v5.1.2.bin`, 885098 Bytes, SHA-256 29940d98…
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Untergrenze der Sprachschwelle von 0,01 auf RMS 0,002, ohne erkannte Sprache wird die ganze Aufnahme erkannt statt verworfen, Pegel-Zeile im Protokoll. Vom Nutzer unter Windows bestätigt: Text wird wieder erkannt (3,6 s Audio in 0,6 s). Die Pegel zeigen aber: Die pegelbasierte Spracherkennung trennt auf diesem Mikrofon Sprache nicht von Ruhe (nur +7 dB) – Auto-Stopp und Zuschnitt greifen dort nicht wie gedacht; Folgearbeit offen.
<!-- SECTION:FINAL_SUMMARY:END -->
