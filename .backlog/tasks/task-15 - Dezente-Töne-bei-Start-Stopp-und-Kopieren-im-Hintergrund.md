---
id: TASK-15
title: 'Dezente Töne bei Start, Stopp und Kopieren im Hintergrund'
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-05 16:35'
updated_date: '2026-11-05 16:39'
labels: []
dependencies: []
priority: medium
type: feature
ordinal: 15000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Mundwerk liegt oft verdeckt hinter anderen Fenstern, während der Nutzer per globalem Tastenkürzel diktiert und die Aufnahme per Stille-Erkennung endet. Er will hören, was passiert, ohne hinzusehen. Vom Nutzer festgelegt: sehr dezente, kurze, voneinander unterscheidbare Töne; Start-Ton beim Start per Kürzel; Stopp-Ton beim Stopp per Kürzel und beim automatischen Stopp; ein dritter Ton, wenn der Text erkannt und kopiert ist – nur bei gesetztem Haken „Automatisch kopieren“ und nur, wenn nicht im Fenster gestoppt wurde. Bedienung im Fenster (Klick, `Strg+R`) bleibt stumm.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Start per `Strg+Alt+Leertaste` spielt einen kurzen, leisen, aufsteigenden Ton; Start per Klick oder `Strg+R` ist stumm
- [x] #2 Stopp per Kürzel oder automatisch spielt einen kurzen, leisen, absteigenden Ton; Stopp per Klick oder `Strg+R` ist stumm
- [x] #3 Nach einem Stopp per Kürzel oder automatisch und mit gesetztem Haken spielt ein dritter, unterscheidbarer Ton, sobald der Text kopiert ist; ohne Haken oder nach Stopp im Fenster nicht
- [x] #4 Die Töne sind leise, ohne Knacken an Anfang und Ende (per Unit-Test geprüft); ohne Ausgabegerät läuft Mundwerk normal weiter und protokolliert das
- [x] #5 README beschreibt die Töne
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. `cue_samples(cue, rate)`: Sinustöne mit weicher Hüllkurve, Amplitude ≤ 0,1; Start 660→880 Hz, Stopp 880→660 Hz, Kopiert zweimal 1175 Hz
2. `play_cue`: Thread, cpal-Ausgabe auf dem Standardgerät, Formate wie bei der Aufnahme generisch
3. `toggle` bekommt den Auslöser (Fenster, Kürzel, Auto); Merker `cue_on_copy` für den Kopier-Ton
4. Unit-Tests: Lautstärke, kein Knacken, Start steigt / Stopp fällt
5. Live: Ausgabe unter WSL über den PulseAudio-Monitor mitschneiden, README
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Live unter WSL geprüft, Ausgabe mit `parec -d RDPSink.monitor` mitgeschnitten (Frequenz über Nulldurchgänge je 20 ms): Start per Kürzel → 660→880 Hz; Stopp per Kürzel → 880→660 Hz; Text kopiert → 2× ~1175 Hz; RMS etwa 0,035 (−29 dBFS). Start per Kürzel und automatischer Stopp nach 15 s → Start- und Stopp-Ton im Abstand von 15,7 s, Kopier-Ton nach der Erkennung. Start und Stopp per Klick: Mitschnitt über 16,8 s komplett still, auch nach dem Kopieren. Unit-Tests: Spitze ≤ 0,05, kein Sprung am Anfang/Ende, Start steigt, Stopp fällt. Fehlerpfad ohne Ausgabegerät nur per Code (Protokollzeile „Ton konnte nicht abgespielt werden“), nicht ausgelöst.
<!-- SECTION:NOTES:END -->
