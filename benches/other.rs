use criterion::{black_box, Criterion};
use rand::prelude::*;
use special_functions::other;

pub fn harmonic_number(c: &mut Criterion) {
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
                black_box(other::harmonic_number(x));
            }
        })
    });
}

pub fn gamma(c: &mut Criterion) {
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
                black_box(other::gamma(x));
            }
        })
    });
}

pub fn binomial(c: &mut Criterion) {
    let mut data: Vec<_> = csv::Reader::from_path("tests/data/other/binomial.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 3] = x.unwrap();
            (x[0] as i32, x[1] as i32)
        })
        .collect();
    data.shuffle(&mut StdRng::seed_from_u64(0x1234_abcd));

    c.bench_function("Binomial", |b| {
        b.iter(|| {
            for &(n, k) in &data {
                black_box(other::binomial(n, k));
            }
        })
    });
}

pub fn polylog(c: &mut Criterion) {
    let mut group = c.benchmark_group("Polylog");

    let mut data: Vec<_> = csv::Reader::from_path("tests/data/other/polylog.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 11] = x.unwrap();
            x[0]
        })
        .collect();
    data.shuffle(&mut StdRng::seed_from_u64(0x1234_abcd));

    for (&name, function) in [
        "li0", "li1", "li2", "li3", "li4", "li5", "li6", "li7", "li8", "li9",
    ]
    .iter()
    .zip(&[
        other::polylog::li0,
        other::polylog::li1,
        other::polylog::li2,
        other::polylog::li3,
        other::polylog::li4,
        other::polylog::li5,
        other::polylog::li6,
        other::polylog::li7,
        other::polylog::li8,
        other::polylog::li9,
    ]) {
        group.bench_function(name, |b| {
            b.iter(|| {
                for &x in &data {
                    black_box(function(x));
                }
            })
        });
    }
}
