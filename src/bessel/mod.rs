//! Bessel functions
//!
//! At this stage, this only implements the function \\(K_0(x)\\), \\(K_1\\),
//! \\(K_2\\) and \\(K_3\\).

mod k_coefficients;

use std::f64;
use polynomial::polynomial;

use self::k_coefficients::*;

/// Approximation of modified Bessel function \\(K_0(x)\\) for all \\(x \geq
/// 0\\).
pub fn k_0(x: f64) -> f64 {
    debug_assert!(x >= 0.0, "Argument of BesselK must be positive.");

    if x == 0.0 {
        debug!("Using exact expression at x = 0.");
        return f64::INFINITY;
    } else if x < 1.6 {
        debug!("Using Taylor series around x = 0.");
        let x2 = x.powi(2);
        polynomial(x2, &K0_ZERO) + polynomial(x2, &K0_ZERO_LNX) * x.ln()
    } else if x > 50.0 {
        debug!("Using Taylor series around x = ∞.");
        let xinv = x.powi(-1);
        (-x).exp() / x.sqrt() * polynomial(xinv, &K0_INFINITY)
    } else {
        let xln = x.ln();
        let (i, x_lim) = K0_INTERVALS
            .iter()
            .enumerate()
            .skip_while(|&(_, &x_lim)| x > x_lim)
            .next()
            .expect("The intervals should cover everything between 0.22 and 2.85.");
        debug!("Using minimax polynomial for x < {}.", x_lim);
        polynomial(xln, &K0_MINIMAX_COEFFICIENTS[i]).exp() * x.powf(K0_MINIMAX_DENOMINATOR[i])
    }
}

/// Approximation of modified Bessel function \\(K_1(x)\\) for all \\(x \geq
/// 0\\).
pub fn k_1(x: f64) -> f64 {
    debug_assert!(x >= 0.0, "Argument of BesselK must be positive.");

    if x == 0.0 {
        debug!("Using exact expression at x = 0.");
        return f64::INFINITY;
    } else if x < 1.85 {
        debug!("Using Taylor series around x = 0.");
        let x2 = x.powi(2);
        polynomial(x2, &K1_ZERO) / x + x * polynomial(x2, &K1_ZERO_LNX) * x.ln()
    } else if x > 50.0 {
        debug!("Using Taylor series around x = ∞.");
        let xinv = x.powi(-1);
        (-x).exp() / x.sqrt() * polynomial(xinv, &K1_INFINITY)
    } else {
        let xln = x.ln();
        let (i, x_lim) = K1_INTERVALS
            .iter()
            .enumerate()
            .skip_while(|&(_, &x_lim)| x > x_lim)
            .next()
            .expect("The intervals should cover everything between 0.22 and 2.85.");
        debug!("Using minimax polynomial for x < {}.", x_lim);
        polynomial(xln, &K1_MINIMAX_COEFFICIENTS[i]).exp() * x.powf(K1_MINIMAX_DENOMINATOR[i])
    }
}

/// Approximation of modified Bessel function \\(K_2(x)\\) for all \\(x \geq
/// 0\\).
pub fn k_2(x: f64) -> f64 {
    debug_assert!(x >= 0.0, "Argument of BesselK must be positive.");

    if x == 0.0 {
        debug!("Using exact expression at x = 0.");
        return f64::INFINITY;
    } else if x < 2.1 {
        debug!("Using Taylor series around x = 0.");
        let x2 = x.powi(2);
        polynomial(x2, &K2_ZERO) * x.powi(-2) + x2 * polynomial(x2, &K2_ZERO_LNX) * x.ln()
    } else if x > 50.0 {
        debug!("Using Taylor series around x = ∞.");
        let xinv = x.powi(-1);
        (-x).exp() / x.sqrt() * polynomial(xinv, &K2_INFINITY)
    } else {
        let xln = x.ln();
        let (i, x_lim) = K2_INTERVALS
            .iter()
            .enumerate()
            .skip_while(|&(_, &x_lim)| x > x_lim)
            .next()
            .expect("The intervals should cover everything between 0.22 and 2.85.");
        debug!("Using minimax polynomial for x < {}.", x_lim);
        polynomial(xln, &K2_MINIMAX_COEFFICIENTS[i]).exp() * x.powf(K2_MINIMAX_DENOMINATOR[i])
    }
}

/// Approximation of modified Bessel function \\(K_3(x)\\) for all \\(x \geq
/// 0\\).
pub fn k_3(x: f64) -> f64 {
    debug_assert!(x >= 0.0, "Argument of BesselK must be positive.");

    if x == 0.0 {
        debug!("Using exact expression at x = 0.");
        return f64::INFINITY;
    } else if x < 2.4 {
        debug!("Using Taylor series around x = 0.");
        let x2 = x.powi(2);
        polynomial(x2, &K3_ZERO) * x.powi(-3) + x.powi(3) * polynomial(x2, &K3_ZERO_LNX) * x.ln()
    } else if x > 50.0 {
        debug!("Using Taylor series around x = ∞.");
        let xinv = x.powi(-1);
        (-x).exp() / x.sqrt() * polynomial(xinv, &K3_INFINITY)
    } else {
        let xln = x.ln();
        let (i, x_lim) = K3_INTERVALS
            .iter()
            .enumerate()
            .skip_while(|&(_, &x_lim)| x > x_lim)
            .next()
            .expect("The intervals should cover everything between 0.22 and 2.85.");
        debug!("Using minimax polynomial for x < {}.", x_lim);
        polynomial(xln, &K3_MINIMAX_COEFFICIENTS[i]).exp() * x.powf(K3_MINIMAX_DENOMINATOR[i])
    }
}

#[cfg(test)]
mod test {
    use csv;
    use std::f64;
    use utilities::test::*;

    #[test]
    fn k_0() {
        let mut rdr = csv::Reader::from_path("test/data/bessel_k0.csv").unwrap();

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
        let mut rdr = csv::Reader::from_path("test/data/bessel_k1.csv").unwrap();

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
        let mut rdr = csv::Reader::from_path("test/data/bessel_k2.csv").unwrap();

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
        let mut rdr = csv::Reader::from_path("test/data/bessel_k3.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::k_3(x);
                approx_eq(n, v, 12.0, 0.0);
            }
        }

        assert_eq!(super::k_3(0.0), f64::INFINITY);
    }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod bench {
    use test::Bencher;
    use utilities::test::*;
    use csv;

    #[bench]
    fn k_0(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("test/data/bessel_k0.csv").unwrap();
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
        let rdr = csv::Reader::from_path("test/data/bessel_k1.csv").unwrap();
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
        let rdr = csv::Reader::from_path("test/data/bessel_k2.csv").unwrap();
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
        let rdr = csv::Reader::from_path("test/data/bessel_k3.csv").unwrap();
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
}
