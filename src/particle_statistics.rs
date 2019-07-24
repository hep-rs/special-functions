//! Particle statistics

use approx_fn;
use std::f64::consts::PI;

mod bose_einstein;
mod bose_einstein_normalized;
mod fermi_dirac;
mod fermi_dirac_normalized;

/// Approximation of the equilibrium number density of massless Bose-Einstein
/// particle with specified chemical potential.
///
/// The arguments `mu` and `beta` must be in units such that `mu * beta` is
/// unitless, where `mu` is the chemical potential and `beta` is the inverse
/// temperature.
pub fn bose_einstein(mu: f64, beta: f64) -> f64 {
    _bose_einstein(-mu * beta) / (PI * PI * beta.powi(3))
}

approx_fn! {
    () fn _bose_einstein(mod = bose_einstein, type = chebyshev);
}

/// Approximation of the equilibrium number density of massive Bose-Einstein
/// particle normalized to a massless Bose-Einstein particle.
///
/// The arguments `m` and `beta` must be in units such that `m * beta` is
/// unitless, where `m` is the particle's mass and `beta` is the inverse
/// temperature.
pub fn bose_einstein_normalized(m: f64, beta: f64) -> f64 {
    _bose_einstein_normalized((m * beta).ln())
}

approx_fn! {
    () fn _bose_einstein_normalized(mod = bose_einstein_normalized, type = chebyshev);
}

/// Approximation of the equilibrium number density of massless Fermi-Dirac
/// particle with specified chemical potential.
///
/// The arguments `mu` and `beta` must be in units such that `mu * beta` is
/// unitless, where `mu` is the chemical potential and `beta` is the inverse
/// temperature.
pub fn fermi_dirac(mu: f64, beta: f64) -> f64 {
    _fermi_dirac(mu * beta) / (PI * PI * beta.powi(3))
}

approx_fn! {
    () fn _fermi_dirac(mod = fermi_dirac, type = chebyshev);
}

/// Approximation of the equilibrium number density of massive Fermi-Dirac
/// particle normalized to a massless Bose-Einstein particle.
///
/// The arguments `m` and `beta` must be in units such that `m * beta` is
/// unitless, where `m` is the particle's mass and `beta` is the inverse
/// temperature.
pub fn fermi_dirac_normalized(m: f64, beta: f64) -> f64 {
    _fermi_dirac_normalized((m * beta).ln())
}

approx_fn! {
    () fn _fermi_dirac_normalized(mod = fermi_dirac_normalized, type = chebyshev);
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
    fn bose_einstein_normalized() {
        let mut rdr =
            csv::Reader::from_path("tests/data/particle_statistics/bose_einstein_normalized.csv")
                .unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::bose_einstein_normalized(1.0, x);
                approx_eq(n, v, 6.0, 10f64.powi(-170));
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
                approx_eq(n, v, 12.0, 10f64.powi(-300));
            }
        }
    }

    #[test]
    fn fermi_dirac_normalized() {
        let mut rdr =
            csv::Reader::from_path("tests/data/particle_statistics/fermi_dirac_normalized.csv")
                .unwrap();

        for result in rdr.deserialize() {
            let (x, v): (f64, f64) = result.unwrap();

            if !v.is_nan() {
                let n = super::fermi_dirac_normalized(1.0, x);
                approx_eq(n, v, 6.0, 10f64.powi(-170));
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
                    let n = super::bose_einstein(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn bose_einstein_normalized(b: &mut Bencher) {
        let rdr =
            csv::Reader::from_path("tests/data/particle_statistics/bose_einstein_normalized.csv")
                .unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::bose_einstein_normalized(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn fermi_dirac() {
        let mut rdr =
            csv::Reader::from_path("tests/data/particle_statistics/fermi_dirac.csv").unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::fermi_dirac(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }

    #[bench]
    fn fermi_dirac_normalized() {
        let mut rdr =
            csv::Reader::from_path("tests/data/particle_statistics/fermi_dirac_normalized.csv")
                .unwrap();
        let data: Vec<(f64, f64)> = rdr.into_deserialize().map(|x| x.unwrap()).collect();

        b.iter(|| {
            for &(x, v) in &data {
                if !v.is_nan() {
                    let n = super::fermi_dirac_normalized(x);
                    test::black_box((x, v, n));
                }
            }
        });
    }
}
