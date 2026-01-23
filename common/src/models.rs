use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Selectable, Insertable, Serialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = PlantTypes)]
pub struct PlantType {
    pub id: i32,
    pub name: String,
    pub scientific: String,
    pub description: String,
    pub ml_per_day: i32,
    pub ideal_ph: f32,
    pub ideal_temp: f32,
    pub ideal_hum: f32,
    pub ideal_ec: f32,
    pub ideal_lux: i32,
    pub image: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = PlantTypes)]
pub struct NewPlantType {
    pub name: String,
    pub scientific: String,
    pub description: String,
    pub ml_per_day: i32,
    pub ideal_ph: f32,
    pub ideal_temp: f32,
    pub ideal_hum: f32,
    pub ideal_ec: f32,
    pub ideal_lux: i32,
    pub image: String,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = PlantTypes)]
pub struct UpdatePlantType {
    pub name: Option<String>,
    pub scientific: Option<String>,
    pub description: Option<String>,
    pub ml_per_day: Option<i32>,
    pub ideal_ph: Option<f32>,
    pub ideal_temp: Option<f32>,
    pub ideal_hum: Option<f32>,
    pub ideal_ec: Option<f32>,
    pub ideal_lux: Option<i32>,
    pub image: Option<String>,
}

#[derive(Identifiable, Queryable, Selectable, Insertable, Serialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = PotTypes)]
pub struct PotType {
    pub id: i32,
    pub name: String,
    pub drainage: i32,
    pub volume: i32,
    pub image: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = PotTypes)]
pub struct NewPotType {
    pub name: String,
    pub drainage: i32,
    pub volume: i32,
    pub image: String,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = PotTypes)]
pub struct UpdatePotType {
    pub name: Option<String>,
    pub drainage: Option<i32>,
    pub volume: Option<i32>,
    pub image: Option<String>,
}

#[derive(Identifiable, Queryable, Selectable, Insertable, Associations, Serialize)]
#[diesel(belongs_to(PlantType, foreign_key = plant))]
#[diesel(belongs_to(PotType, foreign_key = pot))]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = Usages)]
pub struct Usage {
    pub id: i32,
    pub plant: i32,
    pub pot: i32,
    pub planted: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize, Associations)]
#[diesel(belongs_to(PlantType, foreign_key = plant))]
#[diesel(belongs_to(PotType, foreign_key = pot))]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = Usages)]
pub struct NewUsage {
    pub plant: i32,
    pub pot: i32,
    pub planted: NaiveDateTime,
}

#[derive(Deserialize, AsChangeset, Associations)]
#[diesel(belongs_to(PlantType, foreign_key = plant))]
#[diesel(belongs_to(PotType, foreign_key = pot))]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = Usages)]
pub struct UpdateUsage {
    pub plant: Option<i32>,
    pub pot: Option<i32>,
    pub planted: Option<NaiveDateTime>,
}

#[derive(Identifiable, Queryable, Selectable, Insertable, Associations, Serialize)]
#[diesel(belongs_to(Usage, foreign_key = usage))]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = Measurements)]
pub struct Measurement {
    pub id: i32,
    pub usage: i32,
    pub humidity: f32,
    pub temperature: f32,
    pub lux: i32,
    pub ph: f32,
    pub ec: f32,
    pub instant: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize, Associations)]
#[diesel(belongs_to(Usage, foreign_key = usage))]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = Measurements)]
pub struct NewMeasurement {
    pub usage: i32,
    pub humidity: f32,
    pub temperature: f32,
    pub lux: i32,
    pub ph: f32,
    pub ec: f32,
    pub instant: NaiveDateTime,
}

#[derive(Deserialize, AsChangeset, Associations)]
#[diesel(belongs_to(Usage, foreign_key = usage))]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = Measurements)]
pub struct UpdateMeasurement {
    pub usage: Option<i32>,
    pub humidity: Option<f32>,
    pub temperature: Option<f32>,
    pub lux: Option<i32>,
    pub ph: Option<f32>,
    pub ec: Option<f32>,
    pub instant: Option<NaiveDateTime>,
}
