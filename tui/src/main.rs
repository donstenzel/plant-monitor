mod gui;

fn main() -> std::io::Result<()> {
    ratatui::run(|t| gui::App::new().run(t))
}
