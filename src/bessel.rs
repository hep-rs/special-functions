//! Bessel functions

use approx_fn;
use std::convert::identity;

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

approx_fn! {
    #[doc = "Approximation of modified Bessel function \\(K_0(x)\\) for all \\(x > 0\\)."]
    (pub) fn k0(mod = k0, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

approx_fn! {
    #[doc = "Approximation of modified Bessel function \\(K_1(x)\\) for all \\(x > 0\\)."]
    (pub) fn k1(mod = k1, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

approx_fn! {
    #[doc = "Approximation of modified Bessel function \\(K_2(x)\\) for all \\(x > 0\\)."]
    (pub) fn k2(mod = k2, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

approx_fn! {
    #[doc = "Approximation of modified Bessel function \\(K_3(x)\\) for all \\(x > 0\\)."]
    (pub) fn k3(mod = k3, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

approx_fn! {
    #[doc = "Approximation of modified Bessel function \\(K_4(x)\\) for all \\(x > 0\\)."]
    (pub) fn k4(mod = k4, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

approx_fn! {
    #[doc = "Approximation of modified Bessel function \\(K_5(x)\\) for all \\(x > 0\\)."]
    (pub) fn k5(mod = k5, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

approx_fn! {
    #[doc = "Approximation of modified Bessel function \\(K_6(x)\\) for all \\(x > 0\\)."]
    (pub) fn k6(mod = k6, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

approx_fn! {
    #[doc = "Approximation of modified Bessel function \\(K_7(x)\\) for all \\(x > 0\\)."]
    (pub) fn k7(mod = k7, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

approx_fn! {
    #[doc = "Approximation of modified Bessel function \\(K_8(x)\\) for all \\(x > 0\\)."]
    (pub) fn k8(mod = k8, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

approx_fn! {
    #[doc = "Approximation of modified Bessel function \\(K_9(x)\\) for all \\(x > 0\\)."]
    (pub) fn k9(mod = k9, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

approx_fn! {
    #[doc = "Approximatino of the ratio of Bessel function \\(K_1(x) / K_2(x)\\) for all \\(x > 0\\)."]
    (pub) fn k1_on_k2(mod = k1_on_k2, type = chebyshev, outer = identity, inner = f64::ln);
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::f64;

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
