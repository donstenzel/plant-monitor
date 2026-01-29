use chrono::{TimeZone as _, Utc};
use egui::{Ui, WidgetText};
use egui_flex::{Flex, item};
use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};

use super::{Application, error_label};
use common::models::Measurement;

fn plot_usages_by_plant(app: &mut Application, ui: &mut Ui) {
    match app.cache.usage_counts_by_plant() {
        Ok(counts) => {
            let bars = counts
                .iter()
                .enumerate()
                .map(|(idx, (count, _))| Bar::new(idx as f64, *count as f64))
                .collect();

            let chart = BarChart::new("chart", bars);

            Plot::new("plant-usages")
                .view_aspect(2.0)
                .width(600.)
                .x_axis_label("Plant Counts")
                .x_axis_formatter(|mark, _| counts[mark.value as usize].1.clone())
                .allow_axis_zoom_drag(false)
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .show(ui, |ui| ui.bar_chart(chart));
        }
        Err(e) => {
            error_label(ui, format!("Couldn't load usages grouped by plants: {e}"));
        }
    }
}

fn plot_usages_by_pot(app: &mut Application, ui: &mut Ui) {
    match app.cache.usage_counts_by_pot() {
        Ok(counts) => {
            let bars = counts
                .iter()
                .enumerate()
                .map(|(idx, (count, _))| Bar::new(idx as f64, *count as f64))
                .collect();

            let chart = BarChart::new("chart", bars);

            Plot::new("pot-usages")
                .view_aspect(2.0)
                .width(600.)
                .x_axis_label("Pot Counts")
                .x_axis_formatter(|mark, _| counts[mark.value as usize].1.clone())
                .allow_axis_zoom_drag(false)
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .show(ui, |ui| ui.bar_chart(chart));
        }
        Err(e) => {
            error_label(ui, format!("Couldn't load usages grouped by pots: {e}"));
        }
    }
}

fn plot_humidities(app: &mut Application, ui: &mut Ui) {
    plot_measurements(app, ui, "Humidity (%)", |m| m.humidity as f64);
}
fn plot_ecs(app: &mut Application, ui: &mut Ui) {
    plot_measurements(app, ui, "Conductivity (dS/m)", |m| m.ec as f64);
}
fn plot_temperatures(app: &mut Application, ui: &mut Ui) {
    plot_measurements(app, ui, "Temperature (°C)", |m| m.temperature as f64);
}
fn plot_lux(app: &mut Application, ui: &mut Ui) {
    plot_measurements(app, ui, "Light Level (Lux)", |m| m.lux as f64);
}
fn plot_ph(app: &mut Application, ui: &mut Ui) {
    plot_measurements(app, ui, "pH", |m| m.ph as f64);
}

fn plot_measurements(
    app: &mut Application,
    ui: &mut Ui,
    y_label: impl Into<WidgetText>,
    point: fn(&Measurement) -> f64,
) {
    match app.cache.measurements_by_usage() {
        Ok(gs) => {
            let lines = gs.iter().map(|(usage, measures)| {
                let ms = PlotPoints::from_iter(
                    measures
                        .iter()
                        .map(|m| [m.instant.and_utc().timestamp() as f64, point(m)]),
                );
                Line::new(format!("#{}", usage.id), ms)
            });

            fn x_fmt(v: f64) -> String {
                Utc.timestamp_opt(v as i64, 0).unwrap().to_string()
            }

            Plot::new(point)
                .view_aspect(2.0)
                .width(600.)
                .allow_axis_zoom_drag(false)
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .y_axis_label(y_label)
                .x_axis_formatter(|mark, _| x_fmt(mark.value))
                .y_axis_formatter(|mark, _| format!("{:.2}", mark.value))
                .label_formatter(|_, point| format!("[{}]\n{:0.2}", x_fmt(point.x), point.y))
                .show(ui, |plot_ui| {
                    for line in lines {
                        plot_ui.line(line);
                    }
                });
        }
        Err(e) => {
            error_label(
                ui,
                format!("Couldn't load measurements grouped by usages: {e}"),
            );
        }
    }
}

pub fn show(app: &mut Application, ui: &mut Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("MEASUREMENT OVERVIEW");
            ui.add_space(20.);
            Flex::horizontal()
                .grow_items(1.)
                .wrap(true)
                .show(ui, |flex| {
                    for f in [
                        plot_humidities,
                        plot_ecs,
                        plot_temperatures,
                        plot_lux,
                        plot_ph,
                    ] {
                        flex.add_ui(item(), |ui| f(app, ui));
                    }
                });
            ui.add_space(40.);
            ui.heading("SHARES");
            ui.add_space(20.);
            Flex::horizontal()
                .grow_items(1.)
                .w_full()
                .wrap(true)
                .show(ui, |flex| {
                    flex.add_ui(item(), |ui| {
                        plot_usages_by_plant(app, ui);
                    });
                    flex.add_ui(item(), |ui| {
                        plot_usages_by_pot(app, ui);
                    });
                });
        });
    });
}
