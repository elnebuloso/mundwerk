---
id: TASK-24
title: Diktate verlieren seit dem Sprachfilter ein Drittel des Tons
status: Done
assignee:
  - '@claude'
created_date: '2026-11-05 21:55'
updated_date: '2026-11-05 21:55'
labels: []
dependencies: []
priority: high
type: bug
ordinal: 24000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der Nutzer meldete, dass die Erkennung seit dem Sprachfilter (Silero-VAD, seit `v0.10.0`) unter Windows (`large-v3-turbo`, GTX 1070, Mikrofon „Nor-Tec streaming mic“) viel schlechter ist: Wörter fehlen, Sätze kommen zusammengestückelt heraus. Ein Vergleich mit sechs gleichen Texten in `v0.9.1` und `v0.13.0` zeigte, dass in `v0.13.0` deutlich weniger Ton ankommt (langer Absatz 13,7 s gegenüber 8,8 s). `detect_speech` hielt die Sperre auf dem Sample-Puffer, während der VAD lief, und der Audio-Callback in `build_stream` wartete auf dieselbe Sperre.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Unter Windows kommt ein diktierter Absatz in voller Länge bei der Texterkennung an
- [x] #2 Der erkannte Text des Absatzes entspricht wieder dem gesprochenen
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Fix: `detect_speech` bekommt den `Mutex`, kopiert das nötige Stück unter der Sperre und gibt sie vor `resample` und VAD wieder frei.
Unter WSL nicht nachstellbar: 17,1-s-Ton (`espeak-ng` über `RDPSink.monitor`) kam mit altem und neuem Code gleich lang an (21,8 s Aufnahme).
Unter Windows mit Test-`.exe` aus Entwurfs-PR #4 (danach geschlossen): langer Absatz 12,8 s statt 8,8 s, Text bis auf ein fehlendes „dir“ korrekt.
Ungeklärt: Warum der Verlust so groß war. Ein VAD-Schritt braucht unter WSL höchstens 13 ms je 0,5 s, das erklärt keine 35 %.
Kein Regressionstest: Dass Windows bei einem wartenden Audio-Callback Ton verwirft, lässt sich im Unit-Test nicht nachstellen.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Der Sprachfilter hielt während seiner Rechnung die Sperre auf dem Aufnahmepuffer, sodass Windows Ton verwarf. Jetzt kopiert er sein Stück heraus und rechnet ohne Sperre. Vom Nutzer unter Windows bestätigt: 12,8 s statt 8,8 s für denselben Absatz, Text wieder sauber.
<!-- SECTION:FINAL_SUMMARY:END -->
