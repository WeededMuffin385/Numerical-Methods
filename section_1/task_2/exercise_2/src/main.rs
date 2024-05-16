fn function(x: f64) -> f64 {
    (x - 1.0).powi(3) +  0.5 * x.exp()
}

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

struct SolutionSplit {
    current: Point,
    previous: Point,
    iterations: usize,
}

fn find_solution_split(mut left: Point, mut right: Point, epsilon: f64) -> SolutionSplit {
    let epsilon = 2.0 * epsilon;

    let mut iterations = 0;
    let mut measurement_error = f64::MAX;

    while measurement_error > epsilon {
        let center = (left.x + right.x) / 2.0;
        let center_value = function(center);

        if left.y.is_sign_positive() == center_value.is_sign_positive() {
            left = Point {
                x: center,
                y: center_value,
            };
        } else {
            right = Point {
                x: center,
                y: center_value,
            };
        }

        iterations += 1;
        measurement_error = (left.x - right.x).abs();
    }

    return if left.y.abs() < right.y.abs() {
        SolutionSplit{current: left, previous: right, iterations}
    } else {
        SolutionSplit{current: right, previous: left, iterations}
    };
}


struct SolutionNewtonSecant {
    current: Point,
    iterations: usize,
}

fn find_solution_newton_secant(mut current: Point, mut previous: Point, epsilon: f64) -> SolutionNewtonSecant {
    let mut iterations = 0;
    let mut measurement_error = f64::MAX;

    while measurement_error > epsilon {
        let next = current.x - (current.x - previous.x) / (current.y - previous.y) * current.y;
        let next_value = function(next);

        previous = current;
        current = Point {
            x: next,
            y: next_value,
        };

        iterations += 1;
        measurement_error = (current.x - previous.x).abs();
    }


    return SolutionNewtonSecant {
        current,
        iterations,
    };
}


fn main() {
    let left = Point {
        x: 0.0,
        y: function(0.0),
    };

    let right = Point {
        x: 0.5,
        y: function(0.5),
    };


    let solution_a = find_solution_split(left, right, 10.0_f64.powi(-2));
    let solution_b = find_solution_newton_secant(solution_a.current, solution_a.previous, 10.0_f64.powi(-9));

    println!("after split. f(x): {}, iters: {}", solution_a.current.y.abs(), solution_a.iterations);
    println!("after newton. f(x): {}, iters: {}", solution_b.current.y.abs(), solution_b.iterations);
    println!("iters overall: {}", solution_a.iterations + solution_b.iterations)
}
