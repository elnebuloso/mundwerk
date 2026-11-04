---
id: TASK-10
title: Releases mit semantic-release für Linux und Windows
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-04 21:49'
labels: []
dependencies: []
priority: high
type: chore
ordinal: 10000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der Nutzer will Linux amd64 und Windows bei jedem Release als Artefakte am Tag haben, wie in `grimoire` mit semantic-release aus Conventional Commits. Ersetzt TASK-1 (nur Windows). Ohne Startpunkt würde semantic-release mit `1.0.0` beginnen; unter 1.0 bleibt das die Entscheidung des Nutzers, deshalb wie in `grimoire` ein Tag `v0.0.0` auf dem ersten Commit.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Ein Push auf `main` mit `feat` oder `fix` erzeugt einen Tag `vX.Y.Z` und ein GitHub-Release mit `mundwerk-linux-x86_64` und `mundwerk-windows-x86_64.exe`
- [ ] #2 Pull Requests bauen und testen beide Plattformen, ohne zu releasen
- [ ] #3 Die App meldet im Protokoll die Release-Version
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. `.github/workflows/ci.yml`: Job `version` (Dry-Run), Matrix-Build Linux/Windows, Job `release`
2. `.releaserc.json` nach Vorbild `grimoire`, `@semantic-release/github` mit beiden Assets
3. `MUNDWERK_VERSION` beim Build, Fallback `CARGO_PKG_VERSION`
4. Tag `v0.0.0` auf `156c286`, `release.yml` entfernen, README
<!-- SECTION:PLAN:END -->
