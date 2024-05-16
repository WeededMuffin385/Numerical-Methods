use std::collections::LinkedList;
use std::fmt::{Display, Formatter, write};

fn phi(x: f64) -> f64 { x.powi(3) - x.powi(2) + 0.5 * x + 1.0 }
fn function(x: f64) -> f64 {
    x.powi(3) - x.powi(2) - 0.5 * x + 1.0
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} | y: {}", self.x, self.y)
    }
}

struct SolutionSimpleIterations {
    path: LinkedList<Point>,
    current: Point,
    iterations: usize,
}

fn find_solution_simple_iterations(mut current: Point, epsilon: f64) -> SolutionSimpleIterations {
    let corrected_epsilon = 0.1 * epsilon;

    let mut path = LinkedList::new();
    let mut iterations = 0;

    let mut measurement_error = f64::MAX;

    while measurement_error >= corrected_epsilon {
        path.push_back(current);

        let next = phi(current.x);
        let next_value = function(next);

        measurement_error = (current.x - next).abs();
        iterations += 1;

        current = Point {
            x: next,
            y: next_value,
        };
    }

    SolutionSimpleIterations {
        path,
        current,
        iterations,
    }
}

fn main() {
    let current = Point {
        x: -0.5,
        y: function(-0.5),
    };

    let result = find_solution_simple_iterations(current, 10.0_f64.powi(-7));

    let mut i = 1;
    println!("iteration values:");
    for a in result.path {
        println!("{}) {}, ", i, a);
        i += 1;
    }

    println!("f(x): {}, iterations: {}", result.current.y, result.iterations);
}


