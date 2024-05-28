use eframe::Frame;
use egui::{Context, Ui};
use egui_plot::{Line, Plot, PlotPoints, Points};
use crate::model::{f, ORIGINAL};

pub struct App {
	points: Vec<[f64; 2]>,

	show_euler: bool,
	show_original: bool,
}


impl eframe::App for App {
	fn update(&mut self, ctx: &Context, frame: &mut Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.style_mut().spacing.slider_width = (1200.0);

			ui.checkbox(&mut self.show_euler, "show euler");
			ui.checkbox(&mut self.show_original, "show original");

			self.render_plot(ui);

		});
	}
}

impl Default for App {
	fn default() -> Self {
		Self {
			points: generate_euler_points(),
			show_euler: true,
			show_original: true,
		}
	}
}

impl App {
	fn render_plot(&self, ui: &mut Ui) {
		Plot::new("my_plot").show(ui, |plot_ui| {
			let euler_line = self.generate_euler_line();
			let euler_points = self.generate_euler_points();

			let original_line = self.generate_original_line();
			let original_points = self.generate_original_points();


			if self.show_euler {
				plot_ui.line(euler_line);
				plot_ui.points(euler_points);
			}

			if self.show_original {
				plot_ui.line(original_line);
				plot_ui.points(original_points);
			}
		});
	}

	fn generate_euler_line(&self) -> Line {
		Line::new(PlotPoints::new(self.points.clone()))
	}

	fn generate_euler_points(&self) -> Points {
		Points::new(self.points.clone()).radius(6.0)
	}

	fn generate_original_line(&self) -> Line {
		Line::new(PlotPoints::new(ORIGINAL.to_vec()))
	}

	fn generate_original_points(&self) -> Points {
		Points::new(ORIGINAL.to_vec()).radius(6.0)
	}
}


fn generate_euler_points() -> Vec<[f64; 2]> {
	let p = 2;
	let h = 0.05;
	let left = 1.0;
	let right = 1.5;

	let mut x0 = left;
	let mut y0 = 0.0;

	let mut result = vec![[x0, y0]];

	while x0 <= right {
		let x1 = x0 + h;
		let y1 = y0 + h * f(x0, y0);

		let y_other = y0 + h / 2.0 * f(x0, y0);
		let y_half = y_other + h / 2.0 * f(x0 + h / 2.0, y_other);
		let r = ((y1 - y_half) / (2.0_f64.powi(p) - 1.0)).abs();
		println!("{:?}: {r}", [x1, y1]);

		result.push([x1, y1]);
		x0 = x1;
		y0 = y1;
	}

	result
}
