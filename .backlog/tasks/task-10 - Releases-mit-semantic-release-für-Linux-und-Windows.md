---
id: TASK-10
title: Releases mit semantic-release für Linux und Windows
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-04 21:49'
updated_date: '2026-11-05 16:51'
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
- [x] #1 Ein Push auf `main` mit `feat` oder `fix` erzeugt einen Tag `vX.Y.Z` und ein GitHub-Release mit `mundwerk-linux-x86_64` und `mundwerk-windows-x86_64.exe`
- [ ] #2 Pull Requests bauen und testen beide Plattformen, ohne zu releasen
- [x] #3 Die App meldet im Protokoll die Release-Version
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. `.github/workflows/ci.yml`: Job `version` (Dry-Run), Matrix-Build Linux/Windows, Job `release`
2. `.releaserc.json` nach Vorbild `grimoire`, `@semantic-release/github` mit beiden Assets
3. `MUNDWERK_VERSION` beim Build, Fallback `CARGO_PKG_VERSION`
4. Tag `v0.0.0` auf `156c286`, `release.yml` entfernen, README
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Releases `v0.1.0` und `v0.2.0` mit `mundwerk-linux-x86_64` und `mundwerk-windows-x86_64.exe` erzeugt. Das Linux-Binary von `v0.2.0` enthält die Version `0.2.0`. Erster Lauf: Linux scheiterte an `-lvulkan`, behoben mit `libvulkan-dev`. 10 min für „Linux dependencies“ lagen am langsamen Mirror (54 kB/s), im nächsten Lauf 20 s. Build-Cache (`actions/cache`) seit `eacaac8`; der erste Lauf damit hat ihn nur gefüllt (Build 476 s Linux, 426 s Windows). Ein Pull Request ist noch nicht gelaufen.

Mit gefülltem Cache (Lauf für `v0.3.0`): Build 11 s Linux / 30 s Windows statt 476 s / 426 s; Cache-Wiederherstellung 11 s / 42 s.

Ausfall bei `v0.6.0` (Lauf 42062838136): Tag gepusht, dann lehnte GitHub den Push von `refs/notes/semantic-release-v0.6.0` mit „remote rejected (failed)“ ab, Release fehlte. Ein Neustart hilft nicht (semantic-release sieht den Tag als veröffentlicht). Behoben mit `gh release create v0.6.0 --verify-tag` und den Artefakten des Laufs (Version 0.6.0 im Binary bzw. `Info.plist` geprüft). Die fehlende Notiz stört nicht: `v0.7.0` und `v0.8.0` liefen danach normal. Außerdem warteten Mac-Jobs zweimal lange auf einen Runner (einmal nach 15 min abgebrochen, per `gh run rerun --failed` neu gestartet).
<!-- SECTION:NOTES:END -->
