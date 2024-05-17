use std::env::var;
use std::ops::Not;
use std::process::exit;
use eframe::Frame;
use egui::{Context, Ui};
use egui_plot::{Line, Plot, PlotPoints, Points};
use nalgebra::{DMatrix, DVector};
use rand::Rng;
use crate::model::*;



pub struct App {
	random: bool,
	amount: usize,
	points: Vec<[f64; 2]>,
	cubic_spline_coefficients: Vec<f64>,
	parabolic_spline_coefficients: Vec<f64>,


	show_cubic: bool,
	show_parabolic: bool,
	first_derivative: f64,
}


impl eframe::App for App {
	fn update(&mut self, ctx: &Context, frame: &mut Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.style_mut().spacing.slider_width = (1200.0);

			ui.horizontal(|ui|{
				if ui.add(egui::Slider::new(&mut self.amount, 3..=16).text("amount of points")).changed() {
					self.generate_coefficients();
				}
			});

			ui.horizontal(|ui|{
				if ui.add(egui::Slider::new(&mut self.first_derivative, -5.0..=5.0).text("first derivative")).changed() {
					self.generate_coefficients();
				}
			});


			if ui.button("regenerate").clicked() {
				self.generate_coefficients();
			}

			if ui.button("use real derivatives").clicked() {
				self.first_derivative = f_d(self.points[0][0]);
				self.generate_coefficients();
			}

			ui.checkbox(&mut self.random, "generate points in random places");

			ui.checkbox(&mut self.show_cubic, "show cubic");
			ui.checkbox(&mut self.show_parabolic, "show parabolic");

			self.render_plot(ui);
		});
	}
}

impl Default for App {
	fn default() -> Self {
		let amount = 4;
		let random = false;
		let points = generate_points(amount);
		let cubic_spline_coefficients = generate_cubic_spline_coefficients(&points);
		let parabolic_spline_coefficients = generate_parabolic_spline_coefficients(&points, 0.0);

		Self {
			random,
			amount,
			points,
			cubic_spline_coefficients,
			parabolic_spline_coefficients,

			show_cubic: true,
			show_parabolic: true,

			first_derivative: 0.0,
		}
	}
}

impl App {
	fn render_plot(&self, ui: &mut Ui) {
		Plot::new("my_plot").show(ui, |plot_ui| {
			let original_line = self.generate_original_line();
			plot_ui.line(original_line);

			if self.show_parabolic {
				let parabolic_spline_line = self.generate_parabolic_spline_line();
				plot_ui.line(parabolic_spline_line);
			}

			let original_points = self.generate_original_points();
			plot_ui.points(original_points);

			if self.show_cubic {
				let cubic_spline_line = self.generate_cubic_spline_line();
				plot_ui.line(cubic_spline_line);
			}
		});
	}

	fn generate_coefficients(&mut self) {
		if self.random {
			self.points = generate_random_points(self.amount);
		} else {
			self.points = generate_points(self.amount);
		}
		self.cubic_spline_coefficients = generate_cubic_spline_coefficients(&self.points);
		self.parabolic_spline_coefficients = generate_parabolic_spline_coefficients(&self.points, self.first_derivative);
	}

	fn generate_original_line(&self) -> Line {
		Line::new(PlotPoints::new(self.points.to_vec()))
	}

	fn generate_original_points(&self) -> Points {
		Points::new(self.points.clone()).radius(6.0)
	}

	fn parabolic_spline(&self, x: f64) -> f64 {
		for (index, points) in self.points.windows(2).enumerate() {
			let left = points[0][0];
			let right = points[1][0];

			if (x >= left) && (x <= right) {
				let &[a, b, c] = &self.parabolic_spline_coefficients[index * 3 .. (index + 1) * 3] else {panic!()};
				return a * x.powi(2) + b * x + c;
			}
		}

		0.0
	}

	fn cubic_spline(&self, x: f64) -> f64 {
		for (index, points) in self.points.windows(2).enumerate() {
			let left = points[0][0];
			let right = points[1][0];

			if (x >= left) && (x <= right) {
				let &[a, b, c, d] = &self.cubic_spline_coefficients[index * 4 .. (index + 1) * 4] else {panic!()};
				return a * x.powi(3) + b * x.powi(2) + c * x + d;
			}
		}

		0.0
	}

	fn generate_parabolic_spline_line(&self) -> Line {
		let points_amount = 5000;

		let begin = self.points.first().unwrap()[0];
		let end = self.points.last().unwrap()[0];

		let delta = end - begin;
		let step = delta / (points_amount - 1) as f64;

		Line::new(PlotPoints::new((0..points_amount).map(|index| {
			let x = (index as f64) * step + begin;
			let y = self.parabolic_spline(x);
			[x, y]
		}).collect()))
	}

	fn generate_cubic_spline_line(&self) -> Line {
		let points_amount = 5000;

		let begin = self.points.first().unwrap()[0];
		let end = self.points.last().unwrap()[0];

		let delta = end - begin;
		let step = delta / (points_amount - 1) as f64;

		Line::new(PlotPoints::new((0..points_amount).map(|index| {
			let x = (index as f64) * step + begin;
			let y = self.cubic_spline(x);
			[x, y]
		}).collect()))
	}
}

