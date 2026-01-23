use super::{Application, error_label};
use egui::{Color32, Rect, ScrollArea, TextStyle, Ui, UiBuilder, vec2};

pub fn show(app: &mut Application, ui: &mut Ui) {
    const ROW_HEIGHT: f32 = 30.;
    match app.cache.all_measurements() {
        Ok(measurements) => {
            ScrollArea::vertical().show_rows(ui, ROW_HEIGHT, measurements.len(), |ui, range| {
                for m in &measurements[range] {
                    let p = ui.next_widget_position();
                    let s = vec2(ui.available_width(), ROW_HEIGHT);

                    let rect = Rect::from_min_size(p, s);
                    ui.painter().rect_filled(rect, ROW_HEIGHT, pastel(m.usage));
                    ui.scope_builder(
                        UiBuilder::new().max_rect(rect.shrink2(vec2(10., 0.))),
                        |ui| {
                            let labels = [
                                format!("🌿 #{}", m.usage),
                                format!("💧 {}%", m.humidity),
                                format!("☀  {}°C", m.temperature),
                                format!("💡 {}lx", m.lux),
                                format!("⚡ {}dS/m", m.ec),
                                format!("pH {}", m.ph),
                                format!("🕗 {}", m.instant),
                            ];
                            let width: f32 = labels.iter().map(|l| measure_label(ui, l)).sum();
                            let spacing =
                                (ui.available_width() - width) / (labels.len() - 1) as f32;
                            if spacing > 0. {
                                ui.style_mut().spacing.item_spacing.x = spacing;
                            }
                            ui.horizontal_centered(|ui| {
                                for l in labels {
                                    ui.label(l);
                                }
                            });
                        },
                    );
                    ui.advance_cursor_after_rect(rect);
                }
            });
        }
        Err(e) => error_label(ui, format!("Couldn't load history: {e}")),
    }
}

fn pastel(value: i32) -> Color32 {
    const C: f32 = 0.16;

    let hue = (value as f32 * 0.618034 + 0.21).fract() * 360.0;

    let x = C * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());

    let (r, g, b) = match hue as i32 {
        0..=59 => (C, x, 0.0),
        60..=119 => (x, C, 0.0),
        120..=179 => (0.0, C, x),
        180..=239 => (0.0, x, C),
        240..=299 => (x, 0.0, C),
        _ => (C, 0.0, x),
    };

    let scale = |c: f32| ((c + 0.72) * 255.) as u8;
    Color32::from_rgb(scale(r), scale(g), scale(b))
}

fn measure_label(ui: &mut Ui, text: &str) -> f32 {
    let mut temp_ui = ui.new_child(UiBuilder::new());
    temp_ui.set_clip_rect(Rect::NOTHING);
    let galley = temp_ui.fonts_mut(|f| {
        f.layout_no_wrap(
            text.into(),
            TextStyle::resolve(&TextStyle::Body, ui.style()),
            Color32::default(),
        )
    });
    galley.size().x
}
