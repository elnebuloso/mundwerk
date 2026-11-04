---
id: TASK-8
title: Windows-`.exe` auf dem Rechner mit GTX 1070 testen
status: To Do
assignee: []
created_date: '2026-11-04 20:48'
updated_date: '2026-11-04 21:49'
labels: []
dependencies:
  - TASK-10
  - TASK-7
priority: high
type: task
ordinal: 8000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der eigentliche Zielpfad (Windows, Vulkan, GTX 1070, i5-3570K ohne AVX2) ist ungetestet; ebenso der Workflow selbst (`crt-static` mit MSVC, `install-vulkan-sdk-action`, libclang).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Der Workflow läuft auf `windows-latest` grün durch
- [ ] #2 Die `.exe` startet, die Fußzeile zeigt die GTX 1070, eine Diktat-Probe gelingt
<!-- AC:END -->
