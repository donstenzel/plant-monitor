#![allow(non_snake_case)]

diesel::table! {
    PlantTypes (id) {
        id -> Integer,
        name -> Text,
        scientific -> Text,
        description -> Text,
        ml_per_day -> Integer,
        ideal_ph -> Float,
        ideal_temp -> Float,
        ideal_hum -> Float,
        ideal_ec -> Float,
        ideal_lux -> Integer,
        image -> Text,
    }
}

diesel::table! {
    PotTypes (id) {
        id -> Integer,
        name -> Text,
        drainage -> Integer,
        volume -> Integer,
        image -> Text,
    }
}

diesel::table! {
    Usages (id) {
        id -> Integer,
        plant -> Integer,
        pot -> Integer,
        planted -> Timestamp,
    }
}

diesel::joinable!(Usages -> PlantTypes (plant));
diesel::joinable!(Usages -> PotTypes (pot));

diesel::table! {
    Measurements (id) {
        id -> Integer,
        usage -> Integer,
        humidity -> Float,
        temperature -> Float,
        lux -> Integer,
        ph -> Float,
        ec -> Float,
        instant -> Timestamp,
    }
}

diesel::joinable!(Measurements -> Usages (usage));

diesel::allow_tables_to_appear_in_same_query!(Measurements, PlantTypes, PotTypes, Usages,);
