---
id: TASK-23
title: Icon in der Windows-`.exe`-Datei
status: Done
assignee:
  - '@claude'
created_date: '2026-11-05 20:11'
updated_date: '2026-11-05 20:58'
labels: []
dependencies: []
priority: low
type: enhancement
ordinal: 23000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Seit `v0.11.0` setzt Mundwerk das Logo zur Laufzeit als Fenster-Icon. Der Explorer liest das Icon aber aus der `.exe` selbst, dort fehlt es, und damit auch auf Verknüpfungen. Das Icon soll beim Windows-Build als Ressource eingebettet werden (`winresource` in einer `build.rs`, `logo.ico` aus `logo.png`).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Die `.exe` aus der CI zeigt im Explorer das Logo
- [x] #2 Linux- und Mac-Build bleiben unverändert und grün
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. `logo.ico` (256, 64, 48, 32, 16 px) aus `logo.png` per ImageMagick
2. `build.rs` mit `winresource`, nur für Windows: Icon, Produktname und Version aus `MUNDWERK_VERSION` statt `0.1.0` aus `Cargo.toml`
3. Prüfung per Wegwerf-PR auf dem Windows-Runner
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Wegwerf-PR #3 (Lauf 42101417897): alle drei Builds grün. In der `.exe` aus dem Lauf stecken alle fünf Bildgrößen aus `logo.ico` byte-genau sowie `FileDescription` „Mundwerk“ und `ProductVersion` („dev“ im PR). Ohne eigene Versionsangabe hätte `winresource` `0.1.0` aus `Cargo.toml` in die Dateieigenschaften geschrieben, deshalb kommt die Version wie in der Fußzeile aus `MUNDWERK_VERSION`.

Vom Nutzer unter Windows mit `v0.13.0` bestätigt: Das Icon erscheint im Explorer.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Der Windows-Build bettet `logo.ico` (16 bis 256 px) per `build.rs` mit `winresource` in die `.exe` ein, dazu Produktname „Mundwerk“ und die Release-Version aus `MUNDWERK_VERSION` statt `0.1.0` aus `Cargo.toml`. Linux und Mac überspringen den Schritt. Geprüft per Wegwerf-PR #3 (alle Bildgrößen in der `.exe`), im Release `v0.13.0` (Version 0.13.0 in der Datei) und vom Nutzer im Explorer. Ausgeliefert in `v0.13.0`.
<!-- SECTION:FINAL_SUMMARY:END -->
