use std::f64::consts::PI;

pub const A: f64 = -PI/4.0;
pub const B: f64 =  PI/4.0;
pub const N: usize = 5;

pub fn f(x: f64) -> f64 {
	-(3.0 * x.powi(2) + x + 3.0) / (0.5 * x.powi(2) + PI / 4.0).tan().powi(3)
}

pub fn get_points() -> [[f64; 2]; N] {
	let step = (B - A) / (N - 1) as f64;
	let mut points = [[0.0, 0.0]; N];
	for (index, value) in points.iter_mut().enumerate() {
		let x = A + step * (index as f64);
		let y = f(x);
		*value = [x, y];
	}
	points
}