---
id: TASK-5
title: Prüfsumme nach dem Modell-Download prüfen
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-04 20:48'
updated_date: '2026-11-04 20:52'
labels: []
dependencies:
  - TASK-4
priority: medium
type: feature
ordinal: 5000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Vertrauen: Der Nutzer soll sehen, dass die geladene Datei genau die von Hugging Face veröffentlichte ist. Hugging Face liefert beim Redirect von `resolve/main` den SHA-256 im Header `x-linked-etag`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Nach dem Download wird der SHA-256 berechnet und mit dem von Hugging Face verglichen
- [ ] #2 Bei Abweichung wird die Datei verworfen und ein Fehler angezeigt
- [ ] #3 Das Ergebnis steht im Protokoll
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Erste Anfrage ohne Redirect (`max_redirects(0)`), `x-linked-etag` und `location` lesen
2. SHA-256 beim Schreiben mitrechnen (`sha2`), vor dem `rename` vergleichen
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Geprüft: `x-linked-etag` von `resolve/main/ggml-base.bin` = `sha256sum` der Datei (60ed5bc3…). ureq 3.5.1 mit `max_redirects(0)` liefert 302 samt `x-linked-etag` und `location` (Wegwerf-Projekt). Ein voller Download in der App steht noch aus. Neue Abhängigkeit `sha2`.
<!-- SECTION:NOTES:END -->
