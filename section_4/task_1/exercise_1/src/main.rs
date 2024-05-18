fn f(x: f64, y: f64) -> f64 {
    -y * x.cos() + x.cos() * x.sin()
}

fn main() {
    let h = 0.05;
    let mut point = [0.0, -1.0];

    let n = (0.5 / h) as usize;

    for i in 0..n {
        let next = [
            point[0] + h,
            point[1] + h * f(point[0], point[1]),
        ];

        let next_part = point[1] + h / 2.0 * f(point[0], point[1]);
        let next_half = next_part + h / 2.0 * f(point[0] + h / 2.0, next_part);

        let error = (next[1] - next_half) / (2.0_f64.powi(1) - 1.0);

        point = next;
        println!("{:?} | {error}", point);
    }
}
