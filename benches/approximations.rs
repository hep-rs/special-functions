use criterion::{black_box, AxisScale, Criterion, PlotConfiguration};
use rand::prelude::*;
use std::f64;

pub fn polynomial(c: &mut Criterion) {
    use special_functions::approximations;
    let mut group = c.benchmark_group("Polynomial");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for &i in &[
        0, 1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000,
    ] {
        group.bench_function(format!("{}", i), |b| {
            let mut rng = StdRng::seed_from_u64(0x1234_abcd);
            let c: Vec<f64> = (0..i).map(|_| rng.gen()).collect();

            b.iter(|| {
                black_box(approximations::polynomial(1.234, &c));
            })
        });
    }
}

pub fn polynomial_ratio(c: &mut Criterion) {
    use special_functions::approximations;
    let mut group = c.benchmark_group("Polynomial Ratio");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for &i in &[
        0, 1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000,
    ] {
        group.bench_function(format!("{}", i), |b| {
            let mut rng = StdRng::seed_from_u64(0x1234_abcd);
            let ca: Vec<f64> = (0..i).map(|_| rng.gen()).collect();
            let cb: Vec<f64> = (0..i).map(|_| rng.gen()).collect();

            b.iter(|| {
                black_box(approximations::polynomial_ratio(1.234, (&ca, &cb)));
            })
        });
    }
}

pub fn chebyshev(c: &mut Criterion) {
    use special_functions::approximations;
    let mut group = c.benchmark_group("Chebyshev");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for &i in &[
        0, 1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000,
    ] {
        group.bench_function(format!("{}", i), |b| {
            let mut rng = StdRng::seed_from_u64(0x1234_abcd);
            let c: Vec<f64> = (0..i).map(|_| rng.gen()).collect();

            b.iter(|| {
                black_box(approximations::chebyshev(1.234, &c, 0.0, 2.0));
            })
        });
    }
}
