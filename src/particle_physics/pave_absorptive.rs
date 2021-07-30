//! Passarino-Veltman coefficient functions.
//!
//! Calculation of the imaginary component of Passarino-Veltman coefficient
//! functions.  The code is generated algorithmically from the Mathematica
//! package [Package-X](https://packagex.hepforge.org/)
//! ([arXiv:1503.01469](https://arxiv.org/abs/1503.01469)).
//!
//! As the general reduction algorithm is not implemented, only the following
//! functions are implemented:
//!
//! - `$\boldsymbol{A}_{\underbrace{0\dots0}_{2r}}$` for all values of `$r$`;
//! - `$\boldsymbol{B}_{\underbrace{0\dots0}_{2r}\underbrace{1\dots1}_{n_1}}$`
//!   for all values of `$r$` and  `$n_1$`;
//! - `$\boldsymbol{C}_{\underbrace{0\dots0}_{2r}\underbrace{1\dots1}_{n_1}\underbrace{2\dots2}_{n_2}}$`
//!   for all combinations of `$r$`, `$n_1$` and `$n_2$` in `$\{0, 1, 2\}$`;
//! - `$\boldsymbol{D}_{\underbrace{0\dots0}_{2r}\underbrace{1\dots1}_{n_1}\underbrace{2\dots2}_{n_2}\underbrace{3\dots3}_{n_3}}$`
//!   for the scalar case and for one of `$r$`, `$n_1$`, `$n_2$` or `$n_3$`
//!   being 1.
//!
//! In all cases, the higher order functions are generally less accurate.
//! Furthermore, parameter space which can lead to numerical instabilities are
//! not accounted explicitly handled at this stage.

mod a;
mod b;
mod c;
mod d;

pub use a::a;
pub use b::b;
pub use c::c;
pub use d::d;

/// Evaluates
///
/// ```math
///   \Re \log\left( \frac{a + b}{a - b} \right)
/// ```
///
/// in a way that is numerically stable when `$b \ll a$` and `$b \gg a$`.
fn log_diff(a: f64, b: f64) -> f64 {
    let (aabs, babs) = (a.abs(), b.abs());
    #[allow(clippy::float_cmp)]
    if aabs == babs {
        a.signum() * b.signum() * f64::INFINITY
    } else if aabs > 5.0 * babs {
        2.0 * (b / a).atanh()
    } else {
        2.0 * (a / b).recip().atan()
    }

    // match x.abs() {
    //     xabs if (xabs - 1.0).abs() < f64::EPSILON => x.signum() * f64::INFINITY,
    //     xabs if xabs < 0.5 => 2.0 * x.atanh(),
    //     xabs if xabs > 2.0 => 2.0 * x.recip().atanh(),
    //     _ => ((a + b) / (a - b)).abs().ln(),
    // }
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::{f64, fs::File};

    #[test]
    #[ignore]
    fn log_diff() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/particle_physics/pave_absorptive/log_diff.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);
        let f = super::log_diff;

        for (row, result) in rdr.deserialize().enumerate() {
            let (a, b, y): (f64, f64, f64) = result?;

            if !y.is_nan() {
                let ny = f(a, b);
                approx_eq(ny, y, 1.0, 10f64.powi(-200)).map_err(|err| {
                    println!(
                        "[{}] log_diff({:e}, {:e}) = {:e} but expected {:e}.",
                        row, a, b, ny, y
                    );
                    err
                })?
            }
        }

        Ok(())
    }
}
