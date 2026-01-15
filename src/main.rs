#![windows_subsystem = "windows"]

mod app;
mod config;
mod mouse;
mod ui;

use eframe::egui::ViewportBuilder;
use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([300.0, 200.0])
            .with_always_on_top(),
        ..Default::default()
    };
    eframe::run_native(
        "Mouse Jiggler",
        options,
        Box::new(|cc| Ok(Box::new(app::MouseJigglerApp::new(cc)))),
    )
}
