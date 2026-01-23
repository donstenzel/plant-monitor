use super::{Application, error_label};
use egui::Ui;

pub fn show(app: &mut Application, ui: &mut Ui) {
    match app.cache.all_pot_types() {
        Ok(pots) => {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui_flex::Flex::horizontal()
                    .wrap(true)
                    .grow_items(1.)
                    .show(ui, |flex| {
                        for pot in pots {
                            flex.add_ui(egui_flex::item(), |ui| {
                                super::card(
                                    ui,
                                    &pot.name,
                                    |ui| {
                                        ui.image(&pot.image);
                                    },
                                    |ui| {
                                        ui.add_space(10.);
                                        ui.label(format!("Volume: {}L", pot.volume));
                                        ui.label(format!("Drainage: {}mL/h", pot.drainage));
                                    },
                                );
                            });
                        }
                    });
            });
        }
        Err(e) => error_label(ui, format!("Couldn't load pot types: {e}")),
    }
}
