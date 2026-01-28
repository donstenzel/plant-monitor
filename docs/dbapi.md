# Datenbankschnittstellenbeschreibung

Wir greifen über ein ORM (`rust-diesel`) auf die Datenbank zu.
Als Datenbankengine verwenden wir SQLite.

Die Datenbank kann sowohl lokal als auch auf einem Server liegen.

Um eine lokale Datenbank zu erstellen kann die `create.sql` Datei
verwendet werden. Mit `data.sql` können Test Daten eingefügt werden.
`generate-data` kann Test Verlaufs Daten generieren, welche dann wieder
eingefügt werden können. `create-database` kombiniert und automatisiert
die ersten beiden dieser Prozesse.
