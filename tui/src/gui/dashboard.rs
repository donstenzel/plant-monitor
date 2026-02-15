use common::models::Measurement;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    symbols::Marker,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Padding, Widget},
};

use super::App;

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    use Constraint::{Length, Min};
    let vertical = Layout::vertical([Length(1), Min(0)]);
    let [header_area, content_area] = vertical.areas(area);

    render_title(header_area, buf);
    render_content(app, content_area, buf);
}

fn render_content(app: &mut App, area: Rect, buf: &mut Buffer) {
    use Constraint::Fill;

    // let [a1, a2, a3, a4, a5] = if area.width > 150 {
    let horizontal = Layout::horizontal([Fill(1); 2]).spacing(3);

    let vertical = Layout::vertical([Fill(1); 3]).spacing(1);
    let [row1, row2, row3] = vertical.areas(area);

    let [c1r1, c2r1] = horizontal.areas(row1);
    let [c1r2, c2r2] = horizontal.areas(row2);
    let [c12r3] = Layout::horizontal([Constraint::Fill(1)])
        .flex(Flex::SpaceEvenly)
        .areas(row3);
    let c12r3 = c12r3.centered_horizontally(Constraint::Length(c1r1.width));

    let [a1, a2, a3, a4, a5] = [c1r1, c2r1, c1r2, c2r2, c12r3];
    // } else {
    //     let vertical = Layout::vertical([Fill(1); 5]).spacing(1);
    //     vertical.areas(area)
    // };

    render_stat("Humidities (%)", |m| m.humidity as f64)(app, a1, buf);
    render_stat("Temperature (°C)", |m| m.temperature as f64)(app, a2, buf);
    render_stat("Conductivity (dS/m)", |m| m.ec as f64)(app, a3, buf);
    render_stat("Light (lx)", |m| m.lux as f64)(app, a4, buf);
    render_stat("pH", |m| m.ph as f64)(app, a5, buf);
}

fn render_stat(label: &str, stat: fn(&Measurement) -> f64) -> impl Fn(&mut App, Rect, &mut Buffer) {
    move |app: &mut App, area: Rect, buf: &mut Buffer| match app.cache.measurements_by_usage() {
        Ok(gs) => {
            let mut datasets = Vec::new();
            let mut x_bounds = [f64::INFINITY, 0.];
            let mut y_bounds = [f64::INFINITY, 0.];
            for (usage, measures) in gs {
                let mut ms = Vec::new();
                for m in measures {
                    let timestamp = m.instant.and_utc().timestamp();
                    ms.push((timestamp as f64, stat(m)));
                    x_bounds = [
                        x_bounds[0].min(timestamp as f64),
                        x_bounds[1].max(timestamp as f64),
                    ];
                    y_bounds = [y_bounds[0].min(stat(m)), y_bounds[1].max(stat(m))];
                }

                datasets.push((usage, ms));
            }

            let datasets: Vec<_> = datasets
                .iter()
                .enumerate()
                .map(|(i, (u, ms))| {
                    Dataset::default()
                        .name(format!("Plant #{}", u.id))
                        .marker(Marker::Braille)
                        .graph_type(GraphType::Line)
                        .fg(get_color(i))
                        .data(ms)
                })
                .collect();

            let x_axis = Axis::default().title("Timestamp").bounds(x_bounds);
            let y_axis = Axis::default().title(label).bounds(y_bounds);

            let b = Block::new()
                .padding(Padding::new(1, 1, 0, 0))
                .borders(Borders::all());

            let inner = b.inner(area);

            b.render(area, buf);
            Chart::new(datasets)
                .x_axis(x_axis)
                .y_axis(y_axis)
                .hidden_legend_constraints((Constraint::Min(0), Constraint::Min(0)))
                .render(inner, buf);
        }
        Err(e) => format!("Couldn't load measurements grouped by usages: {e}")
            .bold()
            .red()
            .render(area, buf),
    }
}

fn _render_humidities(app: &mut App, area: Rect, buf: &mut Buffer) {
    match app.cache.measurements_by_usage() {
        Ok(gs) => {
            let mut datasets = Vec::new();
            let mut x_bounds = [f64::INFINITY, 0.];
            let mut y_bounds = [f64::INFINITY, 0.];
            for (usage, measures) in gs {
                let mut ms = Vec::new();
                for m in measures {
                    let timestamp = m.instant.and_utc().timestamp();
                    ms.push((timestamp as f64, m.humidity as f64));
                    x_bounds = [
                        x_bounds[0].min(timestamp as f64),
                        x_bounds[1].max(timestamp as f64),
                    ];
                    y_bounds = [
                        y_bounds[0].min(m.humidity as f64),
                        y_bounds[1].max(m.humidity as f64),
                    ];
                }

                datasets.push((usage, ms));
            }

            let datasets: Vec<_> = datasets
                .iter()
                .enumerate()
                .map(|(i, (u, ms))| {
                    Dataset::default()
                        .name(format!("Plant #{}", u.id))
                        .marker(Marker::Braille)
                        .graph_type(GraphType::Line)
                        .fg(get_color(i))
                        .data(ms)
                })
                .collect();

            let x_axis = Axis::default().title("Timestamp").bounds(x_bounds);
            let y_axis = Axis::default().title("Humidity (%)").bounds(y_bounds);

            Chart::new(datasets)
                .x_axis(x_axis)
                .y_axis(y_axis)
                .hidden_legend_constraints((Constraint::Min(0), Constraint::Min(0)))
                .render(area, buf);
        }
        Err(e) => format!("Couldn't load measurements grouped by usages: {e}")
            .bold()
            .red()
            .render(area, buf),
    }
}

fn get_color(i: usize) -> Color {
    use Color::*;
    const N: usize = 14;
    const COLORS: [Color; N] = [
        Red,
        Blue,
        Green,
        Yellow,
        Magenta,
        Cyan,
        White,
        LightRed,
        LightBlue,
        LightGreen,
        LightYellow,
        LightMagenta,
        LightCyan,
        Gray,
    ];
    COLORS[i % N]
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "MEASUREMENT OVERVIEW"
        .bold()
        .underlined()
        .green()
        .into_centered_line()
        .render(area, buf)
}
