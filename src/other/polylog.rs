//! Polylogarithms functions

use approx_fn;
use std::convert::identity;

mod li2;
mod li3;
mod li4;
mod li5;
mod li6;
mod li7;
mod li8;
mod li9;

/// Approximation of polylogarithm function \\(Li_0(x)\\) for all \\(x < 1\\).
pub fn li0(x: f64) -> f64 {
    x / (1.0 - x)
}

/// Approximation of polylogarithm function \\(Li_1(x)\\) for all \\(x < 1\\).
pub fn li1(x: f64) -> f64 {
    -(-x).ln_1p()
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_2(x)\\) for all \\(x < 1\\)."]
    (pub) fn li2(mod = li2, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_3(x)\\) for all \\(x < 1\\)."]
    (pub) fn li3(mod = li3, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_4(x)\\) for all \\(x < 1\\)."]
    (pub) fn li4(mod = li4, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_5(x)\\) for all \\(x < 1\\)."]
    (pub) fn li5(mod = li5, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_6(x)\\) for all \\(x < 1\\)."]
    (pub) fn li6(mod = li6, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_7(x)\\) for all \\(x < 1\\)."]
    (pub) fn li7(mod = li7, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_8(x)\\) for all \\(x < 1\\)."]
    (pub) fn li8(mod = li8, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = "Approximation of polylogarithm function \\(Li_9(x)\\) for all \\(x < 1\\)."]
    (pub) fn li9(mod = li9, type = chebyshev, outer = identity, inner = identity);
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::f64;

    #[test]
    fn li() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path("tests/data/other/polylog.csv")?;
        let f = &[
            super::li0,
            super::li1,
            super::li2,
            super::li3,
            super::li4,
            super::li5,
            super::li6,
            super::li7,
            super::li8,
            super::li9,
        ];

        for result in rdr.deserialize() {
            let values: [f64; 11] = result?;
            let x = values[0];
            let y = &values[1..];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(x);
                    println!("Li{}({:e}) = {:e} [{:e}]", i, x, yi, nyi);
                    if i == 0 {
                        // Although the `i = 0` case is explicit, there's some
                        // error regardless.
                        approx_eq(nyi, yi, 9.3, 0.0);
                    } else {
                        approx_eq(nyi, yi, 11.0, 10f64.powi(-15));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod benches {
    use test::Bencher;

    #[bench]
    fn li0(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/other/polylog.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 11] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::li0(x));
            }
        });

        Ok(())
    }

    #[bench]
    fn li9(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/other/polylog.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 11] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::li9(x));
            }
        });

        Ok(())
    }
}
