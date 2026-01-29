# Unit-Tests

Wir testen bisher nur die Datenbank Schnittstelle. Hier testen wir
die verschiedenen Funktionen, wie Mengen von Gruppen, Auflistung und
Verknüpfungen. Wir testen diese einerseits, um sicherzustellen, dass
unser Schema in `create.sql` mit dem Schema in `schema.rs`
übereinstimmt, und andererseits, dass die Queries in den Funktionen
das tun, was sie sollen.

In Rust werden Tests häufig in einem Modul mit dem Namen `tests`
in der selben Datei oder außerhalb untergebracht. In diesem Fall
liegen sie direkt in `db.rs`. Das `#[cfg(test)]` Attribut stellt sicher,
dass dieser Code nicht in der Release Konfiguration des Programms
landet, sondern nur, wenn die Tests ausgeführt werden.

Wir verwenden hier die Möglichkeit, eine eigene Verbindung für
die verschiedenen Funktionen als Eingabe zu übergeben. Dadurch
können wir eine Funktion `get_test_conn` definieren, welche eine
Verbindung mit einer Datenbank mit nötigen Test Daten und Tabellen
herstellt. Die Datenbank ist temporär und befindet sich vollkommen
im Arbeitsspeicher. Die Tabellenschemata laden wir aus der selben
Datei, die wir für die Kreation der eigentlichen Datenbank benutzen.

Ein einzelner Unit-Test ist eine Funktion mit dem `#[test]` Attribut.
Alle Funktionen, die markiert sind, werden automatisch ausgeführt,
und Fehler in Form von `panic`s oder `Result::Err`s werden gesammelt
und aufgeführt. Rust hat hier einige Hilfsmacros wie z.B. `assert!`
um Invarianten sicherzustellen.

Die Tests werden mit dem Befehl `cargo test` ausgeführt.
Grundsätzlich führt der Befehl alle Tests gleichzeitig aus, und zeigt
dann eine Statistik der Tests und welche Tests fehlgeschlagen sind.
Man kann allerdings auch einzelne Tests ausführen, dann lautet der
Befehl `cargo test {test-name}`, wo `{test-name}` mit dem Namen der
jeweiligen Funktion ersetzt wird.


In Zukunft sollen noch Tests für die JSON Schnittstelle des Backends
hinzukommen.
