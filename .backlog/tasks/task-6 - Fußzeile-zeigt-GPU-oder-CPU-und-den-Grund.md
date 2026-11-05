---
id: TASK-6
title: Fußzeile zeigt GPU oder CPU und den Grund
status: Done
assignee:
  - '@claude'
created_date: '2026-11-04 20:48'
updated_date: '2026-11-05 16:54'
labels: []
dependencies: []
priority: high
type: feature
ordinal: 6000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Die App nutzt automatisch eine GPU und fällt sonst auf die CPU zurück. Bisher sieht man das nur auf stderr. Der Nutzer will im Fenster sehen, was genutzt wird und warum.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Eine Fußzeile zeigt das Rechenwerk, z. B. GPU-Name mit Vulkan oder CPU
- [x] #2 Bei CPU nennt sie den Grund: keine Vulkan-GPU gefunden oder ohne GPU-Unterstützung gebaut
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Beim Start ggml-Geräte über `whisper_rs_sys::ggml_backend_dev_*` abfragen, gleiche Auswahl wie `whisper_backend_init_gpu` (erstes GPU/IGPU)
2. Grund aus `cfg!(feature = "vulkan")` bzw. fehlendem Gerät ableiten
3. Fußzeile + Protokollzeile
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Feature `raw-api` von `whisper-rs` nötig. Unter WSL geprüft: „CPU – keine Vulkan-fähige GPU gefunden“. Der GPU-Zweig ist nur auf Windows prüfbar (TASK-8). Grenze: scheitert die Initialisierung der gefundenen GPU in whisper.cpp, zeigt die Fußzeile trotzdem die GPU.

CPU-Grund „keine Vulkan-fähige GPU gefunden“ per Screenshot bestätigt. GPU-Zweig (AC 1) wird mit TASK-8 unter Windows geprüft.

Nutzer bestätigt am 2026-11-05 („klappt alles“) den Test von `v0.8.0` unter Windows auf dem Rechner mit GTX 1070 nach dem Ablauf: Fußzeile mit GPU, `Strg+Alt+Leertaste` aus VS Code mit Start-Ton, Auto-Stopp nach Stille mit Stopp-Ton, Kopier-Ton, `Strg+V` ins Claude-Code-Feld, auch mit minimiertem Fenster. Wörtlicher Fußzeilentext und Dauer der Erkennung wurden nicht übermittelt.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Die Fußzeile zeigt das Rechenwerk: GPU-Name samt Backend (Vulkan, auf dem Mac Metal) oder CPU mit Grund. CPU-Zweig unter WSL per Screenshot, GPU-Zweig vom Nutzer unter Windows bestätigt. Grenze: Scheitert die Initialisierung der gefundenen GPU in whisper.cpp, zeigt die Fußzeile trotzdem die GPU.
<!-- SECTION:FINAL_SUMMARY:END -->
