//! Bessel functions
//!
//! At this stage, this only implements the function \\(K_0(x)\\), \\(K_1\\),
//! \\(K_2\\) and \\(K_3\\).

use rgsl::{bessel, Value};
use std::f64;

/// Approximation of modified Bessel function \\(K_0(x)\\) for all \\(x \geq
/// 0\\).
pub fn k_0(x: f64) -> f64 {
    if x == 0.0 {
        f64::INFINITY
    } else if x > 7.053_326_979_872_539e2 {
        0.0
    } else {
        match bessel::K0_e(x) {
            (Value::Success, r) => r.val,
            (Value::UnderFlow, _) => 0.0,
            _ => unimplemented!(),
        }
    }
}

/// Approximation of modified Bessel function \\(K_1(x)\\) for all \\(x \geq
/// 0\\).
pub fn k_1(x: f64) -> f64 {
    if x == 0.0 {
        f64::INFINITY
    } else if x > 7.053_326_979_872_539e2 {
        0.0
    } else {
        match bessel::K1_e(x) {
            (Value::Success, r) => r.val,
            (Value::UnderFlow, _) => 0.0,
            _ => unimplemented!(),
        }
    }
}

/// Approximation of modified Bessel function \\(K_2(x)\\) for all \\(x \geq
/// 0\\).
pub fn k_2(x: f64) -> f64 {
    if x == 0.0 {
        f64::INFINITY
    } else if x > 7.053_326_979_872_539e2 {
        0.0
    } else {
        match bessel::Kn_e(2, x) {
            (Value::Success, r) => r.val,
            (Value::UnderFlow, _) => 0.0,
            _ => unimplemented!(),
        }
    }
}

/// Approximation of modified Bessel function \\(K_3(x)\\) for all \\(x \geq
/// 0\\).
pub fn k_3(x: f64) -> f64 {
    if x == 0.0 {
        f64::INFINITY
    } else if x > 7.053_326_979_872_539e2 {
        0.0
    } else {
        match bessel::Kn_e(3, x) {
            (Value::Success, r) => r.val,
            (Value::UnderFlow, _) => 0.0,
            _ => unimplemented!(),
        }
    }
}

/// Approximation of modified Bessel function \\(K_n(x)\\) for all \\(x \geq
/// 0\\).
pub fn k_n(n: i32, x: f64) -> f64 {
    match bessel::Kn_e(n, x) {
        (Value::Success, r) => r.val,
        (Value::UnderFlow, _) => 0.0,
        _ => unimplemented!(),
    }
}

/// Approximation of the ratio \\(K_2(x) / K_1(x)\\) for all \\(x \geq 0\\).
/// This ratio appears as the dilation factor in (inverse) decay rates of
/// particles.
pub fn k_1_on_k_2(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x > 7.053_326_979_872_539e2 {
        1. - 1.5 * x.recip() + 1.875 * x.powi(-2) - 1.875 * x.powi(-3)
            + 1.05469 * x.powi(-4)
            + 1.40625 * x.powi(-5)
            - 7.25098 * x.powi(-6)
            + 21.0938 * x.powi(-7)
            - 58.152 * x.powi(-8)
            + 177.979 * x.powi(-9)
    } else {
        let mut results = [0.0, 0.0];
        match bessel::Kn_array(1, 2, x, &mut results) {
            Value::Success => results[0] / results[1],
            Value::UnderFlow => 0.0,
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utilities::test::*;
    use csv;
    use std::f64;

    #[test]
    fn k_0() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel_k0.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k_0(x);
                approx_eq(n, v, 12.0, 0.0);
            }
        }

        assert_eq!(super::k_0(0.0), f64::INFINITY);
    }

    #[test]
    fn k_1() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel_k1.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k_1(x);
                approx_eq(n, v, 12.0, 0.0);
            }
        }

        assert_eq!(super::k_1(0.0), f64::INFINITY);
    }

    #[test]
    fn k_2() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel_k2.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k_2(x);
                approx_eq(n, v, 12.0, 0.0);
            }
        }

        assert_eq!(super::k_2(0.0), f64::INFINITY);
    }

    #[test]
    fn k_3() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel_k3.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k_3(x);
                approx_eq(n, v, 12.0, 0.0);
            }
        }

        assert_eq!(super::k_3(0.0), f64::INFINITY);
    }

    #[test]
    fn k_1_on_k_2() {
        let mut rdr = csv::Reader::from_path("tests/data/bessel_k1_on_k2.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();
            println!("x = {:.2e}, v = {:.2e}", x, v);

            if !v.is_nan() {
                let n = super::k_1_on_k_2(x);
                approx_eq(n, v, 11.0, 0.0);
            }
        }

        assert_eq!(super::k_1_on_k_2(0.0), 0.0);
    }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod bench {
    use crate::utilities::test::*;
    use csv;
    use test::Bencher;

    #[bench]
    fn k_0(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel_k0.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k_0(x);
                    approx_eq(n, v, 12.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k_1(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel_k1.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k_1(x);
                    approx_eq(n, v, 12.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k_2(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel_k2.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k_2(x);
                    approx_eq(n, v, 12.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k_3(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel_k3.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k_3(x);
                    approx_eq(n, v, 12.0, 0.0);
                }
            }
        });
    }

    #[bench]
    fn k_1_on_k_2(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/bessel_k1_on_k2.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::k_1_on_k_2(x);
                    approx_eq(n, v, 11.0, 0.0);
                }
            }
        });
    }
}
