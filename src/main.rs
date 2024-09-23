use eframe::Frame;
use egui::Context;
use egui_plot::{Legend, Line, Plot, PlotPoints};

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Graphing Calculator",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let my_plot = Plot::new("My Plot").legend(Legend::default());

            let rect = ui.ctx().screen_rect();

            my_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from_explicit_callback(move |x| x.floor(), .., rect.width() as usize * 2)).name("curve"))
            });
        });
    }
}
