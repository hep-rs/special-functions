//! Bessel functions


use std::convert::identity;

mod i0;
mod i1;
mod i2;
mod i3;
mod i4;
mod i5;
mod i6;
mod i7;
mod i8;
mod i9;

mod j0;
mod j1;
mod j2;
mod j3;
mod j4;
mod j5;
mod j6;
mod j7;
mod j8;
mod j9;

mod k0;
mod k1;
mod k1_on_k2;
mod k2;
mod k3;
mod k4;
mod k5;
mod k6;
mod k7;
mod k8;
mod k9;

mod y0;
mod y1;
mod y2;
mod y3;
mod y4;
mod y5;
mod y6;
mod y7;
mod y8;
mod y9;

approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(I_0(x)\\) for all \\(x > 0\\)."#]
    (pub) fn i0(mod = i0, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(I_1(x)\\) for all \\(x > 0\\)."#]
    (pub) fn i1(mod = i1, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(I_2(x)\\) for all \\(x > 0\\)."#]
    (pub) fn i2(mod = i2, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(I_3(x)\\) for all \\(x > 0\\)."#]
    (pub) fn i3(mod = i3, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(I_4(x)\\) for all \\(x > 0\\)."#]
    (pub) fn i4(mod = i4, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(I_5(x)\\) for all \\(x > 0\\)."#]
    (pub) fn i5(mod = i5, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(I_6(x)\\) for all \\(x > 0\\)."#]
    (pub) fn i6(mod = i6, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(I_7(x)\\) for all \\(x > 0\\)."#]
    (pub) fn i7(mod = i7, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(I_8(x)\\) for all \\(x > 0\\)."#]
    (pub) fn i8(mod = i8, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(I_9(x)\\) for all \\(x > 0\\)."#]
    (pub) fn i9(mod = i9, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(J_0(x)\\) for all \\(x > 0\\)."#]
    (pub) fn j0(mod = j0, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(J_1(x)\\) for all \\(x > 0\\)."#]
    (pub) fn j1(mod = j1, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(J_2(x)\\) for all \\(x > 0\\)."#]
    (pub) fn j2(mod = j2, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(J_3(x)\\) for all \\(x > 0\\)."#]
    (pub) fn j3(mod = j3, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(J_4(x)\\) for all \\(x > 0\\)."#]
    (pub) fn j4(mod = j4, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(J_5(x)\\) for all \\(x > 0\\)."#]
    (pub) fn j5(mod = j5, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(J_6(x)\\) for all \\(x > 0\\)."#]
    (pub) fn j6(mod = j6, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(J_7(x)\\) for all \\(x > 0\\)."#]
    (pub) fn j7(mod = j7, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(J_8(x)\\) for all \\(x > 0\\)."#]
    (pub) fn j8(mod = j8, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(J_9(x)\\) for all \\(x > 0\\)."#]
    (pub) fn j9(mod = j9, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(K_0(x)\\) for all \\(x > 0\\)."#]
    (pub) fn k0(mod = k0, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(K_1(x)\\) for all \\(x > 0\\)."#]
    (pub) fn k1(mod = k1, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(K_2(x)\\) for all \\(x > 0\\)."#]
    (pub) fn k2(mod = k2, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(K_3(x)\\) for all \\(x > 0\\)."#]
    (pub) fn k3(mod = k3, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(K_4(x)\\) for all \\(x > 0\\)."#]
    (pub) fn k4(mod = k4, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(K_5(x)\\) for all \\(x > 0\\)."#]
    (pub) fn k5(mod = k5, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(K_6(x)\\) for all \\(x > 0\\)."#]
    (pub) fn k6(mod = k6, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(K_7(x)\\) for all \\(x > 0\\)."#]
    (pub) fn k7(mod = k7, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(K_8(x)\\) for all \\(x > 0\\)."#]
    (pub) fn k8(mod = k8, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(K_9(x)\\) for all \\(x > 0\\)."#]
    (pub) fn k9(mod = k9, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximatino of the ratio of Bessel function \\(K_1(x) / K_2(x)\\) for all \\(x > 0\\)."#]
    (pub) fn k1_on_k2(mod = k1_on_k2, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(Y_0(x)\\) for all \\(x > 0\\)."#]
    (pub) fn y0(mod = y0, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(Y_1(x)\\) for all \\(x > 0\\)."#]
    (pub) fn y1(mod = y1, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(Y_2(x)\\) for all \\(x > 0\\)."#]
    (pub) fn y2(mod = y2, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(Y_3(x)\\) for all \\(x > 0\\)."#]
    (pub) fn y3(mod = y3, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(Y_4(x)\\) for all \\(x > 0\\)."#]
    (pub) fn y4(mod = y4, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(Y_5(x)\\) for all \\(x > 0\\)."#]
    (pub) fn y5(mod = y5, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(Y_6(x)\\) for all \\(x > 0\\)."#]
    (pub) fn y6(mod = y6, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(Y_7(x)\\) for all \\(x > 0\\)."#]
    (pub) fn y7(mod = y7, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(Y_8(x)\\) for all \\(x > 0\\)."#]
    (pub) fn y8(mod = y8, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function \\(Y_9(x)\\) for all \\(x > 0\\)."#]
    (pub) fn y9(mod = y9, type = chebyshev, outer = identity, inner = identity);
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::f64;

    #[test]
    fn i() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/i.csv")?;
        let f = &[
            super::i0,
            super::i1,
            super::i2,
            super::i3,
            super::i4,
            super::i5,
            super::i6,
            super::i7,
            super::i8,
            super::i9,
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
                    println!("I{}({:e}) = {:e} [{:e}]", i, x, yi, nyi);
                    approx_eq(nyi, yi, 10.0, 10f64.powi(-200));
                }
            }
        }

        Ok(())
    }

    #[test]
    fn j() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/j.csv")?;
        let f = &[
            super::j0,
            super::j1,
            super::j2,
            super::j3,
            super::j4,
            super::j5,
            super::j6,
            super::j7,
            super::j8,
            super::j9,
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
                    println!("J{}({:e}) = {:e} [{:e}]", i, x, yi, nyi);
                    if x < 1e2 {
                        approx_eq(nyi, yi, 10.0, 10f64.powi(-200));
                    } else {
                        // The error for large x is rather large.  Probably due
                        // to error in calculating sin/cos?
                        approx_eq(nyi, yi, 3.5, 10f64.powi(-200));
                    }
                }
            }
        }

        Ok(())
    }

    #[test]
    fn k() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k.csv")?;
        let f = &[
            super::k0,
            super::k1,
            super::k2,
            super::k3,
            super::k4,
            super::k5,
            super::k6,
            super::k7,
            super::k8,
            super::k9,
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
                    println!("K{}({:e}) = {:e} [{:e}]", i, x, yi, nyi);
                    approx_eq(nyi, yi, 10.0, 10f64.powi(-200));
                }
            }
        }

        Ok(())
    }

    #[test]
    fn y() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/y.csv")?;
        let f = &[
            super::y0,
            super::y1,
            super::y2,
            super::y3,
            super::y4,
            super::y5,
            super::y6,
            super::y7,
            super::y8,
            super::y9,
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
                    println!("Y{}({:e}) = {:e} [{:e}]", i, x, yi, nyi);
                    if x < 1e2 {
                        approx_eq(nyi, yi, 10.0, 10f64.powi(-200));
                    } else {
                        // The error for large x is rather large.  Probably due
                        // to error in calculating sin/cos?
                        approx_eq(nyi, yi, 3.5, 10f64.powi(-200));
                    }
                }
            }
        }

        Ok(())
    }

    #[test]
    fn k1_on_k2() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path("tests/data/bessel/k1_on_k2.csv")?;

        for result in rdr.deserialize() {
            let (x, y): (f64, f64) = result?;

            if !y.is_nan() {
                println!("x0 = {:e}", x);
                let ny = super::k1_on_k2(x);
                approx_eq(ny, y, 10.0, 0.0);
            }
        }

        Ok(())
    }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod bench {
    use test::Bencher;

    #[bench]
    fn i0(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/i.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 11] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::i0(x));
            }
        });

        Ok(())
    }

    #[bench]
    fn i9(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/i.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 11] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::i9(x));
            }
        });

        Ok(())
    }

    #[bench]
    fn j0(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/j.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 11] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::j0(x));
            }
        });

        Ok(())
    }

    #[bench]
    fn j9(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/j.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 11] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::j9(x));
            }
        });

        Ok(())
    }

    #[bench]
    fn k0(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/k.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 11] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::k0(x));
            }
        });

        Ok(())
    }

    #[bench]
    fn k9(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/k.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 11] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::k9(x));
            }
        });

        Ok(())
    }

    #[bench]
    fn y0(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/y.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 11] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::y0(x));
            }
        });

        Ok(())
    }

    #[bench]
    fn y9(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/y.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 11] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::y9(x));
            }
        });

        Ok(())
    }

    #[bench]
    fn k1_on_k2(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/bessel/k1_on_k2.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 2] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::k1_on_k2(x));
            }
        });

        Ok(())
    }
}
