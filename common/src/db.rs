use std::fmt::Display;

use crate::{models::*, schema::*};
use actix_web::ResponseError;
use chrono::NaiveDateTime;
use diesel::{dsl::count, prelude::*};

pub fn url() -> String {
    std::env::var("PM_DATABASE_URL").expect("PM_DATABASE_URL must be set.")
}

#[derive(Debug)]
pub enum Error {
    Connection(ConnectionError),
    Query(diesel::result::Error),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Connection(error) => write!(f, "{error}"),
            Error::Query(error) => write!(f, "{error}"),
        }
    }
}
impl ResponseError for Error {}

pub type DBResult<T> = Result<T, Error>;

pub fn with_connection<T>(
    url: &str,
    f: impl FnOnce(&mut SqliteConnection) -> DBResult<T>,
) -> DBResult<T> {
    f(&mut open(url)?)
}

pub fn open(url: &str) -> DBResult<SqliteConnection> {
    SqliteConnection::establish(url)
        .map_err(Error::Connection)
        .map(|mut c| {
            diesel::sql_query("PRAGMA foreign_keys = ON;")
                .execute(&mut c)
                .unwrap();
            c
        })
}

pub fn all_plant_types(conn: &mut SqliteConnection) -> DBResult<Vec<PlantType>> {
    PlantTypes::table
        .select(PlantType::as_select())
        .load(conn)
        .map_err(Error::Query)
}

pub fn all_pot_types(conn: &mut SqliteConnection) -> DBResult<Vec<PotType>> {
    PotTypes::table
        .select(PotType::as_select())
        .load(conn)
        .map_err(Error::Query)
}

pub fn all_usages(conn: &mut SqliteConnection) -> DBResult<Vec<Usage>> {
    Usages::table
        .select(Usage::as_select())
        .load(conn)
        .map_err(Error::Query)
}

pub fn all_usages_inlined(conn: &mut SqliteConnection) -> DBResult<Vec<(Usage, String, String)>> {
    Usages::table
        .inner_join(PlantTypes::table)
        .inner_join(PotTypes::table)
        .select((Usage::as_select(), PlantTypes::name, PotTypes::name))
        .load(conn)
        .map_err(Error::Query)
}

pub fn usage_counts_by_plant(conn: &mut SqliteConnection) -> DBResult<Vec<(i64, String)>> {
    Usages::table
        .inner_join(PlantTypes::table)
        .group_by(PlantTypes::name)
        .select((count(Usages::plant), PlantTypes::name))
        .load(conn)
        .map_err(Error::Query)
}

pub fn usage_counts_by_pot(conn: &mut SqliteConnection) -> DBResult<Vec<(i64, String)>> {
    Usages::table
        .inner_join(PotTypes::table)
        .group_by(PotTypes::name)
        .select((count(Usages::pot), PotTypes::name))
        .load(conn)
        .map_err(Error::Query)
}

pub fn all_measurements(conn: &mut SqliteConnection) -> DBResult<Vec<Measurement>> {
    Measurements::table
        .order_by(Measurements::instant)
        .select(Measurement::as_select())
        .load(conn)
        .map_err(Error::Query)
}

pub fn measurements_of_usage(
    usage: i32,
    conn: &mut SqliteConnection,
) -> DBResult<Vec<Measurement>> {
    Measurements::table
        .select(Measurement::as_select())
        .filter(Measurements::usage.eq(usage))
        .load(conn)
        .map_err(Error::Query)
}

pub fn measurements_by_usage(
    conn: &mut SqliteConnection,
) -> DBResult<Vec<(Usage, Vec<Measurement>)>> {
    let usages = all_usages(conn)?;

    let measurements = Measurement::belonging_to(&usages)
        .select(Measurement::as_select())
        .load(conn)
        .map_err(Error::Query)?;

    let groups = measurements.grouped_by(&usages);

    Ok(usages.into_iter().zip(groups).collect())
}

pub fn single_usage_inlined(
    usage: i32,
    conn: &mut SqliteConnection,
) -> DBResult<(NaiveDateTime, PlantType, PotType)> {
    Usages::table
        .inner_join(PlantTypes::table)
        .inner_join(PotTypes::table)
        .filter(Usages::id.eq(usage))
        .select((
            Usages::planted,
            PlantType::as_select(),
            PotType::as_select(),
        ))
        .first(conn)
        .map_err(Error::Query)
}

macro_rules! crud {
    ($($base:ident)+) => {
        $( crud!{ @ $base } )+
    };
    (@ $base:ident) => {
        paste::paste! { crud!{ @ [<$base:snake:lower s>] [<$base s>] $base } }
    };
    (@ $module:ident $table:ident $type:ident) => {
        paste::paste! {
            pub mod $module {
                use super::*;

                pub fn all(conn: &mut SqliteConnection) -> DBResult<Vec<$type>> {
                    $table::table
                        .select($type::as_select())
                        .load(conn)
                        .map_err(Error::Query)
                }

                pub fn single(id: i32, conn: &mut SqliteConnection) -> DBResult<$type> {
                    $table::table.find(id).first(conn).map_err(Error::Query)
                }

                pub fn delete(id: i32, conn: &mut SqliteConnection) -> DBResult<$type> {
                    diesel::delete($table::table.find(id))
                        .get_result(conn)
                        .map_err(Error::Query)
                }

                pub fn create(new: [<New $type>], conn: &mut SqliteConnection) -> DBResult<$type> {
                    diesel::insert_into($table::table)
                        .values(new)
                        .get_result(conn)
                        .map_err(Error::Query)
                }

                pub fn update(
                    id: i32,
                    update: [<Update $type>],
                    conn: &mut SqliteConnection,
                ) -> DBResult<$type> {
                    diesel::update($table::table.find(id))
                        .set(&update)
                        .get_result(conn)
                        .map_err(Error::Query)
                }
            }
        }
    };
}