fn generate_points(amount: usize) -> Vec<[f64; 2]> {
	let delta = B - A;
	let step = delta / (amount - 1) as f64;

	(0..amount).map(|index| {
		let x = (index as f64) * step + A;
		let y = f(x);
		[x, y]
	}).collect()
}

fn generate_random_points(amount: usize) -> Vec<[f64; 2]> {
	let mut rng = rand::thread_rng();

	let mut points: Vec<_> = (0..amount - 2).map(|index| {
		let x = rng.gen_range(A..B);
		let y = f(x);
		[x, y]
	}).collect();
	points.push([A, f(A)]);
	points.push([B, f(B)]);
	points.sort_by(|[a, _], [b, _]| a.partial_cmp(b).unwrap());

	points
}

fn generate_parabolic_spline_coefficients(points: &[[f64; 2]], first_derivative: f64) -> Vec<f64> {
	let mut variables_matrix = Vec::default();
	let mut free_matrix = Vec::default();

	let num_unknown_variables = points.windows(2).len();

	for (index, points) in points.windows(2).enumerate() {
		let variables: Vec<_> = points.iter().map(|&[x, y]| vec![vec![0.0; index * 3], vec![x.powi(2), x, 1.0], vec![0.0; (num_unknown_variables - index - 1) * 3]].concat()).collect();
		let free: Vec<_> = points.iter().map(|&[x, y]| y).collect();

		variables_matrix.extend_from_slice(&variables.concat());
		free_matrix.extend_from_slice(&free);
	}

	for (index, &[x, y]) in points[1..points.len() - 1].iter().enumerate() {
		let variables: Vec<_> = vec![vec![0.0; index * 3], vec![2.0 * x, 1.0, 0.0], vec![- 2.0 * x, - 1.0, 0.0], vec![0.0; (num_unknown_variables - index - 2) * 3]].concat();
		let free: Vec<_> = vec![0.0];

		variables_matrix.extend_from_slice(&variables);
		free_matrix.extend_from_slice(&free);
	}

	{
		let [x, y] = points[0];
		variables_matrix.extend_from_slice(&vec![vec![2.0 * x, 1.0, 0.0], vec![0.0; (num_unknown_variables - 1) * 3]].concat());
		free_matrix.extend_from_slice(&vec![first_derivative]);
	}


	let variables_matrix = DMatrix::from_vec(num_unknown_variables * 3, num_unknown_variables * 3, variables_matrix).transpose();
	let free_matrix = DVector::from_vec(free_matrix);
	let result = variables_matrix.try_inverse().unwrap() * free_matrix;

	result.as_slice().to_vec()
}

fn generate_cubic_spline_coefficients(points: &[[f64; 2]]) -> Vec<f64> {
	let mut variables_matrix = Vec::default();
	let mut free_matrix = Vec::default();
	let num_unknown_variables = points.windows(2).len();

	for (index, points) in points.windows(2).enumerate() {
		let variables: Vec<_> = points.iter().map(|&[x, y]| vec![vec![0.0; index * 4], vec![x.powi(3), x.powi(2), x, 1.0], vec![0.0; (num_unknown_variables - index - 1) * 4]].concat()).collect();
		let free: Vec<_> = points.iter().map(|&[x, y]| y).collect();

		variables_matrix.extend_from_slice(&variables.concat());
		free_matrix.extend_from_slice(&free);
	}

	for (index, &[x, y]) in points[1..points.len() - 1].iter().enumerate() {
		let variables: Vec<_> = vec![vec![0.0; index * 4], vec![3.0 * x.powi(2), 2.0 * x, 1.0, 0.0], vec![-3.0 * x.powi(2), -2.0 * x, -1.0, 0.0], vec![0.0; (num_unknown_variables - index - 2) * 4]].concat();
		let free: Vec<_> = vec![0.0];

		variables_matrix.extend_from_slice(&variables);
		free_matrix.extend_from_slice(&free);
	}

	for (index, &[x, y]) in points[1..points.len() - 1].iter().enumerate() {
		let variables: Vec<_> = vec![vec![0.0; index * 4], vec![3.0 * 2.0 * x, 2.0, 0.0, 0.0], vec![-3.0 * 2.0 * x, -2.0, 0.0, 0.0], vec![0.0; (num_unknown_variables - index - 2) * 4]].concat();
		let free: Vec<_> = vec![0.0];

		variables_matrix.extend_from_slice(&variables);
		free_matrix.extend_from_slice(&free);
	}

	{
		let &[x, y] = points.first().unwrap();
		variables_matrix.extend_from_slice(&vec![vec![3.0 * 2.0 * x, 2.0, 0.0, 0.0], vec![0.0; (num_unknown_variables - 1) * 4]].concat());
		free_matrix.extend_from_slice(&vec![0.0]);
	}

	{
		let &[x, y] = points.last().unwrap();
		variables_matrix.extend_from_slice(&vec![vec![0.0; (num_unknown_variables - 1) * 4], vec![3.0 * 2.0 * x, 2.0, 0.0, 0.0]].concat());
		free_matrix.extend_from_slice(&vec![0.0]);
	}

	let variables_matrix = DMatrix::from_vec(num_unknown_variables * 4, num_unknown_variables * 4, variables_matrix).transpose();
	let free_matrix = DVector::from_vec(free_matrix);
	let result = variables_matrix.try_inverse().unwrap() * free_matrix;

	result.as_slice().to_vec()
}