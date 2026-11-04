---
id: TASK-9
title: Oberfläche nach UX-Review überarbeiten
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-04 21:31'
updated_date: '2026-11-04 21:38'
labels: []
dependencies: []
priority: high
type: enhancement
ordinal: 9000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Ein UX-Review der ersten Oberfläche fand einen Vertrauensbruch (die Auswahl zeigt ein anderes Modell als das, mit dem transkribiert wird), keine Rückmeldung bei stummem Mikrofon (Whisper erfindet dann „Vielen Dank“), einen kaum sichtbaren, nicht abbrechbaren Download, Datenverlust durch „Leeren“, untergehende Fehler und ein Layout, das bei Mindestgröße bricht. Vom Nutzer entschieden: Fußzeile bleibt, Quelle und Ziel bleiben sichtbar, Protokoll wird einklappbar.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Ein vorhandenes Modell wird beim Auswählen und beim Start automatisch geladen; aufgenommen wird nur mit dem angezeigten Modell
- [ ] #2 Während der Aufnahme zeigt ein Pegel, ob Ton ankommt; eine stumme Aufnahme wird nicht transkribiert, sondern mit Hinweis gemeldet
- [x] #3 Der Download zeigt einen Fortschrittsbalken mit MB und lässt sich abbrechen
- [x] #4 „Leeren“ lässt sich rückgängig machen
- [x] #5 Fehler stehen hervorgehoben mit ⚠ bis zur nächsten Aktion
- [x] #6 Bei Mindestfenstergröße ist nichts abgeschnitten oder überlappt
- [ ] #7 Protokoll ist einklappbar und öffnet sich beim ersten Start, beim Download und bei Fehlern
- [ ] #8 Tastenkürzel `Strg+R` (Aufnahme) und `Strg+Shift+C` (Kopieren) funktionieren
- [ ] #9 Neuer Text wird an der Cursorposition eingefügt
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Modellwahl: Auto-Laden, Button nur „Herunterladen (Größe)“, `small` entfernen, neue Bezeichnungen
2. Aufnahme: Pegel (`ProgressBar`), Gerätename ins Protokoll, Stille-Erkennung mit Unit-Test
3. Download: `Msg::Progress`, Balken, Abbrechen über `AtomicBool`; Spinner beim Laden
4. Text: Rückgängig für Leeren, Einfügen am Cursor, „Kopiert ✓“ am Button, Tastenkürzel
5. Fehlerzeile, Protokoll einklappbar ohne Zeitstempel in der UI, Fußzeile neutral bei CPU, Fenstertitel bei Aufnahme
6. Screenshots aller Zustände inkl. Mindestgröße; README anpassen
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Per Screenshot unter WSL geprüft: Auto-Laden beim Start, Pegel während der Aufnahme, `Strg+R`, Fortschrittsbalken (23 / 141 MB), Abbrechen (`.part` entfernt, Fehler rot mit ⚠), voller Download von `base` mit Prüfsumme, Rückgängig nach Leeren, „Kopiert ✓“, Protokoll öffnet bei Download und Fehler, 640×480 ohne Überlappung. Stille-Erkennung, Einfügen am Cursor und Pegelskala per Unit-Test. Nicht live geprüft: Auto-Laden beim Wechsel auf ein vorhandenes Modell, stumme Aufnahme, `Strg+Shift+C`, Einfügen mitten im Text, Protokoll beim allerersten Start. Raumrauschen ohne Sprache ergibt weiterhin kurze Halluzinationen („Ja.“) – die Stille-Erkennung fängt nur echte Stille.
<!-- SECTION:NOTES:END -->
