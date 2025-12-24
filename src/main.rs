use eframe::{egui, epi};
use egui_extras::{TableBuilder, Column};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)), // Will change later
        ..Default::default()
    };
    eframe::run_native(
        "Rust LSOF GUI",
        options,
        ..
}
