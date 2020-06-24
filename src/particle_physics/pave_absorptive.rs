//! Passarino-Veltman functions.
//!
//! Absorptive part of Passarino-Veltman functions.

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
/// \\begin{equation}
///   \log\left( \frac{a + b}{a - b} \right)
/// \\end{equation}
///
/// in a way that is numerically stable when \\(b \ll a\\).  Furthermore, for
/// \\(b > a\\), this function returns the real part of the logarithm.
fn log_diff(a: f64, b: f64) -> f64 {
    let x = b / a;

    match x.abs() {
        y if y == 1.0 => x.signum() * f64::INFINITY,
        y if y < 0.2 => {
            x * crate::approximations::polynomial(
                x.powi(2),
                &[
                    2.000000000000000,
                    0.6666666666666667,
                    0.4000000000000000,
                    0.2857142857142857,
                    0.2222222222222222,
                    0.1818181818181818,
                    0.1538461538461538,
                    0.1333333333333333,
                    0.1176470588235294,
                    0.1052631578947368,
                ],
            )
        }
        y if y > 5.0 => {
            let xr = x.recip();
            xr * crate::approximations::polynomial(
                xr.powi(2),
                &[
                    2.000000000000000,
                    0.6666666666666667,
                    0.4000000000000000,
                    0.2857142857142857,
                    0.2222222222222222,
                    0.1818181818181818,
                    0.1538461538461538,
                    0.1333333333333333,
                    0.1176470588235294,
                    0.1052631578947368,
                ],
            )
        }
        _ => ((a + b) / (a - b)).abs().ln(),
    }
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::{f64, fs::File};

    #[test]
    fn log_diff() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/particle_physics/pave_absorptive/log_diff.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(ruzstd::StreamingDecoder::new(&mut f)?);
        let f = super::log_diff;

        for result in rdr.deserialize() {
            let (a, b, y): (f64, f64, f64) = result?;

            if !y.is_nan() {
                let ny = f(a, b);
                // println!("log_diff({:e}, {:e}) = {:e} [{:e}]", a, b, ny, y);
                approx_eq(ny, y, 8.0, 10f64.powi(-200))
            }
        }

        Ok(())
    }
}
