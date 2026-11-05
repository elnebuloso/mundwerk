---
id: TASK-13
title: Text automatisch kopieren und Aufnahme per globalem Tastenkürzel
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-05 16:03'
updated_date: '2026-11-05 16:11'
labels: []
dependencies: []
priority: high
type: feature
ordinal: 13000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Der Nutzer will diktierten Text direkt in anderen Programmen nutzen (z. B. das Eingabefeld von Claude Code in VS Code), ohne Plugin und ohne zwischen Fenstern zu wechseln. Stufe 1, vom Nutzer gewählt: Mundwerk kopiert nach jedem Diktat selbst, eingefügt wird von Hand mit `Strg+V`; dazu ein Tastenkürzel, das auch wirkt, wenn Mundwerk nicht im Vordergrund ist. Direktes Einfügen durch Mundwerk (simuliertes `Strg+V`) ist bewusst nicht Teil davon. Globale Kürzel gehen laut `global-hotkey` nur unter Windows, macOS und X11, nicht unter Wayland; unter WSL erreichen sie keine Windows-Fenster.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Ein Haken „Automatisch kopieren“ legt nach jedem Diktat den ganzen Text in die Zwischenablage; der Haken lässt sich setzen und entfernen und bleibt über einen Neustart erhalten
- [x] #2 `Strg+Alt+Leertaste` (Mac: `⌃⌥Leertaste`) startet und stoppt die Aufnahme, auch wenn ein anderes Fenster im Vordergrund ist
- [x] #3 Lässt sich das Kürzel nicht registrieren, läuft Mundwerk normal weiter und das Protokoll nennt den Grund
- [x] #4 Die Oberfläche nennt das globale Kürzel, das README beschreibt Haken, Kürzel und die Grenzen (Wayland, WSL)
- [ ] #5 Unter Windows geprüft: Kürzel in VS Code gedrückt, Diktat, `Strg+V` fügt den Text ein
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. `global-hotkey` 0.9: Manager im eframe-Creator (Hauptthread, wie für Windows/macOS verlangt), `Strg+Alt+Leertaste` registrieren, Event-Handler schickt `Msg::Hotkey` und weckt die UI
2. `handle` bekommt den UI-Kontext; `Msg::Hotkey` schaltet die Aufnahme wie `Strg+R`, `Msg::Text` kopiert bei gesetztem Haken
3. Haken neben „Leeren“, gespeichert in `~/.mundwerk/auto-copy` wie das Theme; Standard an
4. Beschriftung des Aufnahme-Buttons nennt das globale Kürzel; README
5. Unter WSL/X11 live prüfen (Kürzel bei fokussiertem anderem Fenster, Zwischenablage per `xclip`)
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Live unter WSLg/X11 geprüft: Mit fokussiertem `xmessage` startete `Strg+Alt+Leertaste` die Aufnahme (Titel „● Aufnahme – Mundwerk“), ein zweiter Druck stoppte sie, der Fokus blieb bei `xmessage`. Danach lag der erkannte Text in der Zwischenablage (`xclip -o -selection clipboard`), Status „…, kopiert“. Haken abgewählt → `~/.mundwerk/auto-copy` = `off`, nach Neustart weiterhin aus (Screenshot 640×480, kein Überlappen in der Zeile). Zweite Instanz: „Tastenkürzel … nicht verfügbar: HotKey already registered …“, App läuft weiter, Button nennt `Strg+R`.

Stolperfalle beim Testen unter WSLg: Der X-Fokus folgt dem Windows-Fokus; ist ein Windows-Programm aktiv, hat kein X-Fenster den Fokus und X11 verwirft die Tasten. Ein frisch geöffnetes X-Fenster bekommt den Fokus kurz – nur so ließ sich testen.

macOS: `global-hotkey` nutzt `RegisterEventHotKey` (Event-Tap nur für Medientasten), also nach Quelltext keine Bedienungshilfen-Freigabe nötig – nicht auf einem Mac geprüft. Ungeprüft: `⌃⌥Leertaste` ist auf dem Mac womöglich als „Nächste Eingabequelle“ belegt. Offen: AC 5 unter Windows; ob ein minimiertes Fenster unter Windows den Kürzel-Event verarbeitet.
<!-- SECTION:NOTES:END -->
