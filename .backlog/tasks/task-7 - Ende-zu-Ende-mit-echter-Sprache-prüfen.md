---
id: TASK-7
title: Ende-zu-Ende mit echter Sprache prüfen
status: Done
assignee:
  - '@claude'
created_date: '2026-11-04 20:48'
updated_date: '2026-11-04 21:06'
labels: []
dependencies:
  - TASK-3
  - TASK-4
  - TASK-6
priority: high
type: task
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Bisher hat niemand einen erfolgreich transkribierten Text gesehen. Greedy-Decoding und `set_audio_ctx` sind nur mit Stille gemessen, nicht auf Qualität geprüft.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Ein gesprochener deutscher Satz erscheint korrekt im Textfeld
- [x] #2 Dauer und Qualität sind in den Notizen festgehalten
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
WSL, CPU (i5-3570K), `large-v3-turbo-q5_0`, Greedy + `set_audio_ctx`: „Test, Test, Test, Test, das ist ein Test, guten Abend.“ vom Nutzer als korrekt bestätigt. 3,8 s Audio in 20,8 s. Erster Versuch lieferte „Vielen Dank“ (Halluzination bei Stille): Windows hatte Desktop-Apps den Mikrofonzugriff verweigert; Nachweis per `arecord`, Spitzenwert 1 vorher, 14900 nachher. README um den Hinweis ergänzt.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Diktat funktioniert Ende-zu-Ende unter WSL auf der CPU, Text korrekt. Geschwindigkeit auf der CPU ca. 5x Echtzeit; GPU-Tempo wird mit TASK-8 unter Windows geprüft.
<!-- SECTION:FINAL_SUMMARY:END -->
