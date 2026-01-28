# End-to-End Tests

## Test 1

1. **Datenbank vorbereiten**
   z.B. über die existierenden Skripte `create-database` & `generate-data`,
   oder eine bestehende Datenbank die aktiv verwendet wird.

2. **Programm mit DB URL starten**
   `PM_DATABASE_URL=<url> plant-monitor-frontend` oder nur
   `plant-monitor-frontend` wenn die Variable andersweitig gesetzt
   wurde.

3. **Daten prüfen**
   Die Daten können nun in der Anwendung inspiziert und validiert werden.


## Test 2

1. siehe [Test 1]

2. siehe [Test 1]

3. **Live Daten Eingabe**
   z.B. über das `live-data` Skript, oder aus aktiven Messungen.

4. **Beobachtung**
   Die Anwendung spiegelt nun live die Änderungen der Datenbank wieder.


## Test 3

1. siehe [Test 1]

2. **Programm mit DB URL starten**
   `PM_DATABASE_URL=<url> plant-monitor-backend` oder nur
   `plant-monitor-backend` wenn die Variable andersweitig gesetzt
   wurde.

3. **Anfragen senden**
   z.B. von IoT-Geräten wie ein ESP32 oder Raspberry Pi,
   oder durch Funktionen in `curls.sh`

4. **Beobachtung**
   Der Server muss die Anfragen annehmen und beantworten.
   Man könnte z.B. periodisch Daten vom Server auslesen, diese
   müssen dann die neuen Daten beinhalten.
   Wenn die Anfragen manuell gesendet werden, müssen die Rückgabewerte
   stimmen.
