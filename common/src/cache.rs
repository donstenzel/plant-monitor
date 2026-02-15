use chrono::NaiveDateTime;
use std::collections::{HashMap, hash_map::Entry::*};
use std::time::Duration;
use std::{any::Any, time::Instant};

use crate::{
    db::{self, DBResult, open, url},
    models::*,
};

struct Entry {
    data: Box<dyn Any>,
    timestamp: Instant,
}

#[repr(transparent)]
#[derive(Default)]
pub struct Cache(HashMap<String, Entry>);

macro_rules! cached {
    ( $name:ident => $t:ty ) => {
        pub fn $name(&mut self) -> &db::DBResult<Vec<$t>> {
            self.get_or_compute(stringify!($name), Duration::from_secs(1), || {
                db::with_connection(&db::url(), db::$name)
            })
        }
    };
}

impl Cache {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn get_or_compute<K: 'static>(
        &mut self,
        key: &str,
        lifetime: Duration,
        compute: impl FnOnce() -> K,
    ) -> &K {
        let now = Instant::now();

        let new = || Entry {
            data: Box::new(compute()),
            timestamp: now,
        };

        match self.0.entry(key.to_owned()) {
            Occupied(mut occupied) => {
                if now.duration_since(occupied.get().timestamp) > lifetime {
                    occupied.insert(new());
                }
            }
            Vacant(vacant) => {
                vacant.insert(new());
            }
        };

        // we just inserted or updated the value, and its guaranteed to be the correct type
        self.0.get(key).unwrap().data.downcast_ref::<K>().unwrap()
    }

    cached! { all_usages_inlined => (Usage, String, String) }
    cached! { all_measurements => Measurement }
    cached! { all_plant_types => PlantType }
    cached! { all_pot_types => PotType }
    cached! { measurements_by_usage => (Usage, Vec<Measurement>) }
    cached! { usage_counts_by_plant => (i64, String) }
    cached! { usage_counts_by_pot => (i64, String) }

    pub fn measurements_of_usage(&mut self, usage: i32) -> &DBResult<Vec<Measurement>> {
        self.get_or_compute(
            &format!("measurements_of_usage#{usage}"),
            Duration::from_secs(1),
            || open(&url()).and_then(|mut conn| db::measurements_of_usage(usage, &mut conn)),
        )
    }

    pub fn single_usage_inlined(
        &mut self,
        usage: i32,
    ) -> &DBResult<(NaiveDateTime, PlantType, PotType)> {
        self.get_or_compute(
            &format!("single_usage_inlined#{usage}"),
            Duration::from_secs(1),
            || open(&url()).and_then(|mut conn| db::single_usage_inlined(usage, &mut conn)),
        )
    }
}
