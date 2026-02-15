use std::{fmt::Display, io};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize, palette::tailwind},
    text::Line,
    widgets::{Tabs, Widget},
};

use common::cache::Cache;

mod dashboard;
mod history;
mod plants;
mod pots;
mod usages;

#[derive(Clone, Copy, Default, Debug)]
enum Tab {
    #[default]
    Dashboard,
    PlantTypes,
    PotTypes,
    Usages,
    Measurements,
}

impl Tab {
    fn next_tab(self) -> Self {
        use Tab::*;
        match self {
            Dashboard => PlantTypes,
            PlantTypes => PotTypes,
            PotTypes => Usages,
            Usages => Measurements,
            Measurements => Dashboard,
        }
    }
    fn previous_tab(self) -> Self {
        use Tab::*;
        match self {
            Dashboard => Measurements,
            PlantTypes => Dashboard,
            PotTypes => PlantTypes,
            Usages => PotTypes,
            Measurements => Usages,
        }
    }
    fn title(self) -> Line<'static> {
        format!("  {self:?}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }
    const fn palette(self) -> tailwind::Palette {
        use Tab::*;
        match self {
            Dashboard => tailwind::BLUE,
            PlantTypes => tailwind::EMERALD,
            PotTypes => tailwind::INDIGO,
            Usages => tailwind::ORANGE,
            Measurements => tailwind::RED,
        }
    }
}

// impl Widget for Tab {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         use Tab::*;
//         match self {
//             Dashboard => render_dashboard(area, buf),
//             PlantTypes => {}
//             PotTypes => {}
//             Usages => {}
//             Measurements => {}
//         }
//     }
// }

impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tab::*;
        let s = match self {
            Dashboard => "Dashboard",
            PlantTypes => "Plant Types",
            PotTypes => "Pot Types",
            Usages => "Usages",
            Measurements => "Measurements",
        };
        f.write_str(s)
    }
}

impl Iterator for Tab {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(std::mem::replace(self, self.next_tab()))
    }
}

#[derive(Default)]
pub struct App {
    running: bool,
    tab: Tab,
    cache: Cache,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            tab: Tab::default(),
            cache: Cache::new(),
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Tab => self.next_tab(),
                    KeyCode::BackTab => self.previous_tab(),
                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }
    fn next_tab(&mut self) {
        self.tab = self.tab.next_tab()
    }
    fn previous_tab(&mut self) {
        self.tab = self.tab.previous_tab()
    }
    fn quit(&mut self) {
        self.running = false
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        self.render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.render_current(inner_area, buf);
        self.render_footer(footer_area, buf);
    }
}

impl App {
    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        "Plant Monitor TUI".bold().render(area, buf);
    }
    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Line::raw("⇥ : switch tab | q : quit")
            .centered()
            .render(area, buf);
    }
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = Tab::Dashboard.take(5).map(Tab::title);
        let highlight = (Color::default(), self.tab.palette().c700);
        Tabs::new(titles)
            .highlight_style(highlight)
            .select(self.tab as usize)
            .padding(" ", " ")
            .divider("::")
            .render(area, buf);
    }
    fn render_current(&mut self, area: Rect, buf: &mut Buffer) {
        use Tab::*;
        match self.tab {
            Dashboard => dashboard::render(self, area, buf),
            PlantTypes => todo!(),
            PotTypes => todo!(),
            Usages => todo!(),
            Measurements => todo!(),
        }
    }
}
