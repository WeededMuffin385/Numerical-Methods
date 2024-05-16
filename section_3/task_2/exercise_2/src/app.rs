use std::env::var;
use std::ops::Not;
use std::process::exit;
use eframe::Frame;
use egui::{Context, Ui};
use egui_plot::{Line, Plot, PlotPoints};
use nalgebra::{DMatrix, DVector};
use rand::Rng;
use crate::model::*;



pub struct App {
	random: bool,
	amount: usize,
	points: Vec<[f64; 2]>,
	parabolic_spline_coefficients: Vec<f64>,
}


impl eframe::App for App {
	fn update(&mut self, ctx: &Context, frame: &mut Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.style_mut().spacing.slider_width = (1200.0);

			ui.horizontal(|ui|{
				if ui.add(egui::Slider::new(&mut self.amount, 3..=16).text("amount of points")).changed() {
					if self.random {
						self.points = generate_random_points(self.amount);
					} else {
						self.points = generate_points(self.amount);
					}
					self.parabolic_spline_coefficients = generate_parabolic_spline_coefficients(&self.points);
				}
			});

			if ui.button("regenerate").clicked() {
				if self.random {
					self.points = generate_random_points(self.amount);
				} else {
					self.points = generate_points(self.amount);
				}
				self.parabolic_spline_coefficients = generate_parabolic_spline_coefficients(&self.points);
			}

			ui.checkbox(&mut self.random, "generate points in random places");

			self.render_plot(ui);
		});
	}
}

impl Default for App {
	fn default() -> Self {
		let amount = 4;
		let random = false;
		let points = generate_points(amount);
		let parabolic_spline_coefficients = generate_parabolic_spline_coefficients(&points);

		generate_cubic_spline_coefficients(&points);

		Self {
			random,
			amount,
			points,
			parabolic_spline_coefficients,
		}
	}
}

impl App {
	fn render_plot(&self, ui: &mut Ui) {
		Plot::new("my_plot").show(ui, |plot_ui| {
			let original_line = self.generate_original_line();
			plot_ui.line(original_line);

			let parabolic_spline_line = self.generate_parabolic_spline_line();
			plot_ui.line(parabolic_spline_line);
		});
	}

	fn generate_original_line(&self) -> Line {
		Line::new(PlotPoints::new(self.points.to_vec()))
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

	let mut points: Vec<_> = (0..amount).map(|index| {
		let x = rng.gen_range(A..B);
		let y = f(x);
		[x, y]
	}).collect();
	points.push([A, f(A)]);
	points.push([B, f(B)]);
	points.sort_by(|[a, _], [b, _]| a.partial_cmp(b).unwrap());

	points
}

fn generate_parabolic_spline_coefficients(points: &[[f64; 2]]) -> Vec<f64> {
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
		free_matrix.extend_from_slice(&vec![0.0]);
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

	{ // values of functions
		let variables: Vec<_> = points.windows(2).enumerate().map(|(index, points)|{
			let h = points[1][0] - points[0][0];
			vec![vec![0.0; index * 4], vec![h.powi(3), h.powi(2), h, 1.0], vec![0.0; (num_unknown_variables - index - 1) * 4]].concat()
		}).collect();
		let free: Vec<_> = points[1..points.len()].iter().map(|&[_, y]| y).collect();

		println!("{:?}", variables);
		println!("{:?}", free);

		variables_matrix.extend_from_slice(&variables.concat());
		free_matrix.extend_from_slice(&free);
	}

	println!("[DERIVATIVES OF 1ST ORDER]\n\n");


	{ // derivatives of 1-st order
		let variables: Vec<_> = points.windows(2).enumerate().map(|(index, points)|{
			let h = points[1][0] - points[0][0];
			vec![vec![0.0; index * 4], vec![3.0 * h.powi(2), 2.0 * h, 1.0, 0.0], vec![-3.0 * h.powi(2), -2.0 * h, -1.0, 0.0], vec![0.0; (num_unknown_variables - index - 1) * 4]].concat()
		}).collect();
		let free = vec![0.0; variables.len()];

		println!("{:?}", variables);
		println!("{:?}", free);
	}

	exit(0);

	{ // derivatives of 2-nd order
		let variables: Vec<_> = points.windows(2).enumerate().map(|(index, points)|{
			let delta = points[1][0] - points[0][0];
			vec![vec![0.0; index * 4], vec![3.0 * 2.0 * delta.powi(1), 2.0, 0.0, 0.0], vec![0.0; (num_unknown_variables - index - 1) * 4]].concat()
		}).collect();
	}

	exit(0);


	for (index, points) in points.windows(2).enumerate() {
		// let variables: Vec<_> = points.iter().map()


		//let variables: Vec<_> = points.iter().map(|&[x, y]| vec![vec![0.0; index * 3], vec![x.powi(2), x, 1.0], vec![0.0; (num_unknown_variables - index - 1) * 3]].concat()).collect();
		//let free: Vec<_> = points.iter().map(|&[x, y]| y).collect();

		//variables_matrix.extend_from_slice(&variables.concat());
		//free_matrix.extend_from_slice(&free);
	}


	vec![]
}