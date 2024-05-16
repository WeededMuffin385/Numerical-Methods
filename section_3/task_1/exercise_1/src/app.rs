use eframe::Frame;
use egui::{Context, Ui};
use egui_plot::{Line, Plot, PlotPoints, Points};
use crate::model::*;

pub struct App{
	show_lines: bool,
	show_original: bool,
	show_lagrange_2: bool,
	show_lagrange_3: bool,
	show_piecewise_3: bool,
	show_newton: bool,
	points: [[f64; 2]; 5],
	x: f64,
}

impl eframe::App for App {
	fn update(&mut self, ctx: &Context, frame: &mut Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.style_mut().spacing.slider_width = (1200.0);

			let lagrange_2_error = (self.lagrange_2(self.x) - f(self.x)).abs();
			let lagrange_3_error = (self.lagrange_3(self.x) - f(self.x)).abs();
			let newton_error = (self.newton(self.x) - f(self.x)).abs();

			ui.checkbox(&mut self.show_lines, "show lines");
			ui.checkbox(&mut self.show_original, "show original");
			ui.checkbox(&mut self.show_lagrange_2, format!("show lagrange n2: {}", lagrange_2_error));
			ui.checkbox(&mut self.show_lagrange_3, format!("show lagrange n3: {}", lagrange_3_error));
			ui.checkbox(&mut self.show_piecewise_3, format!("show piecewise n3: {}", lagrange_3_error));
			ui.checkbox(&mut self.show_newton, format!("show newton: {}", newton_error));

			ui.add(egui::Slider::new(&mut self.x, self.points[0][0]..=self.points[4][0]).text("x").drag_value_speed(10.0_f64.powi(-5)));


			self.render_plot(ui);
		});
	}
}

impl Default for App {
	fn default() -> Self {
		Self {
			show_lines: true,
			show_original: true,
			show_lagrange_2: true,
			show_lagrange_3: true,
			show_newton: true,
			show_piecewise_3: true,
			points: get_points(),
			x: 0.0,
		}
	}
}

impl App {
	fn render_plot(&self, ui: &mut Ui) {
		Plot::new("my_plot").show(ui, |plot_ui| {
			let nodes = self.generate_nodes();
			plot_ui.points(nodes);

			if self.show_original {
				if self.show_lines {
					let original_line = self.generate_original_line();
					plot_ui.line(original_line);
				}

				let original_point = self.generate_original_point();
				plot_ui.points(original_point);
			}

			if self.show_lagrange_2 {
				if self.show_lines {
					let lagrange_2_line = self.generate_lagrange_2_line();
					plot_ui.line(lagrange_2_line);
				}

				let lagrange_2_point = self.generate_lagrange_2_point();
				plot_ui.points(lagrange_2_point);
			}

			if self.show_lagrange_3 {
				if self.show_lines {
					let lagrange_3_line = self.generate_lagrange_3_line();
					plot_ui.line(lagrange_3_line);
				}

				let lagrange_3_point = self.generate_lagrange_3_point();
				plot_ui.points(lagrange_3_point);
			}

			if self.show_newton {
				if self.show_lines {
					let newton_line = self.newton_line();
					plot_ui.line(newton_line);
				}

				let newton_point = self.newton_point();
				plot_ui.points(newton_point);
			}


			if self.show_piecewise_3 {
				if self.show_lines {
					let line = self.generate_piecewise_3_line();
					plot_ui.line(line);
				}

				let point = self.generate_piecewise_3_point();
				plot_ui.points(point);
			}
		});
	}

	fn generate_original_line(&self) -> Line {
		let amount = 5000;
		let mut points = Vec::with_capacity(amount);

		let step = (B - A) / amount as f64;

		for i in 0..amount {
			let x = A + step * i as f64;
			points.push([x, f(x)]);
		}

		Line::new(PlotPoints::new(points))
	}

	fn generate_original_point(&self) -> Points {
		let points = PlotPoints::new([[self.x, f(self.x)]].to_vec());
		let points = Points::new(points);
		points.radius(5.0)
	}

	fn generate_nodes(&self) -> Points {
		let points = PlotPoints::new(self.points.to_vec());
		let points = Points::new(points);
		points.radius(5.0)
	}

	fn lagrange_2(&self, x: f64) -> f64 {
		let mut points = self.points.clone();
		points.sort_by(|[a, _], [b,_]| {
			(a - x).abs().total_cmp(&(b - x).abs())
		});

		let mut points = Vec::from(&points[0..=1]);
		points.sort_by(|[a,_], [b,_]| a.total_cmp(b));


		let a = points[0][1] * (x - points[1][0]) / (points[0][0] - points[1][0]);
		let b = points[1][1] * (x - points[0][0]) / (points[1][0] - points[0][0]);

		a + b
	}

