---
id: TASK-8
title: Windows-`.exe` auf dem Rechner mit GTX 1070 testen
status: Done
assignee: []
created_date: '2026-11-04 20:48'
updated_date: '2026-11-05 16:54'
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
- [x] #1 Der Workflow läuft auf `windows-latest` grün durch
- [x] #2 Die `.exe` startet, die Fußzeile zeigt die GTX 1070, eine Diktat-Probe gelingt
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Nutzer bestätigt am 2026-11-05 („klappt alles“) den Test von `v0.8.0` unter Windows auf dem Rechner mit GTX 1070 nach dem Ablauf: Fußzeile mit GPU, `Strg+Alt+Leertaste` aus VS Code mit Start-Ton, Auto-Stopp nach Stille mit Stopp-Ton, Kopier-Ton, `Strg+V` ins Claude-Code-Feld, auch mit minimiertem Fenster. Wörtlicher Fußzeilentext und Dauer der Erkennung wurden nicht übermittelt. Workflow auf `windows-latest` seit `v0.1.0` grün.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Windows-`.exe` (`v0.8.0`) auf dem Zielrechner (GTX 1070, i5-3570K ohne AVX2) vom Nutzer erfolgreich getestet: startet, rechnet auf der GPU, Diktat gelingt. Offen bleibt nur, wie schnell die GPU ist – die Dauer wurde nicht gemeldet.
<!-- SECTION:FINAL_SUMMARY:END -->
