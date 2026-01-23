use std::{
    any::Any,
    collections::{HashMap, hash_map::Entry::*},
    time::{Duration, Instant},
};

use chrono::NaiveDateTime;
use eframe::{App, CreationContext};
use egui::{Color32, Sense, Ui, UiBuilder, WidgetText};
use egui_extras::install_image_loaders;

use common::{
    db::{self, DBResult, open, url},
    models::*,
};

mod dashboard;

mod usages;

mod plants;

mod pots;

mod history;

struct Entry {
    data: Box<dyn Any>,
    timestamp: Instant,
}

#[repr(transparent)]
pub struct Cache(HashMap<String, Entry>);

macro_rules! cached {
    ( $name:ident => $t:ty ) => {
        pub fn $name(&mut self) -> &common::db::DBResult<Vec<$t>> {
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

enum Tab {
    Dashboard,
    PlantTypes,
    PotTypes,
    Measurements,
    Usages,
}

pub struct Application {
    tab: Tab,
    cache: Cache,
    usage: Option<i32>,
}

impl Application {
    pub fn new(cx: &CreationContext) -> Self {
        cx.egui_ctx.style_mut(|style| {
            style.visuals = egui::Visuals::light();
        });

        install_image_loaders(&cx.egui_ctx);

        Application {
            tab: Tab::Dashboard,
            cache: Cache::new(),
            usage: None,
        }
    }
}

impl App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.label(egui::RichText::new("Plant Monitor")));

            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Dashboard").clicked() {
                        self.tab = Tab::Dashboard;
                    };
                    if ui.button("Current Plants").clicked() {
                        self.tab = Tab::Usages;
                    };
                    if ui.button("Plant Catalogue").clicked() {
                        self.tab = Tab::PlantTypes;
                    };
                    if ui.button("Pot Catalogue").clicked() {
                        self.tab = Tab::PotTypes;
                    };
                    if ui.button("Event Log").clicked() {
                        self.tab = Tab::Measurements;
                    };
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.tab {
            Tab::Dashboard => dashboard::show(self, ui),
            Tab::Usages => usages::show(&mut self.cache, &mut self.usage, ui),
            Tab::PlantTypes => plants::show(self, ui),
            Tab::PotTypes => pots::show(self, ui),
            Tab::Measurements => history::show(self, ui),
        });
    }
}

pub fn hex(s: &'static str) -> Color32 {
    // this is safe since only literals are allowed and
    // colors are highlighted in my editor
    Color32::from_hex(s).unwrap()
}

pub fn card(ui: &mut Ui, title: &str, content: impl FnOnce(&mut Ui), hover: impl FnOnce(&mut Ui)) {
    let rect = egui::Rect::from_min_size(ui.next_widget_position(), egui::vec2(200., 300.));
    let response = ui.allocate_response(rect.size(), Sense::hover());
    let content_rect = rect.shrink(10.);

    ui.vertical(|ui| {
        if response.hovered() {
            ui.painter().rect_filled(rect, 10., hex("#121212"));
            ui.scope_builder(
                UiBuilder::new()
                    .max_rect(content_rect)
                    .disabled()
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
                |ui| {
                    ui.visuals_mut().override_text_color = Some(hex("#E0E0E0"));
                    ui.heading(title);
                    hover(ui);
                },
            );
        } else {
            ui.painter().rect_filled(rect, 10., hex("#E0E0E0"));

            ui.scope_builder(UiBuilder::new().max_rect(content_rect), |ui| {
                ui.heading(title);
                content(ui);
            });
        }
    });
}

pub fn error_label(ui: &mut Ui, text: impl Into<WidgetText>) {
    ui.vertical_centered(|ui| ui.horizontal_centered(|ui| ui.label(text)));
}
