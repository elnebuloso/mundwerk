---
id: TASK-12
title: Aufnahme stoppt nach Stille automatisch
status: Done
assignee:
  - '@claude'
created_date: '2026-11-05 15:29'
updated_date: '2026-11-05 16:54'
labels: []
dependencies: []
priority: medium
type: feature
ordinal: 12000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der Nutzer vergisst, die Aufnahme zu stoppen. Mundwerk soll das bei Stille selbst tun. Vom Nutzer entschieden: immer aktiv ohne Einstellung, 4 s Stille nach Sprache, 15 s Frist, wenn noch gar nicht gesprochen wurde. Zu beachten: Die vorhandene Stille-Erkennung (`SILENCE`, `is_silent`) erkennt nur echte Stille. Raumrauschen liegt darüber und würde mit derselben festen Schwelle als Sprache zählen. Live prüfbar unter WSL mit Mikrofon, siehe Stolperfallen in `docs/HANDOFF.md`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Nach dem ersten erkannten Sprechen stoppt die Aufnahme nach 4 s ohne Sprache von selbst und der Text wird erkannt wie beim Klick auf „Aufnahme stoppen“
- [x] #2 Kurze Denkpausen unter 4 s beenden die Aufnahme nicht
- [x] #3 Gleichmäßiges Raumrauschen ohne Sprache hält die Aufnahme nicht offen
- [x] #4 Während der Aufnahme zeigt ein Countdown, wann sie wegen Stille stoppt; das Protokoll nennt den automatischen Stopp
- [x] #5 Die Entscheidung Sprache/Stille ist per Unit-Test abgedeckt, das README beschreibt den Auto-Stopp
- [x] #6 Wird nach dem Start 15 s keine Sprache erkannt, stoppt die Aufnahme; echte Stille meldet „Kein Ton aufgenommen“, sonst wird erkannt wie beim Klick (kein Verwerfen, falls lautes Rauschen Sprache verdeckt)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Reine Funktion `secs_until_auto_stop(samples, rate) -> (f32, bool)`: RMS je 100 ms über die letzten 30 s, Grundrauschen = leisestes Fenster ohne digitale Null, Sprache = über `max(Grundrauschen × 4, SILENCE)`; 4 s nach letzter Sprache, sonst 15 s ab Start
2. `rms` aus `level` herauslösen
3. `App::auto_stop` in jedem Frame: Countdown in der Statuszeile, bei 0 Protokollzeile und derselbe Weg wie der Klick (`toggle`)
4. Unit-Tests, Probe mit Sprachausgabe und echtem Raumrauschen, Live-Test unter WSL, README
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Abweichung vom ersten Plan: Ohne erkannte Sprache wird nach 15 s nicht verworfen, sondern normal gestoppt (Stille → „Kein Ton aufgenommen“ über `is_silent`, sonst Erkennung). Grund: Eine Probe mit Sprachausgabe (espeak-ng, Spitze 0,45, Fenster-RMS im Median 0,032) plus Rauschen auf Sprachniveau (RMS 0,03) erkannte keine Sprache – Verwerfen hätte das Diktat gelöscht. AC 2 entsprechend umformuliert. Countdown von neben dem Pegel in die Statuszeile verlegt, weil er bei 640 px die Sprachauswahl überlagerte.

Probe (Sprachausgabe + 10 s echtes Raumrauschen vom WSL-Mikrofon, RMS 0,001–0,002; einmal zusätzlich weißes Rauschen RMS 0,01): Sprache ab 1,1 s erkannt, 2,5 s Denkpause ohne Stopp, Stopp bei 20,1–20,2 s nach Sprachende 16,5 s. Reines Raumrauschen (auch mit RMS 0,01): keine Sprache, Stopp bei 15,0 s.

Live unter WSL: Aufnahme ohne Sprechen stoppte nach 15 s, Protokoll „Aufnahme nach 15 s ohne erkannte Sprache automatisch beendet“, Raumrauschen ergab „Vielen Dank.“ (bekannte Halluzination). Countdown „Aufnahme läuft … Stille, stoppt in 3 s“ per Screenshot bei 780×600 und 640×480 ohne Überlappung. Offen: AC 1 mit echter gesprochener Sprache live – der 4-s-Stopp ist nur mit Sprachausgabe offline geprüft.

Nebenbefund, nicht Teil dieses Tasks: Bei 640×480 überlappt in der Fußzeile die Rechenwerk-Zeile mit „Version dev“ (seit der Versionsanzeige).

Nutzer bestätigt am 2026-11-05 („klappt alles“) den Test von `v0.8.0` unter Windows auf dem Rechner mit GTX 1070 nach dem Ablauf: Fußzeile mit GPU, `Strg+Alt+Leertaste` aus VS Code mit Start-Ton, Auto-Stopp nach Stille mit Stopp-Ton, Kopier-Ton, `Strg+V` ins Claude-Code-Feld, auch mit minimiertem Fenster. Wörtlicher Fußzeilentext und Dauer der Erkennung wurden nicht übermittelt. Damit ist der 4-s-Stopp nach echter Sprache bestätigt.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Aufnahme stoppt 4 s nach der letzten Sprache bzw. 15 s ohne Sprache; Sprache zählt relativ zum leisesten Moment der Aufnahme (+12 dB), damit Raumrauschen sie nicht offen hält. Ohne erkannte Sprache wird nicht verworfen, sondern normal erkannt (Schutz vor Datenverlust bei lautem Rauschen). Countdown in der Statuszeile. Unit-Tests, Probe mit Sprachausgabe und Raumrauschen, Live unter WSL und vom Nutzer unter Windows bestätigt.
<!-- SECTION:FINAL_SUMMARY:END -->
