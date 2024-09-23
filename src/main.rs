#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use graphing_calculator::MyApp;

fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Graphing Calculator",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
