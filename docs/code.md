# Quellcode Beschreibung

Der Code besteht grob aus 5 Teilen:

- Backend Server

- Frontend GUI

- Datenbank Schnittstelle

- Datenbank Definition / Test Daten

- Skripte

Sowohl Backend als auch Frontend verwenden die Datenbank Schnittstelle
um ihre Funktionalitäten zu verwirklichen. Das Backend kann sowohl
Daten *auslesen*, als auch Neue *hinzufügen* oder Bestehende *abändern*.
Das Frontend kann lediglich Daten *auslesen*.

Wir haben uns für die Rust Programmiersprache entschieden, da diese
ein starkes Typsystem besitzt, Macro basierte Meta-Programmierung
unterstützt und es ergonomische Bibliotheken für SQL und HTTP-Routing
gibt. Außerdem ist Rust relativ schnell, weshalb wir an gewissen Stellen
nicht den effizientesten Weg wählen mussten.

Für Skripte haben wir Uiua verwendet, da es die Skripte übersichtlich
und simpel hält.

Frontend und Backend werden nach dem Projekt weiterentwickelt,
da viele gute und sinnvolle Erweiterungen offen stehen.


## Datenbank Schnittstelle

Die Datenbank Schnittstelle ist in `common/*.rs` implementiert.

`schema.rs` beinhaltet Tabellendefinitionen, um diese in Rust Typ-sicher
zu verwenden. Dies beugt Fehler vor, da der Rust Compiler statische
Analyse anwendet, und somit Fehler finden kann, bevor diese tatsächlich
auftreten.

`models.rs` beinhaltet Strukturen, welche auf der Rust Seite verwendet
werden um Entitäten aus der Datenbank abzubilden. Wichtig hierbei ist,
dass diese nicht 1 zu 1 mit Entitäten übereinstimmen müssen.
Es gibt einmal direkte "Kopien" der Entitäten, und Strukturen für
partielle Updates und Neuanlage.

`db.rs` beinhaltet die Schnittstellen Funktionen, welche von Backend und
Frontend verwendet werden. Diese wurden je nach Bedarf erstellt.
Es gibt simplere Funktionen wie `all_plant_types`, die einfach eine
Tabelle ausgeben, aber auch kompliziertere Verknüpfungen, wie z.B.
`single_usage_inlined`. Diese Funktion gibt die Einpflanzzeit und die
referenzierten Entitäten anhand der Fremdschlüssel in einer `Usage`
zurück.

Mithilfe des `crud` Macros wird dann außerdem eine Gruppe an Funktionen
automatisch für die verschiedenen Entitäten generiert. Diese Funktionen
bilden eine sogenannte CRUD API:
- Create
- Read
- Update
- Delete
Diese werden im Backend verwendet um die verschiedenen Funktionalitäten
zu gewährleisten.

Die Schnittstelle verwendet weitläufig `diesel`s Query Builder API.
Dies sorgt für lesbaren und einheitlichen Code und erhöhte Sicherheit
als rohe SQL-Befehle. Im Falle von parameterisierten SQL-Befehlen könnte
es zu SQL-Injection kommen, wenn mit Strings gearbeitet wird.

Es gibt eine zentrale Funktion `open` um Verbindungen aufzubauen. Wir
setzen hier eine SQLite Option "foreign_keys = ON", damit SQLite unsere
Fremdschlüssel-Relationen berücksichtigt und validiert.

Die Datenbank URL wird über eine Umgebungsvariable gesetzt, ist aber an
allen relevanten Stellen ein Parameter, da man so eine Test-Datenbank
verwenden kann.


## Backend Server

Das Backend ist ein Server, welcher Daten aus der Datenbank ausgeben,
neue Daten aufnehmen und bestehende Daten ändern kann. Es stellt eine
RESTful API dar. Konkret gibt es folgende Funktionen für jede Entität:
- Create: nimmt neuen Datensatz in Datenbank auf
- Read: gibt bestehenden Datensatz (spezifisch oder alle) aus
- Update: ändert existierende Daten (partiell oder ganz)
- Delete: löscht bestehenden Datensatz

Wie in der Datenbank Schnittstelle verwenden wir hier ein Macro `crud`
welcher die repetitive Logik automatisch generiert. Für jede Route
die wir unterstützen wollen (pro Entität) wird eine Funktion generiert.
Diese verlaufen alle nach dem selben Schema: Sie erhalten Parameter aus
URL (= Id der Entität) und/oder Body der Request (JSON Daten), rufen
dann die jeweilige Funktion in der Datenbank Schnittstelle auf, und
geben das Ergebnis als Response in Form von JSON Daten zurück.

Die Kernlogik ist dann der Router von `actix_web`, welcher durch HTTP-
Methode und URL an jeweilige Funktionen weiterleiten kann. Für jede
Entität gibt es 5 Routen:
- `/{entity}` mit POST Request und Daten
  Fügt ein Datensatz in die Datenbank ein
- `/{entity}` mit GET
  Gibt die gesamte Tabelle zurück
- `/{entity}/{id}` mit GET
  Gibt Datensatz mit Id = id zurück
- `/{entity}/{id}` mit PUT und Daten
  Überschreibt existierenden Datensatz mit Id = id ganz oder partiell
- `/{entity}/{id}` mit DELETE
  Löscht Datensatz mit Id = id

Wir binden den Server lokal auf Port 8080. IoT-Geräte können dann alle
gemeinsam Daten einspielen, und das Frontend kann diese darstellen.


