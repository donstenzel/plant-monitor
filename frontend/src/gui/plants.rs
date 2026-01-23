use super::{Application, error_label};
use egui::Ui;

pub fn show(app: &mut Application, ui: &mut Ui) {
    match app.cache.all_plant_types() {
        Ok(plants) => {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui_flex::Flex::horizontal()
                    .wrap(true)
                    .grow_items(1.)
                    .show(ui, |flex| {
                        for plant in plants {
                            flex.add_ui(egui_flex::item(), |ui| {
                                super::card(
                                    ui,
                                    &plant.name,
                                    |ui| {
                                        ui.image(&plant.image);
                                    },
                                    |ui| {
                                        ui.label(&plant.description);
                                        ui.add_space(10.);
                                        ui.label(format!("Scientific Name: {}", plant.scientific));
                                        ui.label(format!("Water/Day: {}ml", plant.ml_per_day));
                                        ui.label(format!("Ideal pH: {}", plant.ideal_ph));
                                        ui.label(format!("Ideal Lux: {}", plant.ideal_lux));
                                        ui.label(format!("Ideal EC: {}dS/m", plant.ideal_ec));
                                        ui.label(format!(
                                            "Ideal temperature: {}°C",
                                            plant.ideal_temp
                                        ));
                                        ui.label(format!("Ideal humidity: {}%", plant.ideal_hum));
                                    },
                                );
                            });
                        }
                    });
            });
        }
        Err(e) => error_label(ui, format!("Couldn't get plant types: {e}")),
    }
}
