---
id: TASK-3
title: 'Modellauswahl zeigt Quelle, Ziel und Status'
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-04 20:48'
updated_date: '2026-11-04 21:07'
labels: []
dependencies:
  - TASK-2
priority: high
type: feature
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der Nutzer soll vor dem Download sehen, welche Modelle es gibt, woher sie geladen und wohin sie gespeichert werden, und welche schon da sind.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Jedes Modell in der Auswahl trägt ein Kennzeichen: grün „vorhanden“ oder orange „nicht heruntergeladen“, lesbar auch ohne Farbe
- [ ] #2 Für das gewählte Modell zeigt das Fenster die Download-URL und den Zielpfad
- [x] #3 Der Button heißt „Herunterladen und laden“, wenn das Modell fehlt
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Kennzeichen ●/○ mit Farbe und Wort in `ComboBox` und Auswahltext
2. URL und Zielpfad unter der Leiste anzeigen
3. Buttontext abhängig vom Vorhandensein
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Umgesetzt in `src/main.rs` (`model_label`, Quelle/Ziel unter der Leiste, Buttontext). Am Bildschirm noch nicht angesehen.

Vom Nutzer bestätigt: Kennzeichen grün/orange mit Zeichen und Wort gut lesbar; „Herunterladen und laden“ für fehlendes Modell genutzt.
<!-- SECTION:NOTES:END -->