## Frontend GUI

Das Frontend ist eine Monitoringsoftware für Pflanzen.

`gui.rs` beinhaltet die Hauptlogik und Funktionen die mehrfach verwendet
werden:
- `error_label` um Fehler darzustellen
- `card` implementiert das gleichnamige UI Element
- `hex` um Farben aus Hex-Strings zu parsen

Die `App` lädt beim Starten die Bild-Lade-Funktionalität und
initialisiert den Basiszustand.

Das Framework, welches wir verwenden basiert auf der Immediate Mode
GUI Technik, bei der keine komplexe Datenstruktur aufgebaut wird.
Stattdessen werden die UI-Elemente "direkt" gezeichnet und geben
Interaktionen etc. zurück. Ein Knopf ist kein Objekt mit einer `OnClick`
Methode, welche die Logik darstellt, sondern eine Funktion, welche den
Klick-Status zurückgibt (TRUE = wurde geklickt). Mit einer simplen
If Anweisung kann dann die selbe Logik emuliert werden.

Dadurch gibt es eine zentrale Update Funktion, die das gesamte UI neu
zeichnet. In dieser wird geprüft, ob ein Tab-Button gedrückt wurde, und
wenn ja, wechselt den Tab entsprechend. Anschließend wird die aktuelle
Seite angezeigt. Die Seiten sind jeweils in einer eigenen Datei
implementiert.

Da es viele Updates geben kann, ist es essentiell, die Daten aus der
Datenbank *nicht* jedes mal neu zu laden. Um die zu gewährleisten haben
wir eine Cache implementiert. Diese berechnet Daten nur dann neu, wenn
sie noch nicht existieren (also beim 1. mal), oder wenn sie länger als
1 Sekunde alt sind. Dies verringert die Datenbankauslastung massiv.

Viele der Funktionen, welche wir cachen, haben keine Eingaben und die
selbe Signatur, weshalb wieder ein Macro verwendet wird um diese zu
generieren.

In den Seiten verwenden wir nicht direkt die Datenbank, sondern
die Funktionen der Cache.


`dashboard.rs` beinhaltet die Hauptseite. Hier werden Graphen mit allen
Pflanzen und derer Werte angezeigt. Außerdem eine Statistik, wie die
unterschiedlichen Pflanzen und Töpfe vertreten sind.
Wir bauen hier programmatisch eine `Line` oder ein `BarChart` aus den
Werten, welche wir aus der Datenbank lesen, zusammen, und stellen diese
in `Plot`s dar. Die Daten müssen dafür in F64 umgewandelt werden. Für
Zeiten bedeutet dies z.B., den Unix-Timestamp zu verwenden. Für die
Anzeige haben wir dann extra `Formatter` hinterlegt, damit die Achsen
die eigentlichen Daten anzeigen, also z.B. das tatsächliche Datum.

`usages.rs` ist eine Seite mit den einzelnen Pflanzen. Links kann die
Pflanze ausgewählt werden, und im Hauptfeld werden dann Infos dazu
dargestellt. Wir laden alle Datensätze, zeigen die Liste an, und je
nach aktueller Pflanze zeigen wir genauere Infos an. Dafür laden wir
die Messungen, die zu dieser Pflanze gehören und erstellen wieder
Linien für `Plot`s.

`plants.rs` und `pots.rs` folgen dem gleichen Schema:
Daten werden geladen, dann wird eine FlexBox erstellt und für jeden
Datensatz eine `card` angezeigt. Die Anzeige ist der Name und das Bild
der Pflanze / des Topfes und wenn man mit der Maus über die Karte geht,
werden weitere Infos angezeigt.

`history.rs` zeigt den gesamten Verlauf der Messdaten. Wir haben hier
eine Funktion zum Ausmessen von Texten eingebaut, um die Teile
gleichmäßig zu trennen. Desweiteren haben wir eine Funktion erstellt,
welche anhand einer Id eine Farbe zurück gibt. Dies ist eine Hash-
Funktion, d.h. der selbe Eingabewert sorgt immer für den selben,
(ausreichend) eindeutigen, Ausgabewert. Diese Farbe wird dann für die
Zeile verwendet. Dadurch sind Messungen, welche zusammen gehören,
direkt farblich erkennbar.

Für alle Seiten gilt, wenn Daten nicht geladen werden können, wird ein
Label mit Nachricht angezeigt, statt abzustürzen. Die App sollte im
normalen Verlauf **niemals** abstürzen.


## Datenbank Definition / Test Daten

`create.sql` beinhaltet die Tabellenschemata der Datenbank.

`data.sql` beinhaltet Test Daten für manuelle Tests.


## Skripte

`create-database` erstellt eine Datenbank im gleichen Verzeichnis,
mit den nötigen Tabellen und Test Daten. Wenn die Datenbank bereits
existiert, wird sie **überschrieben**.

`generate-data` generiert Messdaten für 4 Pflanzen (Id 1-4), welche
ebenfalls für Tests verwendet werden können.

`live-data` simuliert das Einspielen von Daten, während die Applikation
läuft. Diese Änderungen werden von der Applikation wiedergespiegelt.

`curls.sh` beinhaltet Funktionen um das Backend zu testen. Diese können
in einer Bash-Shell verwendet werden, dafür einfach `source curls.sh`
und anschließend den Namen der Funktion als Befehl.
