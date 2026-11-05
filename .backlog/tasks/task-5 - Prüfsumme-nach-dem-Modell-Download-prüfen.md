---
id: TASK-5
title: Prüfsumme nach dem Modell-Download prüfen
status: Done
assignee:
  - '@claude'
created_date: '2026-11-04 20:48'
updated_date: '2026-11-05 19:12'
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
- [x] #1 Nach dem Download wird der SHA-256 berechnet und mit dem von Hugging Face verglichen
- [x] #2 Bei Abweichung wird die Datei verworfen und ein Fehler angezeigt
- [x] #3 Das Ergebnis steht im Protokoll
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Erste Anfrage ohne Redirect (`max_redirects(0)`), `x-linked-etag` und `location` lesen
2. SHA-256 beim Schreiben mitrechnen (`sha2`), vor dem `rename` vergleichen

3. Prüfsummen-Vergleich als eigene Funktion, Zweig für falsche Prüfsumme per Unit-Test
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Geprüft: `x-linked-etag` von `resolve/main/ggml-base.bin` = `sha256sum` der Datei (60ed5bc3…). ureq 3.5.1 mit `max_redirects(0)` liefert 302 samt `x-linked-etag` und `location` (Wegwerf-Projekt). Ein voller Download in der App steht noch aus. Neue Abhängigkeit `sha2`.

In der App geprüft: Download von `large-v3-turbo-q5_0` mit „Prüfsumme SHA-256 stimmt: 3942217…“. Der Zweig für eine falsche Prüfsumme ist nicht ausgelöst worden.

Prüfsummen-Vergleich in `keep_if_checksum_matches` ausgelagert. Unit-Test `wrong_checksum_discards_the_download`: Bei Abweichung ist die `.part`-Datei gelöscht, am Ziel liegt nichts, und die Fehlermeldung nennt beide Prüfsummen. Bei Übereinstimmung wird die Datei verschoben. Der Fehler erscheint über `Msg::Error` → `fail` im Fenster (gleicher Weg wie andere Download-Fehler). Live geprüft mit dem VAD-Modell: „Prüfsumme SHA-256 stimmt: 29940d98…“.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Nach jedem Download vergleicht Mundwerk den SHA-256 mit `x-linked-etag` von Hugging Face. Bei Abweichung wird die Datei verworfen und ein Fehler angezeigt, das Ergebnis steht im Protokoll. Der Zweig für eine falsche Prüfsumme ist per Unit-Test abgedeckt, der Erfolgsfall live mit `large-v3-turbo-q5_0` und dem Silero-VAD-Modell geprüft. Eine echte Abweichung bei Hugging Face ist nicht nachgestellt, sie lässt sich ohne manipulierten Server nicht auslösen.
<!-- SECTION:FINAL_SUMMARY:END -->
