//! Particle Statistics
//!
//! Compute Bose-Einstein and Fermi-Dirac integrals, in both massive (`$\mu =
//! 0$`) and massless (`$\mu \inR$`) cases as well as normalized to a single
//! massless bosonic degree of freedom for the massive case.

use std::{convert::identity, f64};

mod bose_einstein_massive;
mod bose_einstein_massless;
mod bose_einstein_normalized;
mod fermi_dirac_massive;
mod fermi_dirac_massless;
mod fermi_dirac_normalized;

/// Equilibrium number density of a massless Bose-Einstein particle with
/// chemical potential `$\mu \inR$`.
///
/// The inverse temperature is in units of inverse GeV, and the result is in
/// units of GeV`$^{3}$`.
pub fn bose_einstein_massless(mu: f64, beta: f64) -> f64 {
    beta.powi(-3) * _bose_einstein_massless(mu * beta)
}

approx_fn! {
    () fn _bose_einstein_massless(mod = bose_einstein_massless, type = chebyshev, outer = identity, inner = identity);
}

/// Equilibrium number density of massive Bose-Einstein particle.
///
/// The arguments `$m$` and `$\beta$` must be in units such that `$m \beta$` is
/// unitless, where `$m$` is the mass and `$\beta$` is the inverse temperature.
/// The result is in units of \[m\]`$^{3}$`.
pub fn bose_einstein_massive(m: f64, beta: f64) -> f64 {
    if m == 0.0 {
        // ζ(3) / π² ≅ 0.12179382823357308
        0.121_793_828_233_573_08 * beta.powi(-3)
    } else {
        m.powi(3) * _bose_einstein_massive(m * beta)
    }
}

approx_fn! {
    () fn _bose_einstein_massive(mod = bose_einstein_massive, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Equilibrium number density of massive Bose-Einstein particle normalized to a
/// massless Bose-Einstein particle.
///
/// The arguments `$m$` and `$\beta$` must be in units such that `$m \beta$` is
/// unitless, where `$m$` is the particle's mass and `$\beta$` is the inverse
/// temperature.
pub fn bose_einstein_normalized(m: f64, beta: f64) -> f64 {
    _bose_einstein_normalized(m * beta)
}

approx_fn! {
    () fn _bose_einstein_normalized(mod = bose_einstein_normalized, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Equilibrium number density of a massless Fermi-Dirac particle with chemical
/// potential `$\mu \inR$`.
///
/// The inverse temperature is in units of inverse GeV, and the result is in
/// units of GeV`$^{3}$`.
pub fn fermi_dirac_massless(mu: f64, beta: f64) -> f64 {
    beta.powi(-3) * _fermi_dirac_massless(mu * beta)
}

approx_fn! {
    () fn _fermi_dirac_massless(mod = fermi_dirac_massless, type = chebyshev, outer = identity, inner = identity);
}

/// Equilibrium number density of massive Fermi-Dirac particle.
///
/// The arguments `$m$` and `$\beta$` must be in units such that `$m \beta$` is
/// unitless, where `$m$` is the mass and `$\beta$` is the inverse temperature.  The
/// result is in units of \[m\]`$^{3}$`.
pub fn fermi_dirac_massive(m: f64, beta: f64) -> f64 {
    if m == 0.0 {
        // 3 ζ(3) / 4 π² ≅ 0.09134537117517981
        0.091_345_371_175_179_81 * beta.powi(-3)
    } else {
        m.powi(3) * _fermi_dirac_massive(m * beta)
    }
}

approx_fn! {
    () fn _fermi_dirac_massive(mod = fermi_dirac_massive, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Equilibrium number density of massive Fermi-Dirac particle normalized to a
/// massless Bose-Einstein particle.
///
/// The arguments `$m$` and `$beta$` must be in units such that `$m \beta$` is
/// unitless, where `$m$` is the particle's mass and `$\beta$` is the inverse
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
    use std::{f64, fs::File};

    #[test]
    fn massless() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/particle_physics/statistics/massless.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);

        let f = [super::bose_einstein_massless, super::fermi_dirac_massless];

        let mut i = 1;
        for result in rdr.deserialize() {
            println!("iteration: {}", {
                i += 1;
                i
            });
            let values: [f64; 7] = result?;
            let _m = values[0];
            let mu = values[1];
            let beta = values[2];
            let y = [values[3], values[5]];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(mu, beta);
                    approx_eq(nyi, yi, 8.0, 10f64.powi(-200))?;
                }
            }
        }

        Ok(())
    }

    #[test]
    fn massive() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/particle_physics/statistics/massive.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);

        let f = [
            super::bose_einstein_massive,
            super::bose_einstein_normalized,
            super::fermi_dirac_massive,
            super::fermi_dirac_normalized,
        ];

        for result in rdr.deserialize() {
            let values: [f64; 7] = result?;
            let m = values[0];
            let _mu = values[1];
            let beta = values[2];
            let y = &values[3..];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(m, beta);
                    approx_eq(nyi, yi, 8.0, 10f64.powi(-200))?;
                }
            }
        }

        Ok(())
    }
}
