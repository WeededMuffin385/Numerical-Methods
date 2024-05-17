

pub const A: f64 = 0.5;
pub const B: f64 = 1.5;

pub fn f(x: f64) -> f64 {
	2.0 * x.powi(2) - 5.0 * x + x.powi(2).sin() * x.atan()
}

pub fn f_d(x: f64) -> f64 {
	4.0 * x - 5.0 + 2.0 * x * x.powi(2).cos() * x.atan() + x.powi(2) / (x.powi(2) + 1.0)
}

pub fn f_dd(x: f64) -> f64 {
	4.0 + 2.0 * x.powi(2).cos() * x.atan() - 4.0 * x.powi(2) * x.powi(2).sin() * x.atan() + 4.0 * x * x.powi(2).cos() / (x.powi(2) + 1.0) - 2.0 * x.powi(2).sin() * x / (x.powi(2) + 1.0).powi(2)
}


/*
pub const A: f64 = 1.0;
pub const B: f64 = 2.2;

pub fn f(x: f64) -> f64 {
	(1.0-x).exp() + x.powi(2).cos()

	// 2.0 * x.powi(2) - 5.0 * x + x.powi(2).sin() * x.atan()
}

//pub fn f_derivative(x: f64) -> f64 {

//}

 */