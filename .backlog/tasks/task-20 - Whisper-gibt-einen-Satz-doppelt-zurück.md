---
id: TASK-20
title: Whisper gibt einen Satz doppelt zurück
status: Done
assignee:
  - '@claude'
created_date: '2026-11-05 19:13'
updated_date: '2026-11-05 19:38'
labels: []
dependencies: []
priority: high
type: bug
ordinal: 20000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Im Live-Test zu TASK-19 (WSL, `large-v3-turbo-q5_0`, CPU) kam für einen 9,1-s-Ausschnitt mit einem einzigen gesprochenen Satz (`espeak-ng`) der Text doppelt zurück: „Guten Morgen. Ich diktiere hier … Danach kommt eine Pause. Guten Morgen. Ich diktiere hier … Danach kommt eine Pause.“ In der Aufnahme kommt der Satz nachweislich nur einmal vor. Verdacht, ungeprüft: `p.set_audio_ctx(...)` in `transcribe` verkleinert das Audio-Fenster passend zur Aufnahmelänge (damals gemessen 126 s → 17 s für 5 s Audio auf dem i5-3570K); verkleinerte Audio-Fenster sind für Wiederholungen bekannt.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Der Fehler ist mit einer Aufnahme reproduziert und die Ursache gemessen (mit und ohne verkleinertes Audio-Fenster)
- [x] #2 Ein diktierter Satz erscheint genau einmal
- [x] #3 Die Erkennungszeit auf der CPU ist vorher und nachher gemessen und im Task festgehalten
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Probe: gleiche Aufnahme mit verschiedenen `audio_ctx`
2. Nach Entscheidung des Nutzers (volles Fenster): `set_audio_ctx` entfernen, README-Zeitangabe anpassen
3. Live unter WSL wiederholen
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Reproduziert (Probe-Test, danach entfernt; i5-3570K, CPU, `large-v3-turbo-q5_0`, `espeak-ng`-Satz mit 0,3 s Rand davor und 0,5 s danach = 9,4 s):
- `audio_ctx` 534 (heutige Formel, Länge + 64): 19,9 s, **Satz doppelt**
- `audio_ctx` 768: 28,8 s, korrekt
- `audio_ctx` 1024: 37,9 s, korrekt
- voll (1500, nicht gesetzt): 65,1 s, korrekt
- 4 s Ausschnitt mit `audio_ctx` 264: 10,2 s, kein Doppel (Satz endet mitten drin, wie das Audio)
Ursache ist das zu knappe Audio-Fenster. Nur eine Aufnahme gemessen, eine sichere Grenze lässt sich daraus nicht ableiten. GPU-Zeiten mit vollem Fenster nicht gemessen. Entscheidung beim Nutzer: volles Fenster oder großzügigere Formel.

Nutzer hat das volle Fenster gewählt. `set_audio_ctx` entfernt, ein Kommentar an der Stelle hält fest, warum. Live unter WSL über `RDPSink.monitor`: „Text erkannt: 9,0 s Audio in 92,4 s“, Satz genau einmal. Davor in der App 74,6 s mit doppeltem Satz. Kein Regressionstest: Er bräuchte das 0,5-GB-Modell, das in CI nicht liegt, und Minuten an CPU-Zeit.

GPU-Zeit mit vollem Fenster, vom Nutzer unter Windows mit `v0.10.0` gemessen (GTX 1070): 2,5 s Audio in 1,7 s. Vorher mit verkleinertem Fenster: 3,6 s Audio in 0,6 s (`v0.9.1`).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Ursache war das an die Aufnahmelänge angepasste Audio-Fenster (`set_audio_ctx`): Damit gab Whisper einen 9-s-Satz doppelt zurück, mit Fenstern ab 768 oder dem vollen Fenster nicht. Auf Wunsch des Nutzers bekommt Whisper wieder das volle 30-s-Fenster. Live geprüft: Der Satz erscheint einmal. Preis: auf der CPU etwa 3× langsamer (9 s Sprache 65–90 s statt 20 s), README angepasst. Die GPU-Zeit unter Windows ist nicht gemessen und sollte beim nächsten Test des Nutzers mit notiert werden.
<!-- SECTION:FINAL_SUMMARY:END -->
