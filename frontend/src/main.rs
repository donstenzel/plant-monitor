mod gui;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Plant Monitor",
        eframe::NativeOptions::default(),
        Box::new(|ctx| Ok(Box::new(gui::Application::new(ctx)))),
    )
}
