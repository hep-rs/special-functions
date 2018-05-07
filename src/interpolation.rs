//! Interpolation functions

/// Perform linear interpolation on data.
///
/// The data should be provided in an array of the shape `[(x0, y0), (x1, y1),
/// ...]`.  The interpolation is done linearly such that a point half-way in
/// between `x0` and `x1` will yield a value half-way between `y0` and `y1`.
///
/// # Warning
///
/// The slice if input values must be sorted in strictly ascending `x` values.
/// Any repeated values, unordered entries or `NaN` values will result in
/// undefined behaviour.
///
/// # Extrapolation
///
/// This linear interpolation repeats the boundary value for all values outside
/// the domain of the interpolation data.
pub fn linear(data: &[(f64, f64)], x: f64) -> f64 {
    debug_assert!(
        !data.is_empty(),
        "Interpolation data must contain at least one element."
    );
    debug_assert!(
        !x.is_nan(),
        "Interpolation to a NaN value is not supported."
    );

    unsafe {
        match data.binary_search_by(|&(xi, _)| xi.partial_cmp(&x).unwrap()) {
            Ok(idx) => data.get_unchecked(idx).1,
            Err(0) => data.get_unchecked(0).1,
            Err(idx) if idx >= data.len() => data.get_unchecked(idx - 1).1,
            Err(idx) => {
                let &(x0, y0) = data.get_unchecked(idx - 1);
                let &(x1, y1) = data.get_unchecked(idx);
                (y0 * (x1 - x) + y1 * (x - x0)) / (x1 - x0)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::f64;
    use utilities::test::*;

    pub(crate) const DATA: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 1.0), (10.0, 0.0), (20.0, 1.0)];

    #[test]
    fn linear() {
        for i in 0..1_001 {
            let x = i as f64 / 1_000.0;
            let y = i as f64 / 1_000.0;
            approx_eq(super::linear(&DATA, x), y, 10.0, 0.0);
        }

        for i in 1_000..10_001 {
            let x = i as f64 / 1_000.0;
            let y = (10_000 - i) as f64 / 9_000.0;
            approx_eq(super::linear(&DATA, x), y, 10.0, 0.0);
        }

        for i in 10_000..20_001 {
            let x = i as f64 / 1_000.0;
            let y = (i - 10_000) as f64 / 10_000.0;
            approx_eq(super::linear(&DATA, x), y, 10.0, 0.0);
        }

        assert_eq!(super::linear(&DATA, f64::NEG_INFINITY), 0.0);
        assert_eq!(super::linear(&DATA, -100.0), 0.0);
        assert_eq!(super::linear(&DATA, 100.0), 1.0);
        assert_eq!(super::linear(&DATA, f64::INFINITY), 1.0);
    }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod bench {
    use std::f64;
    use test::Bencher;
    use utilities::test::*;

    macro_rules! linear {
        ($f:ident, $n:expr) => {
            #[bench]
            fn $f(b: &mut Bencher) {
                let data: Vec<_> = (0..=$n)
                    .map(|i| {
                        let x = i as f64 / ($n as f64);
                        (x, x)
                    })
                    .collect();
                b.iter(|| {
                    for i in 0..100_001 {
                        let x = i as f64 / 100_000.0;
                        let y = i as f64 / 100_000.0;
                        approx_eq(super::linear(&data, x), y, 10.0, 0.0);
                    }
                });
            }
        }
    }

    linear!(linear_1, 1);
    linear!(linear_10, 10);
    linear!(linear_100, 100);
    linear!(linear_1000, 1_000);
    linear!(linear_10000, 10_000);
    linear!(linear_100000, 100_000);
    linear!(linear_1000000, 1_000_000);
}
