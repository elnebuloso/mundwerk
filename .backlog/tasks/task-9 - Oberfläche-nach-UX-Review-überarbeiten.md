---
id: TASK-9
title: Oberfläche nach UX-Review überarbeiten
status: Done
assignee:
  - '@claude'
created_date: '2026-11-04 21:31'
updated_date: '2026-11-05 19:23'
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
- [x] #1 Ein vorhandenes Modell wird beim Auswählen und beim Start automatisch geladen; aufgenommen wird nur mit dem angezeigten Modell
- [x] #2 Während der Aufnahme zeigt ein Pegel, ob Ton ankommt; eine stumme Aufnahme wird nicht transkribiert, sondern mit Hinweis gemeldet
- [x] #3 Der Download zeigt einen Fortschrittsbalken mit MB und lässt sich abbrechen
- [x] #4 „Leeren“ lässt sich rückgängig machen
- [x] #5 Fehler stehen hervorgehoben mit ⚠ bis zur nächsten Aktion
- [x] #6 Bei Mindestfenstergröße ist nichts abgeschnitten oder überlappt
- [x] #7 Protokoll ist einklappbar und öffnet sich beim ersten Start, beim Download und bei Fehlern
- [x] #8 Tastenkürzel `Strg+R` (Aufnahme) und `Strg+Shift+C` (Kopieren) funktionieren
- [x] #9 Neuer Text wird an der Cursorposition eingefügt
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

Einfügen am Cursor live geprüft: „Okay.“ diktiert, Cursor davor gesetzt, zweites Diktat ergab „Ja. Okay.“. Der Nutzer hatte zuerst Text am Ende gesehen, hat das nach Rückfrage aber nicht weiter verfolgt.

Restliche Live-Prüfungen unter WSL (`v0.9.1`+VAD-Stand, `dev`):
- Allererster Start mit leerem `HOME` im Scratchpad: „Ordner angelegt“, Protokoll aufgeklappt, nichts heruntergeladen, Aufnahme gesperrt, „Dieses Modell erst herunterladen“.
- Modellwechsel: Auf `base` (nicht vorhanden) bleibt die Aufnahme gesperrt, „Herunterladen (0,15 GB)“ erscheint. Zurück auf `large-v3-turbo-q5_0` lädt es von selbst („Lade … in den Speicher“, „Bereit“, „✓ geladen“).
- Stumme Aufnahme (`pactl set-default-source RDPSink.monitor` ohne Wiedergabe): Auto-Stopp nach 15 s, „⚠ Kein Ton aufgenommen. Mikrofon prüfen …“ rot im Fenster, keine Texterkennung.
- `Strg+R` startet die Aufnahme. `Strg+Shift+C` legt den ganzen Text in die Zwischenablage (vorher „MARKER“, danach Feldinhalt). Beides nur in den ersten Sekunden nach dem Fensterstart prüfbar, solange das X-Fenster den Fokus hat. Stoppen per `Strg+R` ist derselbe Code-Weg (`toggle`), live kam das zweite `Strg+R` wegen Fokusverlust nicht mehr an.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Die Oberfläche ist nach dem UX-Review überarbeitet. Vorhandene Modelle laden beim Start und beim Auswählen von selbst, aufgenommen wird nur mit dem geladenen, angezeigten Modell. Dazu gibt es einen Pegel während der Aufnahme, eine Meldung statt Texterkennung bei stummer Aufnahme, einen Download-Balken mit Abbrechen und Rückgängig nach „Leeren“. Fehler stehen rot mit ⚠, das Protokoll ist einklappbar und öffnet sich beim ersten Start, beim Download und bei Fehlern. Dazu kommen `Strg+R` und `Strg+Shift+C`, Einfügen am Cursor und eine Mindestgröße ohne Überlappung. Alle Punkte sind unter WSL live geprüft. Grenze: Unter WSLg hat das Fenster nur kurz nach dem Start den X-Fokus, die Tastenkürzel im Fenster sind deshalb nur kurz geprüft. Unter Windows bestätigt der Nutzer die Bedienung seit `v0.8.0`.
<!-- SECTION:FINAL_SUMMARY:END -->
