---
id: TASK-17
title: Halluzinationen und kaputte Zeichen in der Texterkennung vermeiden
status: Done
assignee:
  - '@claude'
created_date: '2026-11-05 17:04'
updated_date: '2026-11-05 17:06'
labels: []
dependencies: []
priority: high
type: bug
ordinal: 17000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Whisper erfindet bei Stille oder Rauschen kurze Sätze („Vielen Dank.“, „Ja.“), auch am Ende echter Diktate, deren Aufnahme nach 4 s Stille automatisch endet. Einmal kam „B��. Dispatch.“ heraus: Ersatzzeichen aus ungültigem UTF-8. Gemessen: `no_speech_probability()` liefert mit `large-v3-turbo-q5_0` bei Rauschen wie bei Sprache 0,000 und taugt nicht als Filter.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 An Whisper geht nur der Abschnitt vom ersten bis zum letzten erkannten Sprechen, mit kurzem Rand; Stille davor und danach nicht
- [x] #2 Enthält die Aufnahme keine erkannte Sprache, wird nicht transkribiert; Mundwerk meldet „Keine Sprache erkannt“ und fügt nichts ein oder kopiert
- [x] #3 Der erkannte Text enthält keine Ersatzzeichen `�`; über Segmentgrenzen geteilte Zeichen bleiben erhalten
- [x] #4 Unit-Tests für das Zuschneiden und das Dekodieren; README nennt die Grenze bei sehr lautem Rauschen
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. `speech_windows` aus `secs_until_auto_stop` herauslösen
2. `speech_span`: erster bis letzter Sprach-Abschnitt, Rand 0,3 s davor, 0,5 s danach; None ohne Sprache
3. `toggle`: ohne Sprache nicht transkribieren, Status „Keine Sprache erkannt – nichts eingefügt“
4. `decode_text`: Segment-Bytes zusammensetzen, einmal dekodieren, `U+FFFD` entfernen
5. Unit-Tests, Probe mit echten Aufnahmen, Live-Test, README
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Gemessen (temporärer Probe-Test mit `large-v3-turbo-q5_0`): `no_speech_probability()` = 0,000 für Raumrauschen (Ausgabe „ ...“, „ *Signal*“), Lüfterrauschen und Sprache – unbrauchbar als Filter. `speech_span` auf echten Aufnahmen: nur Sprache 6,1 s → ganz behalten; Sprache 1,0–16,5 s mit 8 s Stille danach → 0,7–16,7 s (mit und ohne Lüfterrauschen RMS 0,01); 10 s Raumrauschen und 20 s Lüfterrauschen → keine Sprache; 4 s Raum mit Klopfer → als Sprache gewertet (Grenze, ergab „*Signal*“). Live unter WSL: Aufnahme ohne Sprechen → Auto-Stopp nach 15 s, „Keine Sprache erkannt, nichts an die Texterkennung gegeben“, Zwischenablage unverändert, keine Erkennungszeit mehr. Damit kehrt sich die Entscheidung aus TASK-12 um, ohne Sprache trotzdem zu erkennen: Bei Rauschen so laut wie die Stimme kann ein Diktat verworfen werden; im README genannt. Nicht umgesetzt: Filter für Whisper-Anmerkungen wie „*Signal*“ oder „[Musik]“.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Whisper bekommt nur noch den gesprochenen Teil der Aufnahme (Stille davor/danach weg, kleiner Rand); ohne erkannte Sprache wird nichts transkribiert und nichts eingefügt. Der Text wird aus den Bytes aller Segmente einmal dekodiert, ungültige Bytes fallen weg statt „�“. Grenzen: Klopfen/Tippen zählt als Sprache; Rauschen so laut wie die Stimme kann Sprache verdecken.
<!-- SECTION:FINAL_SUMMARY:END -->
