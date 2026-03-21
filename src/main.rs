mod app;
mod state;
mod theme;
mod typing;
mod ui;
mod words;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("10fastprogramme.rs")
            .with_inner_size([1100.0, 600.0])
            .with_min_inner_size([800.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "10fastprogramme.rs",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
