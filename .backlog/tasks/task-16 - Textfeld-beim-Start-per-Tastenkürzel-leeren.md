---
id: TASK-16
title: Textfeld beim Start per Tastenkürzel leeren
status: Done
assignee:
  - '@claude'
created_date: '2026-11-05 16:55'
updated_date: '2026-11-05 16:57'
labels: []
dependencies: []
priority: medium
type: feature
ordinal: 16000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der Nutzer diktiert per `Strg+Alt+Leertaste` in andere Programme und fügt mit `Strg+V` ein. Der alte Text bleibt in Mundwerk stehen, jedes neue Diktat hängt sich an, und „Automatisch kopieren“ kopiert alles Bisherige mit. Mundwerk kann nicht erkennen, ob anderswo eingefügt wurde. Vom Nutzer gewählt (Variante A): Beim Start per globalem Kürzel wird das Feld vorher geleert; Start im Fenster setzt wie bisher fort.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Start per `Strg+Alt+Leertaste` leert vorher das Textfeld; Start per Button oder `Strg+R` lässt den Text stehen
- [x] #2 „Rückgängig“ holt den geleerten Text zurück, auch nachdem das neue Diktat da ist; das neue Diktat bleibt dahinter erhalten
- [x] #3 Mit „Automatisch kopieren“ landet nur das neue Diktat in der Zwischenablage
- [x] #4 README beschreibt das Verhalten
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Live unter WSL: „Alter Text …“ im Feld, Start per `Strg+Alt+Leertaste` (fokussiertes `xmessage`) → Feld leer, Button „Rückgängig“; nach Stopp und Erkennung nur das neue Diktat im Feld und in der Zwischenablage (`xclip`). „Rückgängig“ → „Alter Text aus dem letzten Diktat. <neues Diktat>“. Start per Button → Text bleibt stehen. Unit-Test `restore_cleared`. Undo überlebt jetzt auch ein Diktat (vorher setzte `Msg::Text` es zurück), damit der geleerte Text nach dem Hintergrund-Diktat noch erreichbar ist. Nebenbefund: Whisper lieferte aus Raumrauschen „B��. Dispatch.“ – Ersatzzeichen deuten auf ungültiges UTF-8 in der Ausgabe hin, nicht untersucht.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Start per globalem Kürzel leert das Textfeld vorher; „Rückgängig“ stellt den alten Text vor das neue Diktat zurück; Start im Fenster setzt wie bisher fort. Mit „Automatisch kopieren“ landet so nur das neue Diktat in der Zwischenablage. Live unter WSL geprüft.
<!-- SECTION:FINAL_SUMMARY:END -->
