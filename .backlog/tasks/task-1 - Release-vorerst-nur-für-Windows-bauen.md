---
id: TASK-1
title: Release vorerst nur für Windows bauen
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-04 20:48'
updated_date: '2026-11-04 20:52'
labels: []
dependencies: []
priority: high
type: chore
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
GPU-Beschleunigung (Vulkan, GTX 1070) ist nur mit der Windows-`.exe` erreichbar; unter WSL sieht Vulkan keine NVIDIA-GPU. Das Linux-Artefakt bringt bis dahin nur offene Fragen (`libvulkan.so.1`, ALSA) und wird später wieder aufgenommen.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `.github/workflows/release.yml` baut nur noch auf `windows-latest` und veröffentlicht `mundwerk-windows-x86_64.exe`
- [ ] #2 `README.md` nennt nur Windows als ausgelieferte Plattform; Bauen unter Linux bleibt beschrieben
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Linux-Eintrag und Linux-Abhängigkeiten aus `release.yml` entfernen
2. `README.md` anpassen
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Linux-Eintrag und Linux-Abhängigkeiten aus `release.yml` entfernt, Matrix bleibt, damit Linux mit einem Eintrag zurückkommt. README angepasst. Workflow ist noch nie gelaufen, das kommt mit TASK-8.
<!-- SECTION:NOTES:END -->
