use eframe::Frame;
use egui::{Context, Ui};
use egui_plot::{Line, Plot, PlotPoints, Points};
use crate::model::{f, ORIGINAL};

pub struct App {
	euler_points: Vec<[f64; 2]>,
	adams_points: Vec<[f64; 2]>,
	runge_kutta_points: Vec<[f64; 2]>,

	show_euler: bool,
	show_adams: bool,
	show_original: bool,
	show_runge_kutta: bool,
}


impl eframe::App for App {
	fn update(&mut self, ctx: &Context, frame: &mut Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.style_mut().spacing.slider_width = (1200.0);

			ui.checkbox(&mut self.show_euler, "show euler");
			ui.checkbox(&mut self.show_adams, "show adams");
			ui.checkbox(&mut self.show_original, "show original");
			ui.checkbox(&mut self.show_runge_kutta, "show runge kutta");

			self.render_plot(ui);
		});
	}
}

impl Default for App {
	fn default() -> Self {
		let runge_kutta_points = generate_runge_kutta_points();

		Self {
			euler_points: generate_euler_points(),
			adams_points: generate_adams_points(&runge_kutta_points[0..4]),
			// adams_points: generate_adams_points(&ORIGINAL[0..4]),
			runge_kutta_points,

			show_euler: true,
			show_adams: true,
			show_original: true,
			show_runge_kutta: true,
		}
	}
}

impl App {
	fn render_plot(&self, ui: &mut Ui) {
		Plot::new("my_plot").show(ui, |plot_ui| {
			if self.show_euler {
				let euler_line = self.generate_euler_line();
				let euler_points = self.generate_euler_points();

				plot_ui.line(euler_line);
				plot_ui.points(euler_points);
			}

			if self.show_original {
				let original_line = self.generate_original_line();
				let original_points = self.generate_original_points();

				plot_ui.line(original_line);
				plot_ui.points(original_points);
			}

			if self.show_runge_kutta {
				let line = self.generate_runge_kutta_line();
				let points = self.generate_runge_kutta_points();

				plot_ui.line(line);
				plot_ui.points(points);
			}

			if self.show_adams {
				let line = self.generate_adams_line();
				let points = self.generate_adams_points();

				plot_ui.line(line);
				plot_ui.points(points);
			}
		});
	}

	fn generate_euler_line(&self) -> Line {
		Line::new(PlotPoints::new(self.euler_points.clone()))
	}

	fn generate_euler_points(&self) -> Points {
		Points::new(self.euler_points.clone()).radius(6.0)
	}


	fn generate_runge_kutta_line(&self) -> Line {
		Line::new(PlotPoints::new(self.runge_kutta_points.clone()))
	}

	fn generate_runge_kutta_points(&self) -> Points {
		Points::new(self.runge_kutta_points.clone()).radius(6.0)
	}


	fn generate_adams_line(&self) -> Line {
		Line::new(PlotPoints::new(self.adams_points.clone()))
	}

	fn generate_adams_points(&self) -> Points {
		Points::new(self.adams_points.clone()).radius(6.0)
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

	println!("[euler]");
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
	println!("\n\n");

	result
}


fn generate_runge_kutta_points() -> Vec<[f64; 2]> {
	let p = 4;
	let h = 0.05;
	let left = 1.0;
	let right = 1.5;

	let mut x0 = left;
	let mut y0 = 0.0;

	let mut result = vec![[x0, y0]];


	println!("[runge kutta]");
	while x0 <= right {
		let x1 = x0 + h;
		let y1 = runge_kutta(x0, y0, h);

		let y_other = y0 + h / 2.0 * f(x0, y0);
		let y_half = y_other + h / 2.0 * f(x0 + h / 2.0, y_other);
		let r = ((y1 - y_half) / (2.0_f64.powi(p) - 1.0)).abs();
		println!("{:?}: {r}", [x1, y1]);

		result.push([x1, y1]);
		x0 = x1;
		y0 = y1;
	}
	println!("\n\n");

	result
}

fn k1(x: f64, y: f64, h: f64) -> f64 {
	h * f(x, y)
}

fn k2(x: f64, y: f64, h: f64) -> f64 {
	h * f(x + h / 4.0, y + k1(x, y, h) / 4.0)
}

fn k3(x: f64, y: f64, h: f64) -> f64 {
	h * f(x + h / 2.0, y + k2(x, y, h) / 2.0)
}

fn k4(x: f64, y: f64, h: f64) -> f64 {
	h * f(x + h, y + k1(x, y, h) - 2.0 * k2(x, y, h) + 2.0 * k3(x, y, h))
}

fn runge_kutta(x: f64, y: f64, h: f64) -> f64 {
	y + 1.0 / 6.0 * (k1(x, y, h) + 4.0 * k3(x, y, h) + k4(x, y, h))
}




fn generate_adams_points(points: &[[f64; 2]]) -> Vec<[f64; 2]> {
	let p = 4;
	let h = 0.05;
	let left = 1.0;
	let right = 1.5;

	let mut result = points.to_vec();
	let mut points = &result[0..4];

	while points[3][0] <= right {
		let x1 = points[3][0] + h;
		let y1 = adams(points, h);

		result.push([x1, y1]);
		points = &result[result.len() - 4..result.len()];
	}

	result
}


fn adams(points: &[[f64; 2]], h: f64) -> f64 {
	let [x0, y0] = points[3];
	let [x1, y1] = points[2];
	let [x2, y2] = points[1];
	let [x3, y3] = points[0];

	y0 + h / 24.0 * (55.0 * f(x0, y0) - 59.0 * f(x1, y1) + 37.0 * f(x2, y2) - 9.0 * f(x3, y3))
}