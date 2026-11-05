---
id: TASK-21
title: Zuletzt verwendetes Modell beim Start wieder wählen
status: Done
assignee:
  - '@claude'
created_date: '2026-11-05 19:59'
updated_date: '2026-11-05 20:00'
labels: []
dependencies: []
priority: medium
type: enhancement
ordinal: 21000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Wechselt der Nutzer das Modell, schließt Mundwerk und startet neu, ist wieder das erste Modell (`large-v3-turbo-q5_0`) gewählt statt des zuletzt verwendeten. Die Wahl soll gespeichert werden wie schon `theme` und `auto-copy` in `~/.mundwerk/`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Nach einem Neustart ist das zuletzt geladene Modell gewählt und wird geladen, falls vorhanden
- [x] #2 Ein ausgewähltes, aber nie heruntergeladenes Modell überschreibt die gespeicherte Wahl nicht
- [x] #3 Fehlt die Datei oder steht ein unbekannter Name darin, gilt das erste Modell
- [x] #4 README nennt die neue Datei im Abschnitt „Dateien“
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Datei `~/.mundwerk/model` nach dem Muster von `auto-copy`: `load_model_choice` liefert den Index in `MODELS`, unbekannt oder fehlend ergibt 0
2. Gespeichert wird bei `Msg::Loaded`, also erst wenn das Modell wirklich verwendet wird, nicht schon bei der Auswahl
3. `App::new` startet mit dem gespeicherten Modell und lädt es, wenn es vorhanden ist
4. Unit-Test für das Einlesen, live unter WSL prüfen, README
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Umgesetzt wie geplant, Datei `~/.mundwerk/model`. Unit-Test `saved_model_name_selects_its_index` deckt bekannte Namen mit Zeilenumbruch, leere Datei und unbekannten Namen ab. Live unter WSL mit eigenem `HOME` im Scratchpad: Beim ersten Start ohne Datei wird `large-v3-turbo-q5_0` geladen und gespeichert. `base` heruntergeladen (Prüfsumme stimmt) und geladen, gespeichert ist `base`. Danach `large-v3-turbo` (nicht vorhanden) ausgewählt, gespeichert bleibt `base`. Neustart: „Lade base in den Speicher …“, „Bereit: base“, `base` in der Auswahl. Nebenbei im README die fehlende Datei `auto-copy` ergänzt (eigener Commit).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Mundwerk merkt sich das zuletzt geladene Modell in `~/.mundwerk/model` und wählt und lädt es beim nächsten Start wieder. Gespeichert wird erst beim Laden, eine bloße Auswahl eines nicht heruntergeladenen Modells zählt nicht. Ohne Datei oder bei unbekanntem Namen gilt das erste Modell. Unit-Test und Live-Test unter WSL. Die Sprachauswahl (Deutsch/Englisch/Automatisch) wird weiterhin nicht gespeichert.
<!-- SECTION:FINAL_SUMMARY:END -->