crud! { PotType PlantType Usage Measurement }

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use diesel::RunQueryDsl;

    fn get_test_conn() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        let tables = [
            "CREATE TABLE PlantTypes (id INTEGER PRIMARY KEY, name TEXT NOT NULL, scientific TEXT NOT NULL, description TEXT NOT NULL, ml_per_day INTEGER NOT NULL, ideal_ph REAL NOT NULL, ideal_temp REAL NOT NULL, ideal_hum REAL NOT NULL, ideal_ec REAL NOT NULL, ideal_lux INTEGER NOT NULL, image TEXT NOT NULL)",
            "CREATE TABLE PotTypes (id INTEGER PRIMARY KEY, name TEXT NOT NULL, drainage INTEGER NOT NULL, volume INTEGER NOT NULL, image TEXT NOT NULL)",
            "CREATE TABLE Usages (id INTEGER PRIMARY KEY, plant INTEGER NOT NULL, pot INTEGER NOT NULL, planted TEXT NOT NULL, FOREIGN KEY(plant) REFERENCES PlantTypes(id), FOREIGN KEY(pot) REFERENCES PotTypes(id))",
            "CREATE TABLE Measurements (id INTEGER PRIMARY KEY, usage INTEGER NOT NULL, humidity REAL NOT NULL, temperature REAL NOT NULL, lux INTEGER NOT NULL, ph REAL NOT NULL, ec REAL NOT NULL, instant TEXT NOT NULL, FOREIGN KEY(usage) REFERENCES Usages(id))",
        ];

        for sql in tables {
            diesel::sql_query(sql).execute(&mut conn).unwrap();
        }

        macro_rules! S {
            ($s:literal) => {
                String::from($s)
            };
        }

        diesel::insert_into(PlantTypes::table)
            .values(PlantType {
                id: 1,
                name: S!("Tomato"),
                scientific: S!("Solanum"),
                description: S!("Red"),
                ml_per_day: 500,
                ideal_ph: 6.5,
                ideal_temp: 22.0,
                ideal_hum: 65.0,
                ideal_ec: 2.0,
                ideal_lux: 5000,
                image: S!("t.png"),
            })
            .execute(&mut conn)
            .unwrap();

        diesel::insert_into(PotTypes::table)
            .values(PotType {
                id: 1,
                name: S!("Terra Cotta"),
                drainage: 5,
                volume: 10,
                image: S!("pot.png"),
            })
            .execute(&mut conn)
            .unwrap();

        let planted = NaiveDate::from_ymd_opt(2024, 1, 1)
            .and_then(|d| d.and_hms_opt(12, 0, 0))
            .unwrap();

        diesel::insert_into(Usages::table)
            .values(Usage {
                id: 1,
                plant: 1,
                pot: 1,
                planted,
            })
            .execute(&mut conn)
            .unwrap();

        conn
    }
    #[test]
    fn test_all_plant_types() -> Result<(), Error> {
        let mut conn = get_test_conn();
        let plants = all_plant_types(&mut conn)?;
        assert_eq!(plants.len(), 1);
        assert_eq!(plants[0].name, "Tomato");
        Ok(())
    }

    #[test]
    fn test_count_pot_types() -> Result<(), Error> {
        let mut conn = get_test_conn();
        let pots = all_pot_types(&mut conn)?;
        assert_eq!(pots.len(), 1);
        assert_eq!(pots[0].name, "Terra Cotta");
        Ok(())
    }

    #[test]
    fn test_plant_details() -> Result<(), Error> {
        let mut conn = get_test_conn();
        let plants = all_plant_types(&mut conn)?;
        assert_eq!(plants[0].scientific, "Solanum");
        assert_eq!(plants[0].ml_per_day, 500);
        Ok(())
    }

    #[test]
    fn test_all_usages_with_names() -> Result<(), Error> {
        let mut conn = get_test_conn();
        let results = all_usages_inlined(&mut conn)?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, "Tomato");
        assert_eq!(results[0].2, "Terra Cotta");
        Ok(())
    }

    #[test]
    fn test_single_usage_found() -> Result<(), Error> {
        let mut conn = get_test_conn();
        let (_, plant, pot) = single_usage_inlined(1, &mut conn)?;
        assert_eq!(plant.name, "Tomato");
        assert_eq!(pot.name, "Terra Cotta");
        Ok(())
    }

    #[test]
    fn test_single_usage_not_found_returns_error() {
        let mut conn = get_test_conn();
        assert!(single_usage_inlined(999, &mut conn).is_err());
    }

    #[test]
    fn test_all_measurements_empty() -> Result<(), Error> {
        let mut conn = get_test_conn();
        let results = all_measurements(&mut conn)?;
        assert!(results.is_empty());
        Ok(())
    }
}
