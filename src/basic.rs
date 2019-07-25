//! Basic functions.

#[cfg(feature = "nightly")]
#[cfg(test)]
mod bench {
    use rand::{
        distributions::{Distribution, Uniform},
        rngs::StdRng,
        SeedableRng,
    };
    use test::Bencher;

    const DATA_SIZE: usize = 100_000;

    #[bench]
    fn log2(b: &mut Bencher) {
        let data: Vec<f64> = Uniform::new(-1000.0f64, 1000.0)
            .sample_iter(StdRng::seed_from_u64(0x1234abcd))
            .map(|x| 2f64.powf(x))
            .take(DATA_SIZE)
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(x.log2());
            }
        });
    }

    #[bench]
    fn log10(b: &mut Bencher) {
        let data: Vec<f64> = Uniform::new(-1000.0f64, 1000.0)
            .sample_iter(StdRng::seed_from_u64(0x1234abcd))
            .map(|x| 10f64.powf(x))
            .take(DATA_SIZE)
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(x.log10());
            }
        });
    }

    #[bench]
    fn ln(b: &mut Bencher) {
        let data: Vec<f64> = Uniform::new(-1000.0f64, 1000.0)
            .sample_iter(StdRng::seed_from_u64(0x1234abcd))
            .map(|x| x.exp())
            .take(DATA_SIZE)
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(x.ln());
            }
        });
    }

    #[bench]
    fn ln_1p(b: &mut Bencher) {
        let data: Vec<f64> = Uniform::new(-1000.0f64, 1000.0)
            .sample_iter(StdRng::seed_from_u64(0x1234abcd))
            .map(|x| x.exp() - 1.0)
            .take(DATA_SIZE)
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(x.ln_1p());
            }
        });
    }

    #[bench]
    fn sin(b: &mut Bencher) {
        let data: Vec<f64> = Uniform::new(-1000.0f64, 1000.0)
            .sample_iter(StdRng::seed_from_u64(0x1234abcd))
            .take(DATA_SIZE)
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(x.sin());
            }
        });
    }

    #[bench]
    fn cos(b: &mut Bencher) {
        let data: Vec<f64> = Uniform::new(-1000.0f64, 1000.0)
            .sample_iter(StdRng::seed_from_u64(0x1234abcd))
            .take(DATA_SIZE)
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(x.cos());
            }
        });
    }

    #[bench]
    fn sin_cos(b: &mut Bencher) {
        let data: Vec<f64> = Uniform::new(-1000.0f64, 1000.0)
            .sample_iter(StdRng::seed_from_u64(0x1234abcd))
            .take(DATA_SIZE)
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(x.sin_cos());
            }
        });
    }

    #[bench]
    fn exp(b: &mut Bencher) {
        let data: Vec<f64> = Uniform::new(-1000.0f64, 1000.0)
            .sample_iter(StdRng::seed_from_u64(0x1234abcd))
            .take(DATA_SIZE)
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(x.exp());
            }
        });
    }

    #[bench]
    fn pow10(b: &mut Bencher) {
        let data: Vec<f64> = Uniform::new(-1000.0f64, 1000.0)
            .sample_iter(StdRng::seed_from_u64(0x1234abcd))
            .take(DATA_SIZE)
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(10f64.powf(x));
            }
        });
    }

    #[bench]
    fn pow2(b: &mut Bencher) {
        let data: Vec<f64> = Uniform::new(-1000.0f64, 1000.0)
            .sample_iter(StdRng::seed_from_u64(0x1234abcd))
            .take(DATA_SIZE)
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(2f64.powf(x));
            }
        });
    }
}
