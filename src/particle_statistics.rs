//! Particle statistics

use approx_fn;
use std::f64::consts::PI;

mod bose_einstein;
mod fermi_dirac;

/// Approximation of the equilibrium number density of massless Bose-Einstein
/// particle with specified chemical potential.
///
/// The arguments `mu` and `beta` must be in units such that `mu * beta` is
/// unitless.
pub fn bose_einstein(mu: f64, beta: f64) -> f64 {
    _bose_einstein(-mu * beta) / (PI * PI * beta.powi(3))
}

approx_fn! {
    () fn _bose_einstein(mod = bose_einstein, type = chebyshev);
}

/// Approximation of the equilibrium number density of massless Fermi-Dirac
/// particle with specified chemical potential.
///
/// The arguments `mu` and `beta` must be in units such that `mu * beta` is
/// unitless.
pub fn fermi_dirac(mu: f64, beta: f64) -> f64 {
    _fermi_dirac(mu * beta) / (PI * PI * beta.powi(3))
}

approx_fn! {
    () fn _fermi_dirac(mod = fermi_dirac, type = chebyshev);
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::f64;

    #[test]
    fn bose_einstein() {
        let mut rdr =
            csv::Reader::from_path("tests/data/particle_statistics/bose_einstein.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::_bose_einstein(x);
                approx_eq(n, v, 12.0, 10f64.powi(-300));
            }
        }
    }

    #[test]
    fn fermi_dirac() {
        let mut rdr =
            csv::Reader::from_path("tests/data/particle_statistics/fermi_dirac.csv").unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::_fermi_dirac(x);
                println!("x0 = {:e}", x);
                println!("v = {:e}", v);
                println!("n = {:e}", n);
                approx_eq(n, v, 12.0, 10f64.powi(-300));
            }
        }
    }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod bench {
    use test::Bencher;

    #[bench]
    fn bose_einstein(b: &mut Bencher) {
        let rdr =
            csv::Reader::from_path("tests/data/particle_statistics/bose_einstein.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::_bose_einstein(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn fermi_dirac(b: &mut Bencher) {
        let rdr = csv::Reader::from_path("tests/data/particle_statistics/fermi_dirac.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::_fermi_dirac(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }
}
