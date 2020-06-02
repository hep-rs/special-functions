use criterion::{black_box, Criterion};
use rand::prelude::*;

pub fn harmonic_number(c: &mut Criterion) {
    use special_functions::other::harmonic_number;
    let mut data: Vec<_> = csv::Reader::from_path("tests/data/other/harmonic_number.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 2] = x.unwrap();
            x[0]
        })
        .collect();
    data.shuffle(&mut StdRng::seed_from_u64(0x1234_abcd));

    c.bench_function("Harmonic Number", |b| {
        b.iter(|| {
            for &x in &data {
                black_box(harmonic_number(x));
            }
        })
    });
}

pub fn gamma(c: &mut Criterion) {
    use special_functions::other::gamma;
    let mut data: Vec<_> = csv::Reader::from_path("tests/data/other/gamma.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 2] = x.unwrap();
            x[0]
        })
        .collect();
    data.shuffle(&mut StdRng::seed_from_u64(0x1234_abcd));

    c.bench_function("Gamma", |b| {
        b.iter(|| {
            for &x in &data {
                black_box(gamma(x));
            }
        })
    });
}
