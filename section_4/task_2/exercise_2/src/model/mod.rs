pub fn f(x: f64, y: f64) -> f64 {
	(x.powi(2) * y.powi(2) - (2.0 * x + 1.0) * y + 1.0) / x
}

pub const ORIGINAL: [[f64; 2]; 11] = [
	[1.,0.],
	[1.05,0.0453515],
	[1.1,0.0826446],
	[1.15,0.113422],
	[1.2,0.138889],
	[1.25,0.16],
	[1.3,0.177515],
	[1.35,0.192044],
	[1.4,0.204082],
	[1.45,0.214031],
	[1.5,0.222222]
];
