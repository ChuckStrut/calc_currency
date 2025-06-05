use eframe::{egui, epi};
mod ui;
mod currency;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let app = ui::CalculatorApp::new().await;
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Multi Calculator",
        native_options,
        Box::new(|_cc| Box::new(app)),
    )
}
