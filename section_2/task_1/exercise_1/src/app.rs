use eframe::egui::{Context, Ui};
use eframe::{egui, Frame};
use egui_plot::{Line, Plot};
use crate::model::Model;

#[derive(Default)]
pub struct App{
    model: Model,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_plot(ui);
        });
    }
}

impl App {
    fn render_plot(&self, ui: &mut Ui){
        Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| {
            let function = self.generate_function();
            plot_ui.line(function);
        });
    }

    fn generate_function(&self) -> Line {
        let amount = 10000;
        let step = 2.0 / amount as f64;
        let mut points = Vec::with_capacity(amount + 1);

        for i in 0..(amount + 1) {
            let x = -1.0 + (i as f64) * step;
            let y = self.model.solve(x);

            points.push([x, y]);
        }

        Line::new(points)
    }
}