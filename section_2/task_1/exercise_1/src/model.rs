const LENGTH: usize = 11;
const X: [f64; LENGTH] = [-1.0, -0.8, -0.6, -0.4, -0.2, 0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
const Y: [f64; LENGTH] = [0.0384615, 0.0588235, 0.1, 0.2, 0.5, 1.0, 0.5, 0.2, 0.1, 0.0588235, 0.0384615];
const D: [f64; LENGTH] = [0.0739645, 0.923078, 2.11765, 6.0, 12.0, 0.0, -12.0, -6.0, -2.11765, -0.923078, -0.0739645];

pub struct Model {
    s: Vec<f64>,
}


impl Model {
    pub fn new() -> Self {
        Self {
            s: solve_s(),
        }
    }

    pub fn solve(&self, t: f64) -> f64 {
        let mut i = 1;
        while t >= X[i] && i < (LENGTH - 1) {
            i += 1;
        }

        let x0 = X[i - 1];
        let y0 = Y[i - 1];
        let s0 = self.s[i - 1];

        let x1 = X[i];
        let y1 = Y[i];
        let s1 = self.s[i];

        let hx = x1 - x0;

        let pl =
            y0 * (t - x1).powi(2) * (2.0 * (t - x0) + hx) / hx.powi(3) +
            y1 * (t - x0).powi(2) * (2.0 * (x1 - t) + hx) / hx.powi(3) +
            s0 * (t - x1).powi(2) * (t - x0) / hx.powi(2) +
            s1 * (t - x0).powi(2) * (t - x1) / hx.powi(2);
        pl
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

fn solve_s() -> Vec<f64> {
    // direct sweep method
    let mut a = vec![1.0; LENGTH];
    let mut b = vec![4.0; LENGTH];
    let mut c = vec![1.0; LENGTH];

    a[LENGTH - 1] = 0.0;
    c[0] = 0.0;

    b[LENGTH - 1] = 1.0;
    b[0] = 1.0;

    let mut alpha = vec![0.0; LENGTH];
    let mut betta = vec![0.0; LENGTH];

    alpha[0] = -c[0]/b[0];
    betta[0] = D[0]/b[0];

    for i in 1..LENGTH - 1 {
        alpha[i] = -c[i]/(b[i] + a[i] * alpha[i - 1]);
        betta[i] = (D[i] - a[i] * betta[i - 1]) / (b[i] + a[i] * alpha[i - 1]);
    }

    // reverse sweep method
    betta[LENGTH - 1] = (D[LENGTH - 1] - a[LENGTH - 1] * betta[LENGTH - 2]) / (b[LENGTH - 1] + a[LENGTH - 1] * alpha[LENGTH - 2]);

    let mut s = vec![0.0; LENGTH];
    s[LENGTH - 1] = betta[LENGTH - 1];

    for i in (0..LENGTH - 1).rev() {
        s[i] = alpha[i] * s[i + 1] + betta[i];
    }

    for i in &s {
        println!("{i}");
    }

    s
}

