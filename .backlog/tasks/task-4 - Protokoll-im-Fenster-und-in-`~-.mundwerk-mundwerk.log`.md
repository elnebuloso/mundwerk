---
id: TASK-4
title: Protokoll im Fenster und in `~/.mundwerk/mundwerk.log`
status: Done
assignee:
  - '@claude'
created_date: '2026-11-04 20:48'
updated_date: '2026-11-04 22:55'
labels: []
dependencies:
  - TASK-2
priority: medium
type: feature
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der Nutzer soll das Verhalten der App nachvollziehen können (Ordner anlegen, Download, Speichern, Modell laden, Rechenwerk). Die Datei hilft bei Fehlerberichten vom Windows-Rechner, auf dem kein stderr sichtbar ist.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Ein scrollbarer Bereich im Fenster zeigt jede Protokollzeile mit Zeitangabe
- [x] #2 Dieselben Zeilen werden an `~/.mundwerk/mundwerk.log` angehängt
- [x] #3 Download protokolliert URL, Fortschritt und Speicherort
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. `Msg::Log` statt teilweise `Msg::Status`; UI-Thread hängt Zeile an Liste und Datei an
2. Zeitangabe als Sekunden seit Start (keine Zeitzonen-Abhängigkeit nötig)
3. Unteres Panel mit `ScrollArea`
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Zeitangabe: Sekunden seit Start (kein Zeitzonen-Crate). Datei geprüft: Startzeilen landen in `~/.mundwerk/mundwerk.log`. Das Panel im Fenster ist noch nicht angesehen.

Vom Nutzer im Fenster gesehen: Prüfsumme, Speicherort, Laden, Transkriptionsdauer.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Protokoll im einklappbaren Bereich „Protokoll“ und in `~/.mundwerk/mundwerk.log` (Sekunden seit Start). Download, Prüfsumme, Speicherort, Laden und Texterkennung werden protokolliert. Unter WSL geprüft.
<!-- SECTION:FINAL_SUMMARY:END -->
