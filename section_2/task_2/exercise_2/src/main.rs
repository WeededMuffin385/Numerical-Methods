use nalgebra::{DMatrix, DVector, Dyn, Matrix, OMatrix, SMatrix, VecStorage, Vector};

const A: [f64;49] = [
    11.8336,	0.109449,	0.470703,	0.535582,	0.583178,	0.293942,	0.165154,
    0.601258,	12.3133,	0.771123,	0.778574,	0.0236104,	0.922757,	0.992454,
    0.350409,	0.0450047,	8.52039,	0.633756,	0.642208,	0.389875,	0.664971,
    0.843882,	0.56904,	0.398212,	4.57977,	0.673513,	0.419507,	0.587398,
    0.008335,	0.942441,	0.771263,	0.147503,	15.4716,	0.898747,	0.332963,
    0.204548,	0.839035,	0.250388,	0.238638,	0.616616,	14.1895,	0.404504,
    0.402517,	0.516192,	0.292009,	0.349202,	0.185556,	0.603218,	1.50995
];

const B: [f64;7] = [0.772658, 0.642185, 0.815048, 0.47394, 0.452691, 0.283226, 0.984643];


fn simple_iterations_method() {
    let a = DMatrix::from_vec(7, 7, Vec::from(A)).transpose().normalize();
    let b = DVector::from_vec(Vec::from(B));

    let g = {
        let mut g = a.clone();
        for i in 0..7 {
            for j in 0..7 {
                if i == j {
                    g[(i, j)] = 0.0;
                } else {
                    g[(i, j)] = -(a[(i,j)] / a[(i,i)]);
                }
            }
        }
        g
    };

    let f = {
        let mut f = DVector::<f64>::zeros(7);
        for i in 0..7 {
            f[i] = b[i] / a[(i,i)];
        }
        f
    };

    let epsilon = 0.5 * 10.0_f64.powi(-8);
    let norm = g.norm();
    let epsilon = ((1.0 - norm) / norm) * epsilon;


    let mut iters = 0;
    let mut error = f64::MAX;
    let mut x = g.clone() * f.clone() + f.clone();

    while error > epsilon {
        iters += 1;
        let next = g.clone() * x.clone() + f.clone();
        error = (next.clone() - x).norm();
        x = next;
    }

    println!("x: {x}");
    println!("a*x: {}", a.clone() * x.clone());

    println!("iters: {iters}");
    println!("error: {error}");
    println!("residual vector norm: {}", (b - a*x).norm());
}

fn seidel_method() {
    let a = DMatrix::from_vec(7, 7, Vec::from(A)).transpose();
    let b = DVector::from_vec(Vec::from(B));

    let g = {
        let mut g = a.clone();
        for i in 0..7 {
            for j in 0..7 {
                if i == j {
                    g[(i, j)] = 0.0;
                } else {
                    g[(i, j)] = -(a[(i,j)] / a[(i,i)]);
                }
            }
        }
        g
    };

    let f = {
        let mut f = DVector::<f64>::zeros(7);
        for i in 0..7 {
            f[i] = b[i] / a[(i,i)];
        }
        f
    };

    let epsilon = 0.5 * 10.0_f64.powi(-8);
    let norm = g.norm();
    let epsilon = ((1.0 - norm) / norm) * epsilon;

    let mut iters = 0;
    let mut error = f64::MAX;
    let mut x = g.clone() * f.clone() + f.clone();

    while error > epsilon {
        iters += 1;
        let mut next = x.clone();

        for i in 0..7 {
            next[i] = {
                let s1 = {
                    let mut sum = 0.0;
                    if i >= 1 {
                        for j in 0..i - 1 {
                            sum += g[(i,j)] * next[j];
                        }
                        sum
                    } else {
                        0.0
                    }
                };

                let s2 = {
                    let mut sum = 0.0;
                    for j in i..7 {
                        sum += g[(i,j)] * x[j];
                    }
                    sum
                };

                s1 + s2 + f[i]
            }
        }

        error = (next.clone() - x).norm();
        x = next;
    }

    println!("x: {x}");
    println!("a*x: {}", a.clone() * x.clone());

    println!("iters: {iters}");
    println!("error: {error}");
    println!("residual vector norm: {}", (b - a*x).norm());
}



fn main() {
    let a = DMatrix::from_vec(7, 7, Vec::from(A)).transpose();
    let b = DVector::from_vec(Vec::from(B));

    println!("a: {a}");
    println!("b: {b}");


    println!("\n\n\n simple_iterations_method:");
    simple_iterations_method();
    println!("\n\n\n seidel_method:");
    seidel_method();
}
