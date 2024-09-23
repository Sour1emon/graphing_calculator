mod parser;

use eframe::Frame;
use egui::{Context, TextEdit};
use egui_plot::{Legend, Line, Plot, PlotPoints};

#[derive(Default)]
pub struct MyApp {
    line_text: &'static str
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::left("left_panel").min_width(50.0).show(ctx,|ui| {
            if ui.add(TextEdit::singleline(&mut self.line_text).hint_text("Equation")).changed() {
                println!("Changed!");
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let my_plot = Plot::new("My Plot").legend(Legend::default());

            let rect = ui.ctx().screen_rect();
            
            my_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from_explicit_callback(move |x| x.floor(), .., rect.width() as usize * 2)).name("curve"))
            });
        });
    }
}
