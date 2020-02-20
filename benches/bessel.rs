use criterion::{black_box, Criterion};
use rand::prelude::*;

pub fn bessel_i_sorted(c: &mut Criterion) {
    use special_functions::bessel;
    let mut group = c.benchmark_group("Bessel I [Sorted]");

    let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/i.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 11] = x.unwrap();
            x[0]
        })
        .collect();

    for (&name, function) in ["i0", "i1", "i2", "i3", "i4", "i5", "i6", "i7", "i8", "i9"]
        .iter()
        .zip(&[
            bessel::i0,
            bessel::i1,
            bessel::i2,
            bessel::i3,
            bessel::i4,
            bessel::i5,
            bessel::i6,
            bessel::i7,
            bessel::i8,
            bessel::i9,
        ])
    {
        group.bench_function(name, |b| {
            b.iter(|| {
                for &x in &data {
                    black_box(function(x));
                }
            })
        });
    }
}

pub fn bessel_i_random(c: &mut Criterion) {
    use special_functions::bessel;
    let mut group = c.benchmark_group("Bessel I [Random]");

    let mut data: Vec<_> = csv::Reader::from_path("tests/data/bessel/i.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 11] = x.unwrap();
            x[0]
        })
        .collect();
    data.shuffle(&mut StdRng::seed_from_u64(0x1234_abcd));

    for (&name, function) in ["i0", "i1", "i2", "i3", "i4", "i5", "i6", "i7", "i8", "i9"]
        .iter()
        .zip(&[
            bessel::i0,
            bessel::i1,
            bessel::i2,
            bessel::i3,
            bessel::i4,
            bessel::i5,
            bessel::i6,
            bessel::i7,
            bessel::i8,
            bessel::i9,
        ])
    {
        group.bench_function(name, |b| {
            b.iter(|| {
                for &x in &data {
                    black_box(function(x));
                }
            })
        });
    }
}

pub fn bessel_j_sorted(c: &mut Criterion) {
    use special_functions::bessel;
    let mut group = c.benchmark_group("Bessel J [Sorted]");

    let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/j.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 11] = x.unwrap();
            x[0]
        })
        .collect();

    for (&name, function) in ["j0", "j1", "j2", "j3", "j4", "j5", "j6", "j7", "j8", "j9"]
        .iter()
        .zip(&[
            bessel::j0,
            bessel::j1,
            bessel::j2,
            bessel::j3,
            bessel::j4,
            bessel::j5,
            bessel::j6,
            bessel::j7,
            bessel::j8,
            bessel::j9,
        ])
    {
        group.bench_function(name, |b| {
            b.iter(|| {
                for &x in &data {
                    black_box(function(x));
                }
            })
        });
    }
}

pub fn bessel_j_random(c: &mut Criterion) {
    use special_functions::bessel;
    let mut group = c.benchmark_group("Bessel J [Random]");

    let mut data: Vec<_> = csv::Reader::from_path("tests/data/bessel/j.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 11] = x.unwrap();
            x[0]
        })
        .collect();
    data.shuffle(&mut StdRng::seed_from_u64(0x1234_abcd));

    for (&name, function) in ["j0", "j1", "j2", "j3", "j4", "j5", "j6", "j7", "j8", "j9"]
        .iter()
        .zip(&[
            bessel::j0,
            bessel::j1,
            bessel::j2,
            bessel::j3,
            bessel::j4,
            bessel::j5,
            bessel::j6,
            bessel::j7,
            bessel::j8,
            bessel::j9,
        ])
    {
        group.bench_function(name, |b| {
            b.iter(|| {
                for &x in &data {
                    black_box(function(x));
                }
            })
        });
    }
}

pub fn bessel_k_sorted(c: &mut Criterion) {
    use special_functions::bessel;
    let mut group = c.benchmark_group("Bessel K [Sorted]");

    let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/k.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 11] = x.unwrap();
            x[0]
        })
        .collect();

    for (&name, function) in [
        "k0", "k1", "k2", "k3", "k4", "k5", "k6", "k7", "k8", "k9", "k1_on_k2",
    ]
    .iter()
    .zip(&[
        bessel::k0,
        bessel::k1,
        bessel::k2,
        bessel::k3,
        bessel::k4,
        bessel::k5,
        bessel::k6,
        bessel::k7,
        bessel::k8,
        bessel::k9,
        bessel::k1_on_k2,
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

pub fn bessel_k_random(c: &mut Criterion) {
    use special_functions::bessel;
    let mut group = c.benchmark_group("Bessel K [Random]");

    let mut data: Vec<_> = csv::Reader::from_path("tests/data/bessel/k.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 11] = x.unwrap();
            x[0]
        })
        .collect();
    data.shuffle(&mut StdRng::seed_from_u64(0x1234_abcd));

    for (&name, function) in [
        "k0", "k1", "k2", "k3", "k4", "k5", "k6", "k7", "k8", "k9", "k1_on_k2",
    ]
    .iter()
    .zip(&[
        bessel::k0,
        bessel::k1,
        bessel::k2,
        bessel::k3,
        bessel::k4,
        bessel::k5,
        bessel::k6,
        bessel::k7,
        bessel::k8,
        bessel::k9,
        bessel::k1_on_k2,
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

pub fn bessel_y_sorted(c: &mut Criterion) {
    use special_functions::bessel;
    let mut group = c.benchmark_group("Bessel Y [Sorted]");

    let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/y.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 11] = x.unwrap();
            x[0]
        })
        .collect();

    for (&name, function) in ["y0", "y1", "y2", "y3", "y4", "y5", "y6", "y7", "y8", "y9"]
        .iter()
        .zip(&[
            bessel::y0,
            bessel::y1,
            bessel::y2,
            bessel::y3,
            bessel::y4,
            bessel::y5,
            bessel::y6,
            bessel::y7,
            bessel::y8,
            bessel::y9,
        ])
    {
        group.bench_function(name, |b| {
            b.iter(|| {
                for &x in &data {
                    black_box(function(x));
                }
            })
        });
    }
}

pub fn bessel_y_random(c: &mut Criterion) {
    use special_functions::bessel;
    let mut group = c.benchmark_group("Bessel Y [Random]");

    let mut data: Vec<_> = csv::Reader::from_path("tests/data/bessel/y.csv")
        .unwrap()
        .into_deserialize()
        .map(|x| {
            let x: [f64; 11] = x.unwrap();
            x[0]
        })
        .collect();
    data.shuffle(&mut StdRng::seed_from_u64(0x1234_abcd));

    for (&name, function) in ["y0", "y1", "y2", "y3", "y4", "y5", "y6", "y7", "y8", "y9"]
        .iter()
        .zip(&[
            bessel::y0,
            bessel::y1,
            bessel::y2,
            bessel::y3,
            bessel::y4,
            bessel::y5,
            bessel::y6,
            bessel::y7,
            bessel::y8,
            bessel::y9,
        ])
    {
        group.bench_function(name, |b| {
            b.iter(|| {
                for &x in &data {
                    black_box(function(x));
                }
            })
        });
    }
}
