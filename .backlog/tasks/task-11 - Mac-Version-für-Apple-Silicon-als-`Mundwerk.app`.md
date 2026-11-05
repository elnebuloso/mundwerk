---
id: TASK-11
title: Mac-Version für Apple Silicon als `Mundwerk.app`
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-05 15:19'
updated_date: '2026-11-05 15:22'
labels: []
dependencies: []
priority: high
type: feature
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Kollegen des Nutzers wollen Mundwerk auf ARM-Macs (Apple Silicon, M1 und neuer) nutzen. Ohne Installation, per Doppelklick startbar. Vom Nutzer entschieden: nur ARM (kein Intel/Universal), nicht signiert/notarisiert wie bei Windows, GPU über Metal. macOS gibt das Mikrofon nur einer App mit `NSMicrophoneUsageDescription` in der `Info.plist` frei, daher ein `.app`-Bundle statt einer nackten Binary. Hier gibt es keinen Mac: Live-Test machen die Kollegen.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Jedes Release enthält `mundwerk-macos-arm64.zip` mit `Mundwerk.app`, gebaut und getestet auf einem macOS-ARM-Runner
- [ ] #2 Der Mac-Build nutzt Metal; die Fußzeile nennt bei CPU den Grund ohne Vulkan-Bezug
- [ ] #3 Die Tastenkürzel heißen auf dem Mac `⌘R` und `⌘⇧C`
- [ ] #4 Die README erklärt einem Mac-Nutzer Download, Entpacken, „Trotzdem öffnen“ und die Mikrofon-Freigabe
- [ ] #5 Ein Kollege hat die App auf einem ARM-Mac gestartet, die Fußzeile zeigt die GPU und ein Diktat gelingt
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. `Cargo.toml`: Feature `metal`
2. `compute_device`: Metal mitzählen, CPU-Grund je Backend; Kürzel-Beschriftung je OS
3. `scripts/macos-app.sh`: `.app` mit `Info.plist`, ad-hoc signieren, mit `ditto` zippen
4. `ci.yml`: Matrix-Eintrag `macos-latest`, Vulkan-SDK dort auslassen; `.releaserc.json`: drittes Asset
5. README: Mac-Abschnitt
6. PR-Lauf oder `workflow_dispatch` prüfen
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Umgesetzt: Feature `metal`, `compute_device` zählt Metal mit (CPU-Grund „keine Metal-fähige GPU gefunden“), Kürzel-Beschriftung und Mikrofon-Hinweis je OS, `scripts/macos-app.sh` (Info.plist mit `NSMicrophoneUsageDescription`, ad-hoc `codesign`, `ditto`), CI-Matrix `macos-latest` mit `MACOSX_DEPLOYMENT_TARGET=13.0`, drittes Asset, README-Abschnitt „Mac“. Unter Linux gebaut und getestet, clippy ohne neue Warnung. Der Mac-Build selbst ist noch nicht gelaufen – erst im CI. Die Dialogtexte in der README („wurde nicht geöffnet“, „Trotzdem öffnen“) stammen aus dem Gedächtnis, nicht von einem Mac. Die `GGML_AVX*`-Schalter greifen laut `ggml-cpu/CMakeLists.txt` nur auf x86.
<!-- SECTION:NOTES:END -->
