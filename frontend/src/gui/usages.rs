use super::{Cache, error_label, hex};
use common::models::Measurement;

use chrono::{TimeZone, Utc};
use egui::{CentralPanel, Rect, ScrollArea, Sense, SidePanel, Stroke, Ui, UiBuilder, vec2};
use egui_flex::{Flex, item};
use egui_plot::{Line, Plot, PlotPoints};

pub fn show(cache: &mut Cache, current: &mut Option<i32>, ui: &mut Ui) {
    const ROW_HEIGHT: f32 = 50.;

    match cache.all_usages_inlined() {
        Ok(rows) => {
            SidePanel::left("usage-list")
                .min_width(150.)
                .show_inside(ui, |ui| {
                    ScrollArea::vertical().show_rows(ui, ROW_HEIGHT, rows.len(), |ui, range| {
                        for (usage, plant, pot) in &rows[range] {
                            let p = ui.next_widget_position();
                            let s = vec2(ui.available_width(), ROW_HEIGHT);

                            let r = Rect::from_min_size(p, s);

                            if ui.allocate_rect(r, Sense::click()).clicked() {
                                *current = Some(usage.id);
                            }

                            if let Some(id) = current
                                && *id == usage.id
                            {
                                ui.painter().rect_filled(r, 0, hex("#BFBFBF80"));
                            }

                            ui.scope_builder(UiBuilder::new().max_rect(r), |ui| {
                                let r = ui.max_rect();

                                ui.painter().line_segment(
                                    [r.left_bottom(), r.right_bottom()],
                                    Stroke::new(1., hex("#adadad80")),
                                );
                                ui.painter().line_segment(
                                    [r.left_top(), r.right_top()],
                                    Stroke::new(1., hex("#adadad80")),
                                );

                                ui.horizontal_centered(|ui| {
                                    ui.label(format!(" #{}", usage.id));
                                    ui.label(format!("Plant: {plant}\nPot: {pot}"));
                                });
                                ui.advance_cursor_after_rect(r);
                            });
                        }
                    });
                });
        }
        Err(e) => {
            error_label(ui, format!("Couldn't get all usages inlined: {e}"));
            return;
        }
    }

    if let Some(id) = current {
        CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                match cache.single_usage_inlined(*id) {
                    Ok((planted, plant, pot)) => {
                        ui.vertical_centered(|ui| {
                            ui.heading(format!("#{id}: {} in {}", plant.name, pot.name));
                            ui.visuals_mut().override_text_color = Some(hex("#ADADAD"));
                            ui.label(format!("Planted @ {planted}"));
                        });

                        ui.columns(2, |uis| {
                            uis[0].vertical_centered(|ui| {
                                ui.heading("Plant Info");
                                ui.label(format!("Scientific name: {}", plant.scientific));
                                ui.label(format!("{}mL water per day", plant.ml_per_day));
                                ui.label(format!("Ideal pH: {}", plant.ideal_ph));
                                ui.label(format!("Ideal humidity: {}%", plant.ideal_hum));
                                ui.label(format!("Ideal EC: {}dS/m", plant.ideal_ec));
                                ui.label(format!("Ideal light level: {} Lux", plant.ideal_lux));
                                ui.label(format!("Ideal temperature: {}°C", plant.ideal_temp));
                            });
                            uis[1].vertical_centered(|ui| {
                                ui.heading("Pot Info");
                                ui.label(format!("Volume: {}L", pot.volume));
                                ui.label(format!("Drainage: {}mL/h", pot.drainage));
                            });
                        });
                    }
                    Err(e) => error_label(ui, format!("Couldn't get active plant data: {e}")),
                }

                ui.add_space(50.);

                match cache.measurements_of_usage(*id) {
                    Ok(measurements) => {
                        let mut hums = vec![];
                        let mut phs = vec![];
                        let mut luxs = vec![];
                        let mut temps = vec![];
                        let mut ecs = vec![];

                        for &Measurement {
                            humidity,
                            temperature,
                            lux,
                            ph,
                            ec,
                            instant,
                            ..
                        } in measurements
                        {
                            hums.push([instant.and_utc().timestamp() as f64, humidity as f64]);
                            phs.push([instant.and_utc().timestamp() as f64, ph as f64]);
                            luxs.push([instant.and_utc().timestamp() as f64, lux as f64]);
                            temps.push([instant.and_utc().timestamp() as f64, temperature as f64]);
                            ecs.push([instant.and_utc().timestamp() as f64, ec as f64]);
                        }

                        ui.vertical_centered(|ui| {
                            Flex::horizontal()
                                .grow_items(1.)
                                .wrap(true)
                                .show(ui, |flex| {
                                    flex.add_ui(item(), |ui| {
                                        plot(ui, "Humidity (%)", hums);
                                    });
                                    flex.add_ui(item(), |ui| {
                                        plot(ui, "pH", phs);
                                    });
                                    flex.add_ui(item(), |ui| {
                                        plot(ui, "Light Level (Lux)", luxs);
                                    });
                                    flex.add_ui(item(), |ui| {
                                        plot(ui, "Temperature (°C)", temps);
                                    });
                                    flex.add_ui(item(), |ui| {
                                        plot(ui, "Electrical Conductivity (dS/m)", ecs);
                                    });
                                });
                        });
                    }
                    Err(e) => {
                        error_label(ui, format!("Couldn't get active plant measurements: {e}"))
                    }
                }
            });
        });
    }
}

fn plot(ui: &mut Ui, y_label: &str, line: Vec<[f64; 2]>) {
    let line = Line::new("humidities", PlotPoints::from(line));

    fn x_fmt(v: f64) -> String {
        Utc.timestamp_opt(v as i64, 0).unwrap().to_string()
    }

    Plot::new(y_label)
        .view_aspect(2.0)
        .width(350.)
        .allow_axis_zoom_drag(false)
        .allow_drag(false)
        .allow_zoom(false)
        .allow_scroll(false)
        .y_axis_label(y_label)
        .x_axis_formatter(|mark, _| x_fmt(mark.value))
        .y_axis_formatter(|mark, _| format!("{:.2}", mark.value))
        .label_formatter(|_, point| format!("[{}]\n{:0.2}", x_fmt(point.x), point.y))
        .show(ui, |plot_ui| {
            plot_ui.line(line);
        });
}
