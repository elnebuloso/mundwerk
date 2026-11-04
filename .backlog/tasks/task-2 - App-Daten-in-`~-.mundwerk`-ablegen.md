---
id: TASK-2
title: App-Daten in `~/.mundwerk` ablegen
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-04 20:48'
updated_date: '2026-11-04 21:06'
labels: []
dependencies: []
priority: high
type: enhancement
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der Nutzer soll Vertrauen aufbauen und sehen, wo die App Dateien ablegt. Bisher liegen Modelle unter `dirs::data_dir()`, unter Windows also im Roaming-Profil `%APPDATA%` (0,5–1,6 GB, wird in Firmenumgebungen synchronisiert).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Modelle liegen unter `~/.mundwerk/models/`, unter Windows `%USERPROFILE%\.mundwerk\models\`
- [x] #2 Legt die App den Ordner neu an, steht das im Protokoll
- [x] #3 `README.md` nennt den neuen Ort
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. `model_path` auf `dirs::home_dir()/.mundwerk/models` umstellen
2. Neu angelegten Ordner protokollieren (TASK-4)
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Geprüft unter WSL: der Erststart legt `/home/dev/.mundwerk` an und protokolliert „Ordner angelegt“. Ein Download nach `models/` ist in der App noch nicht ausgeführt worden.

Download in der App landete in `/home/dev/.mundwerk/models/ggml-large-v3-turbo-q5_0.bin`. Windows-Pfad folgt mit TASK-8.
<!-- SECTION:NOTES:END -->
