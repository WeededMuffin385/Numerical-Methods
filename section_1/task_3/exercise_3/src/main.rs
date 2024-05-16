use plotly::{Plot, Scatter};

fn function_1(x: f64, y: f64) -> f64 {
    2.0 * y - (x + 1.0).cos()
}

fn function_2(x: f64, y: f64) -> f64 {
    x + y.sin() + 0.4
}


fn main() {
    println!("Hello, world!");
    let path = "C:/Users/Mi/Desktop/Курс 2 Семестр 2/ЧМ (ЗАЧЁТ С ОЦЕНКОЙ) Численные методы/Раздел 1/Задача 3/images/2.2.png";

    let mut plot = Plot::new();

    let fragments = 1000;
    let radius = 20.0;
    let step = radius * 2.0 / ((fragments - 1) as f64);

    let mut range = Vec::with_capacity(fragments);
    let mut funct_1 = Vec::with_capacity(fragments);
    let mut funct_2 = Vec::with_capacity(fragments);

    for i in 0..fragments {
        let x = -radius + step * (i as f64);

        range.push(x);
        funct_1.push(function_1(x, 0.0));
        funct_2.push(function_2(x, 0.0))
    }

    let trace_1 = Scatter::new(range.clone(), funct_1);
    let trace_2 = Scatter::new(range.clone(), funct_2);

    plot.add_trace(trace_1);
    plot.add_trace(trace_2);
    plot.show();

    loop {

    }
}
