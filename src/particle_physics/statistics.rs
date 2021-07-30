//! Particle Statistics
//!
//! Compute Bose-Einstein and Fermi-Dirac integrals, in both massive (`$\mu =
//! 0$`) and massless (`$\mu \inR$`) cases as well as normalized to a single
//! massless bosonic degree of freedom for the massive case.

use std::{convert::identity, f64};

mod bose_einstein_massive;
mod bose_einstein_massless;
mod bose_einstein_normalized_massive;
mod bose_einstein_normalized_massless;
mod fermi_dirac_massive;
mod fermi_dirac_massless;
mod fermi_dirac_normalized_massive;
mod fermi_dirac_normalized_massless;

/// Equilibrium number density of a massless Bose-Einstein particle with
/// chemical potential `$\mu \inR$`.
///
/// The inverse temperature is in units of inverse GeV, and the result is in
/// units of GeV`$^{3}$`.
pub fn bose_einstein_massless(beta: f64, mu: f64) -> f64 {
    beta.powi(-3) * _bose_einstein_massless(mu * beta)
}

approx_fn! {
    fn _bose_einstein_massless(mod = bose_einstein_massless, type = chebyshev, outer = f64::exp, inner = identity);
}

/// Equilibrium number density of massless Bose-Einstein particle with chemical
/// potential `$\mu \inR$` normalized to a massless Bose-Einstein particle.
///
/// The arguments `$mu$` and `$\beta$` must be in units such that `$mu \beta$`
/// is unitless, where `$mu$` is the particle's chemical potential and `$\beta$`
/// is the inverse temperature.
pub fn bose_einstein_normalized_massless(beta: f64, mu: f64) -> f64 {
    _bose_einstein_normalized_massless(mu * beta)
}

approx_fn! {
    fn _bose_einstein_normalized_massless(mod = bose_einstein_normalized_massless, type = chebyshev, outer = f64::exp, inner = identity);
}

/// Equilibrium number density of massive Bose-Einstein particle.
///
/// The arguments `$m$` and `$\beta$` must be in units such that `$m \beta$` is
/// unitless, where `$m$` is the mass and `$\beta$` is the inverse temperature.
/// The result is in units of \[m\]`$^{3}$`.
pub fn bose_einstein_massive(beta: f64, m: f64) -> f64 {
    if m == 0.0 {
        // ζ(3) / π² ≅ 0.12179382823357308
        0.121_793_828_233_573_08 * beta.powi(-3)
    } else {
        m.powi(3) * _bose_einstein_massive(m * beta)
    }
}

approx_fn! {
    fn _bose_einstein_massive(mod = bose_einstein_massive, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Equilibrium number density of massive Bose-Einstein particle normalized to a
/// massless Bose-Einstein particle.
///
/// The arguments `$m$` and `$\beta$` must be in units such that `$m \beta$` is
/// unitless, where `$m$` is the particle's mass and `$\beta$` is the inverse
/// temperature.
pub fn bose_einstein_normalized_massive(beta: f64, m: f64) -> f64 {
    _bose_einstein_normalized_massive(m * beta)
}

approx_fn! {
    fn _bose_einstein_normalized_massive(mod = bose_einstein_normalized_massive, type = chebyshev, outer = identity, inner = f64::ln);
}

/// Equilibrium number density of a massless Fermi-Dirac particle with chemical
/// potential `$\mu \inR$`.
///
/// The inverse temperature is in units of inverse GeV, and the result is in
/// units of GeV`$^{3}$`.
pub fn fermi_dirac_massless(beta: f64, mu: f64) -> f64 {
    beta.powi(-3) * _fermi_dirac_massless(mu * beta)
}

approx_fn! {
    fn _fermi_dirac_massless(mod = fermi_dirac_massless, type = chebyshev, outer = f64::exp, inner = identity);
}

/// Equilibrium number density of massless Fermi-Dirac particle with chemical
/// potential `$\mu \inR$` normalized to a massless Bose-Einstein particle.
///
/// The arguments `$mu$` and `$beta$` must be in units such that `$mu \beta$` is
/// unitless, where `$mu$` is the particle's chemical potential and `$\beta$` is
/// the inverse temperature.
pub fn fermi_dirac_normalized_massless(beta: f64, mu: f64) -> f64 {
    _fermi_dirac_normalized_massless(mu * beta)
}

approx_fn! {
    fn _fermi_dirac_normalized_massless(mod = fermi_dirac_normalized_massless, type = chebyshev, outer = f64::exp, inner = identity);
}

/// Equilibrium number density of massive Fermi-Dirac particle.
///
/// The arguments `$m$` and `$\beta$` must be in units such that `$m \beta$` is
/// unitless, where `$m$` is the mass and `$\beta$` is the inverse temperature.  The
/// result is in units of \[m\]`$^{3}$`.
pub fn fermi_dirac_massive(beta: f64, m: f64) -> f64 {
    if m == 0.0 {
        // 3 ζ(3) / 4 π² ≅ 0.09134537117517981
        0.091_345_371_175_179_81 * beta.powi(-3)
    } else {
        m.powi(3) * _fermi_dirac_massive(m * beta)
    }
}

approx_fn! {
    fn _fermi_dirac_massive(mod = fermi_dirac_massive, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Equilibrium number density of massive Fermi-Dirac particle normalized to a
/// massless Bose-Einstein particle.
///
/// The arguments `$m$` and `$beta$` must be in units such that `$m \beta$` is
/// unitless, where `$m$` is the particle's mass and `$\beta$` is the inverse
/// temperature.
pub fn fermi_dirac_normalized_massive(beta: f64, m: f64) -> f64 {
    _fermi_dirac_normalized_massive(m * beta)
}

approx_fn! {
    fn _fermi_dirac_normalized_massive(mod = fermi_dirac_normalized_massive, type = chebyshev, outer = identity, inner = f64::ln);
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::{f64, fs::File};

    #[test]
    fn massless() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/particle_physics/statistics/massless.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);

        let f = [
            super::bose_einstein_massless,
            super::bose_einstein_normalized_massless,
            super::fermi_dirac_massless,
            super::fermi_dirac_normalized_massless,
        ];

        for (row, result) in rdr.deserialize().enumerate() {
            let values: [f64; 7] = result?;
            let beta = values[0];
            let m = values[1];
            let mu = values[2];
            let y = &values[3..];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(beta, mu);
                    approx_eq(nyi, yi, 7.0, 10f64.powi(-300)).map_err(|err| {
                        println!(
                            "[{}] N{}(beta = {:e}, m = {:e}, mu = {:e}) = {:e} but expected {:e}",
                            row, i, beta, m, mu, nyi, yi
                        );
                        err
                    })?;
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
            super::bose_einstein_normalized_massive,
            super::fermi_dirac_massive,
            super::fermi_dirac_normalized_massive,
        ];

        for (row, result) in rdr.deserialize().enumerate() {
            let values: [f64; 7] = result?;
            let beta = values[0];
            let m = values[1];
            let mu = values[2];
            let y = &values[3..];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(beta, m);
                    approx_eq(nyi, yi, 7.0, 10f64.powi(-300)).map_err(|err| {
                        print!(
                            "[{}] N{}(beta = {:e}, m = {:e}, mu = {:e}) = {:e} but expected {:e}",
                            row, i, beta, m, mu, nyi, yi
                        );
                        err
                    })?;
                }
            }
        }

        Ok(())
    }
}
