---
id: TASK-22
title: Zuletzt gewählte Sprache beim Start wieder wählen
status: Done
assignee:
  - '@claude'
created_date: '2026-11-05 20:03'
updated_date: '2026-11-05 20:04'
labels: []
dependencies: []
priority: medium
type: enhancement
ordinal: 22000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Die Sprachauswahl (Deutsch, Englisch, Automatisch) steht nach jedem Neustart wieder auf Deutsch. Sie soll gespeichert werden wie das Modell in TASK-21, in `~/.mundwerk/language`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Nach einem Neustart ist die zuletzt gewählte Sprache eingestellt
- [x] #2 Fehlt die Datei oder steht ein unbekannter Wert darin, gilt Deutsch
- [x] #3 README nennt die neue Datei im Abschnitt „Dateien“
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Datei `~/.mundwerk/language` mit dem Sprachcode (`de`, `en`, `auto`), `lang_index` wie `model_index`
2. Gespeichert wird bei jeder Änderung in der Auswahl, beim Start eingelesen
3. Unit-Test, live unter WSL, README
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Umgesetzt wie geplant, Datei `~/.mundwerk/language`. Unit-Test `saved_language_code_selects_its_index` (bekannte Codes mit Zeilenumbruch, leer, unbekannt). Live unter WSL mit eigenem `HOME` im Scratchpad: „Englisch“ gewählt, gespeichert `en`, nach Neustart steht „Englisch“ in der Auswahl.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Mundwerk merkt sich die zuletzt gewählte Sprache in `~/.mundwerk/language` (`de`, `en` oder `auto`) und stellt sie beim nächsten Start wieder ein. Ohne Datei oder bei unbekanntem Wert gilt Deutsch. Unit-Test und Live-Test unter WSL, README ergänzt.
<!-- SECTION:FINAL_SUMMARY:END -->
