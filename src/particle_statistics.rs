//! Particle statistics

use approx_fn;
use std::convert::identity;
use std::f64;

mod bose_einstein_massive;
mod bose_einstein_massless;
mod bose_einstein_normalized;
mod fermi_dirac_massive;
mod fermi_dirac_massless;
mod fermi_dirac_normalized;

/// Equilibrium number density of a massless Bose-Einstein particle with
/// chemical potential \\(\mu \leq 0\\).
///
/// The inverse temperature is in units of inverse GeV, and the result is in
/// units of GeV^3.
pub fn bose_einstein_massless(mu: f64, beta: f64) -> f64 {
    beta.powi(-3) * _bose_einstein_massless(-mu * beta)
}

approx_fn! {
    () fn _bose_einstein_massless(mod = bose_einstein_massless, type = chebyshev, outer = identity, inner = identity);
}

/// Equilibrium number density of massive Bose-Einstein particle.
///
/// The arguments `m` and `beta` must be in units such that `m * beta` is
/// unitless, where `m` is the mass and `beta` is the inverse temperature.  The
/// result is in units of `[m^3]`.
pub fn bose_einstein_massive(m: f64, beta: f64) -> f64 {
    m.powi(3) * _bose_einstein_massive(m * beta)
}

approx_fn! {
    () fn _bose_einstein_massive(mod = bose_einstein_massive, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Equilibrium number density of massive Bose-Einstein particle normalized to a
/// massless Bose-Einstein particle.
///
/// The arguments `m` and `beta` must be in units such that `m * beta` is
/// unitless, where `m` is the particle's mass and `beta` is the inverse
/// temperature.
pub fn bose_einstein_normalized(m: f64, beta: f64) -> f64 {
    _bose_einstein_normalized(m * beta)
}

approx_fn! {
    () fn _bose_einstein_normalized(mod = bose_einstein_normalized, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Equilibrium number density of a massless Fermi-Dirac particle with chemical
/// potential \\(\mu \in \mathbb{R}\\).
///
/// The inverse temperature is in units of inverse GeV, and the result is in
/// units of GeV^3.
pub fn fermi_dirac_massless(mu: f64, beta: f64) -> f64 {
    beta.powi(-3) * _fermi_dirac_massless(mu * beta)
}

approx_fn! {
    () fn _fermi_dirac_massless(mod = fermi_dirac_massless, type = chebyshev, outer = identity, inner = identity);
}

/// Equilibrium number density of massive Fermi-Dirac particle.
///
/// The arguments `m` and `beta` must be in units such that `m * beta` is
/// unitless, where `m` is the mass and `beta` is the inverse temperature.  The
/// result is in units of `[m^3]`.
pub fn fermi_dirac_massive(m: f64, beta: f64) -> f64 {
    m.powi(3) * _fermi_dirac_massive(m * beta)
}

approx_fn! {
    () fn _fermi_dirac_massive(mod = fermi_dirac_massive, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Equilibrium number density of massive Fermi-Dirac particle normalized to a
/// massless Bose-Einstein particle.
///
/// The arguments `m` and `beta` must be in units such that `m * beta` is
/// unitless, where `m` is the particle's mass and `beta` is the inverse
/// temperature.
pub fn fermi_dirac_normalized(m: f64, beta: f64) -> f64 {
    _fermi_dirac_normalized(m * beta)
}

approx_fn! {
    () fn _fermi_dirac_normalized(mod = fermi_dirac_normalized, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::f64;

    #[test]
    fn massless() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path("tests/data/particle_statistics/massless.csv")?;

        for result in rdr.deserialize() {
            let (mu, beta, be, fd): (f64, f64, f64, f64) = result?;

            if !be.is_nan() {
                let v = super::bose_einstein_massless(mu, beta);
                println!("n({:e}, {:e}) = {:e} [{:e}]", mu, beta, v, be);
                approx_eq(be, v, 11.7, 10f64.powi(-280));
            }
            if !fd.is_nan() {
                let v = super::fermi_dirac_massless(mu, beta);
                println!("n({:e}, {:e}) = {:e} [{:e}]", mu, beta, v, fd);
                approx_eq(fd, v, 11.7, 10f64.powi(-280));
            }
        }

        Ok(())
    }

    #[test]
    fn massive() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path("tests/data/particle_statistics/massive.csv")?;

        let f = [
            super::bose_einstein_massive,
            super::bose_einstein_normalized,
            super::fermi_dirac_massive,
            super::fermi_dirac_normalized,
        ];

        for result in rdr.deserialize() {
            let values: [f64; 6] = result?;
            let m = values[0];
            let beta = values[1];
            let y = &values[2..];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(m, beta);
                    println!("n[{}]({:e}, {:e}) = {:e} [{:e}]", i, m, beta, yi, nyi);
                    approx_eq(nyi, yi, 10.0, 10f64.powi(-200));
                }
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
    fn massless_bose_einstein(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/particle_statistics/massless.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 3] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &beta in &data {
                test::black_box(super::massless_bose_einstein(beta));
            }
        });

        Ok(())
    }

    #[bench]
    fn bose_einstein(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/particle_statistics/massive.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 6] = x.unwrap();
                (x[0], x[1])
            })
            .collect();

        b.iter(|| {
            for &(m, beta) in &data {
                test::black_box(super::bose_einstein(m, beta));
            }
        });

        Ok(())
    }

    #[bench]
    fn bose_einstein_normalized(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/particle_statistics/massive.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 6] = x.unwrap();
                (x[0], x[1])
            })
            .collect();

        b.iter(|| {
            for &(m, beta) in &data {
                test::black_box(super::bose_einstein_normalized(m, beta));
            }
        });

        Ok(())
    }

    #[bench]
    fn massless_fermi_dirac(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/particle_statistics/massless.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 3] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &beta in &data {
                test::black_box(super::massless_fermi_dirac(beta));
            }
        });

        Ok(())
    }

    #[bench]
    fn fermi_dirac(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/particle_statistics/massive.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 6] = x.unwrap();
                (x[0], x[1])
            })
            .collect();

        b.iter(|| {
            for &(m, beta) in &data {
                test::black_box(super::fermi_dirac(m, beta));
            }
        });

        Ok(())
    }

    #[bench]
    fn fermi_dirac_normalized(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/particle_statistics/massive.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 6] = x.unwrap();
                (x[0], x[1])
            })
            .collect();

        b.iter(|| {
            for &(m, beta) in &data {
                test::black_box(super::fermi_dirac_normalized(m, beta));
            }
        });

        Ok(())
    }
}
