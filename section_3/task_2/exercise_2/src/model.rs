pub const A: f64 = 0.5;
pub const B: f64 = 1.5;

pub fn f(x: f64) -> f64 {
	2.0 * x.powi(2) - 5.0 * x + x.powi(2).sin() * x.atan()
}