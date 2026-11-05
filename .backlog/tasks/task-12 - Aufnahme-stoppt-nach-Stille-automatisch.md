---
id: TASK-12
title: Aufnahme stoppt nach Stille automatisch
status: To Do
assignee: []
created_date: '2026-11-05 15:29'
labels: []
dependencies: []
priority: medium
type: feature
ordinal: 12000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der Nutzer vergisst, die Aufnahme zu stoppen. Mundwerk soll das bei Stille selbst tun. Vom Nutzer entschieden: immer aktiv ohne Einstellung, 4 s Stille nach Sprache, 15 s Frist, wenn noch gar nicht gesprochen wurde. Zu beachten: Die vorhandene Stille-Erkennung (`SILENCE`, `is_silent`) erkennt nur echte Stille. Raumrauschen liegt darüber und würde mit derselben festen Schwelle als Sprache zählen. Live prüfbar unter WSL mit Mikrofon, siehe Stolperfallen in `docs/HANDOFF.md`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Nach dem ersten erkannten Sprechen stoppt die Aufnahme nach 4 s ohne Sprache von selbst und der Text wird erkannt wie beim Klick auf „Aufnahme stoppen“
- [ ] #2 Wird nach dem Start 15 s gar nicht gesprochen, stoppt die Aufnahme mit der Meldung „Kein Ton aufgenommen“
- [ ] #3 Kurze Denkpausen unter 4 s beenden die Aufnahme nicht
- [ ] #4 Gleichmäßiges Raumrauschen ohne Sprache hält die Aufnahme nicht offen
- [ ] #5 Während der Aufnahme zeigt ein Countdown, wann sie wegen Stille stoppt; das Protokoll nennt den automatischen Stopp
- [ ] #6 Die Entscheidung Sprache/Stille ist per Unit-Test abgedeckt, das README beschreibt den Auto-Stopp
<!-- AC:END -->
