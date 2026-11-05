---
id: TASK-14
title: 'Oberfläche luftiger: Abstände, Fenstergröße, Zeilenhöhe'
status: Done
assignee:
  - '@claude'
created_date: '2026-11-05 16:23'
updated_date: '2026-11-05 16:54'
labels: []
dependencies: []
priority: medium
type: enhancement
ordinal: 14000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der Nutzer findet die Oberfläche gedrungen: Buttons und Texte stehen eng beieinander, längere Diktate haben im Textfeld sehr enge Zeilen. Bei der Mindestgröße 640×480 überdeckt in der Fußzeile die Zeile „Berechnung läuft auf: …“ die Versionsangabe.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Zwischen Bedienelementen und Texten ist sichtbar mehr Abstand; Buttons sind größer und stehen deutlich voneinander getrennt
- [x] #2 Das Textfeld hat einen größeren Zeilenabstand, mehrzeilige Diktate sind gut lesbar; der Cursor sitzt nach dem Einfügen richtig
- [x] #3 Das Fenster startet größer; bei der Mindestgröße ist nichts überdeckt oder abgeschnitten, auch nicht die Version in der Fußzeile
- [x] #4 Vorher/Nachher per Screenshot geprüft, in Standard- und Mindestgröße, im dunklen und hellen Modus
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Umgesetzt: `item_spacing` 10×8 (vorher 8×6), `button_padding` 14×7 (10×5), `interact_size.y` 30, Panel-Ränder 20×14 (14×10), Aufnahme-Button 280×48 (250×44), Abstände zwischen Button-Zeile, Status, Textfeld und Buttons darunter. Textfeld: Schrift 16 px, `extra_text_line_spacing` 8 nur dort (egui rechnet die Zeilenhöhe aus Zeilenhöhe der Schrift plus diesem Wert), Innenrand 16×14. Grid „Quelle/Ziel“ mit `min_row_height(18)`, sonst übernimmt es die neue Mindesthöhe 30. Fußzeile: Rechenwerk-Zeile wird zuletzt im Rest-Platz gesetzt und kürzt mit „…“ ab (voller Text als Tooltip). Fenster 880×680 (780×600), Mindestgröße 720×600 (640×480) – höher als 680 würde mit Taskleiste nicht auf 1366×768 passen.

Screenshots (Scratchpad `ux/`): vorher 780×600 und 640×480, nachher 880×680 und 720×600, dunkel und hell, mit offenem Protokoll. Version in der Fußzeile bei 720 px frei, Rechenwerk gekürzt. Cursor sitzt nach dem Einfügen hinter dem eingefügten Text. Bei Mindestgröße mit offenem Protokoll bleiben etwa 4 Textzeilen sichtbar, der Rest scrollt.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Mehr Abstand, größere Buttons, Fenster 880×680 (mindestens 720×600), Diktatfeld mit 16 px und 8 px Zusatz-Zeilenabstand, Fußzeile kürzt statt zu überdecken, Theme-Schalter nur mit Symbolen (🌙 rechts). Per Screenshot in beiden Größen und Themes geprüft, erschienen in `v0.7.0`.
<!-- SECTION:FINAL_SUMMARY:END -->