	fn generate_lagrange_2_line(&self) -> Line {
		Line::new(PlotPoints::new(self.points.to_vec()))
	}

	fn generate_lagrange_2_point(&self) -> Points {
		let points = PlotPoints::new([[self.x, self.lagrange_2(self.x)]].to_vec());
		let points = Points::new(points);
		points.radius(5.0)
	}

	fn lagrange_3(&self, x: f64) -> f64 {
		let mut points = self.points.clone();

		points.sort_by(|[a, _], [b,_]| {
			(a - x).abs().total_cmp(&(b - x).abs())
		});

		let mut points = Vec::from(&points[0..=2]);
		points.sort_by(|[a,_], [b,_]| a.total_cmp(b));

		let a = points[0][1] * (x - points[1][0]) * (x - points[2][0]) / (points[0][0] - points[1][0]) / (points[0][0] - points[2][0]);
		let b = points[1][1] * (x - points[0][0]) * (x - points[2][0]) / (points[1][0] - points[0][0]) / (points[1][0] - points[2][0]);
		let c = points[2][1] * (x - points[0][0]) * (x - points[1][0]) / (points[2][0] - points[0][0]) / (points[2][0] - points[1][0]);

		a + b + c
	}

	fn generate_lagrange_3_point(&self) -> Points {
		let points = PlotPoints::new([[self.x, self.lagrange_3(self.x)]].to_vec());
		let points = Points::new(points);
		points.radius(5.0)
	}

	fn generate_lagrange_3_line(&self) -> Line {
		let amount = 5000;
		let mut points = Vec::with_capacity(amount);

		let step = (B - A) / amount as f64;

		for i in 0..amount {
			let x = A + step * i as f64;
			points.push([x, self.lagrange_3(x)]);
		}

		Line::new(PlotPoints::new(points))
	}

	fn newton(&self, x: f64) -> f64 {
		newton_solve(&self.points, x)
	}

	fn newton_point(&self) -> Points {
		let points = PlotPoints::new([[self.x, self.newton(self.x)]].to_vec());
		let points = Points::new(points);
		points.radius(5.0)
	}

	fn newton_line(&self) -> Line {
		let amount = 1000;
		let mut points = Vec::with_capacity(amount);

		let step = (B - A) / amount as f64;

		for i in 0..amount {
			let x = A + step * i as f64;
			points.push([x, self.newton(x)]);
		}

		Line::new(PlotPoints::new(points))
	}

	fn piecewise_3(&self, x: f64) -> f64 {
		let mut points = self.points.clone().to_vec();

		if x <= 0.0 {
			points = points[0..=2].to_vec();
		} else {
			points = points[2..=4].to_vec();
		}

		let a = points[0][1] * (x - points[1][0]) * (x - points[2][0]) / (points[0][0] - points[1][0]) / (points[0][0] - points[2][0]);
		let b = points[1][1] * (x - points[0][0]) * (x - points[2][0]) / (points[1][0] - points[0][0]) / (points[1][0] - points[2][0]);
		let c = points[2][1] * (x - points[0][0]) * (x - points[1][0]) / (points[2][0] - points[0][0]) / (points[2][0] - points[1][0]);

		a + b + c
	}

	fn generate_piecewise_3_point(&self) -> Points {
		let points = PlotPoints::new([[self.x, self.piecewise_3(self.x)]].to_vec());
		let points = Points::new(points);
		points.radius(5.0)
	}

	fn generate_piecewise_3_line(&self) -> Line {
		let amount = 5000;
		let mut points = Vec::with_capacity(amount);

		let step = (B - A) / amount as f64;

		for i in 0..amount {
			let x = A + step * i as f64;
			points.push([x, self.piecewise_3(x)]);
		}

		Line::new(PlotPoints::new(points))
	}
}

fn newton(points: &[[f64; 2]]) -> f64 {
	if points.len() == 1 {
		return points[0][1];
	}

	let a = newton(&points[1..points.len()]) - newton(&points[0..points.len() - 1]);
	let b = points.last().unwrap()[0] - points.first().unwrap()[0];

	a / b
}

fn newton_solve(points: &[[f64; 2]], x: f64) -> f64 {
	newton(&points[0..=0])
	 + newton(&points[0..=1]) * (x - points[0][0])
	 + newton(&points[0..=2]) * (x - points[0][0]) * (x - points[1][0])
	 + newton(&points[0..=3]) * (x - points[0][0]) * (x - points[1][0]) * (x - points[2][0])
	 + newton(&points[0..=4]) * (x - points[0][0]) * (x - points[1][0]) * (x - points[2][0]) * (x - points[3][0])
}