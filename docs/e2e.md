# End-to-End Test

## Test 1

1. **Datenbank vorbereiten**
   z.B. über die existierenden Skripte `create-database` & `generate-data`,
   oder eine bestehende Datenbank die aktiv verwendet wird.

2. **Programm mit DB URL starten**
   `PM_DATABASE_URL=<url> plant-monitor` oder nur `plant-monitor` wenn die
   Variable andersweitig gesetzt wurde.

3. **Daten prüfen**
   Die Daten können nun in der Anwendung inspiziert und validiert werden.

## Test 2

1. siehe [Test 1]

2. siehe [Test 1]

3. **Live Daten Eingabe**
   z.B. über das `live-data` Skript, oder aus aktiven Messungen.

4. **Beobachtung**
   Die Anwendung spiegelt nun live die Änderungen der Datenbank wieder.
