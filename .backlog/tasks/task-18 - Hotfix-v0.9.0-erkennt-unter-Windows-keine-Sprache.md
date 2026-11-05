---
id: TASK-18
title: 'Hotfix: v0.9.0 erkennt unter Windows keine Sprache'
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-05 17:19'
updated_date: '2026-11-05 17:21'
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
- [ ] #4 Unter Windows vom Nutzer geprüft: Diktat wird erkannt, Pegelzeile liegt vor
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Regressionstest `quiet_microphone_speech_counts` (Sprache RMS 0,006 über Rauschen 0,0005) schlägt mit der alten Untergrenze 0,01 fehl. Probe mit echten Aufnahmen nach dem Fix, Entscheidungen unverändert: nur Sprache → ganz; Sprache+Stille → 0,7–16,7 s; Raum 10 s (leisester 0,0010, lautester 0,0021, Schwelle 0,0038) und Lüfter (0,0097/0,0107/0,0387) → keine Sprache. Live WSL: Pegelzeile im Protokoll, ohne erkannte Sprache „die ganze Aufnahme geht an die Texterkennung“, Text erkannt. Offen: AC 4 unter Windows.
<!-- SECTION:NOTES:END -->
