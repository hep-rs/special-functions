use crate::{
    other::binomial,
    particle_physics::{kallen_lambda, kallen_lambda_sqrt},
};
use std::f64::consts::PI;

/// Internal parameters shared within the evaluation of the B function.
struct Parameters {
    s: f64,
    s_2: f64,
    m0_2: f64,
    m1_2: f64,
    lambda: f64,
    f: f64,
}

impl Parameters {
    /// Create a new instance of internal parameters.  The arguments are defined
    /// in the same way as the coefficient function.
    fn new(s: f64, m0: f64, m1: f64) -> Self {
        let m0_2 = m0.powi(2);
        let m1_2 = m1.powi(2);
        Self {
            s,
            s_2: s.powi(2),
            m0_2,
            m1_2,
            lambda: kallen_lambda(s, m0_2, m1_2),
            f: s - m1_2 + m0_2,
        }
    }
}

/// Absorptive part of the Passarin-Veltman coefficient function
/// `$\boldsymbol{B}$`.
///
/// ```math
/// \boldsymbol{B}_{\underbrace{0\dots0}_{2r}\underbrace{1\dots1}_{n_1}}(s; m0, m1)
/// ```
///
/// This is implemented using the general recursion relation and thus in
/// principal works for all values of `$r$` and `$n_1$`; however, higher order
/// values are prone to numerical instabilities.
pub fn b(r: i32, n1: i32, s: f64, m0: f64, m1: f64) -> f64 {
    debug_assert!(r >= -1, "r cannot be smaller than -1");
    debug_assert!(n1 >= 0, "n1 must be non-negative");
    debug_assert!(m0 >= 0.0 && m1 >= 0.0, "masses must be non-negative.");

    // Result is vanishing if kinematically forbidden.
    if s <= (m0 + m1).powi(2) {
        return 0.0;
    }

    let params = Parameters::new(s, m0, m1);
    b_internal(r, n1, &params)
}

/// Internal implementation of the coefficient function which uses precomputed
/// internal variables shared across recursive calls.
///
/// All checks for the validity of input parameters should be already done.
fn b_internal(r: i32, n1: i32, param: &Parameters) -> f64 {
    match (r, n1) {
        (0, 0) => scalar(&param),
        (0, n1) => {
            neg_power(n1) / (n1 + 1) as f64
                * (0..=(n1 / 2))
                    .map(|k| {
                        binomial(n1 + 1, 2 * k + 1)
                            * (param.f / (2.0 * param.s)).powi(n1 - 2 * k)
                            * (param.lambda / (4.0 * param.s_2)).powi(k)
                            * disc(param)
                    })
                    .sum::<f64>()
        }
        (r, n1) => {
            -(2.0 * (n1 + 1) as f64).recip()
                * (param.f * b_internal(r - 1, n1 + 1, param)
                    + 2.0 * param.s * b_internal(r - 1, n1 + 2, param))
        }
    }
}

/// Scalar Passarino-Veltmann coefficient function:
///
/// ```math
///   \boldsymbol{B}_{0}(s; m0, m1)
/// ```
fn scalar(param: &Parameters) -> f64 {
    disc(param)
}

/// Part of the B coefficient function which contains the discontinuity.
fn disc(param: &Parameters) -> f64 {
    PI * kallen_lambda_sqrt(param.s, param.m0_2, param.m1_2) / param.s
}

/// Function for `(-1)^n`.
fn neg_power(n: i32) -> f64 {
    if n % 2 == 0 {
        1.0
    } else {
        -1.0
    }
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::{f64, fs::File};

    #[test]
    fn b() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/particle_physics/pave_absorptive/b.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(ruzstd::StreamingDecoder::new(&mut f)?);
        let f = super::b;

        for result in rdr.deserialize() {
            let (r, n1, s, m0, m1, y): (f64, f64, f64, f64, f64, f64) = result?;
            let r = r as i32;
            let n1 = n1 as i32;

            if !y.is_nan() {
                let ny = f(r, n1, s, m0, m1);
                // println!(
                //     "B({}, {}, {:e}, {:e}, {:e}) = {:e} [{:e}]",
                //     r, n1, s, m0, m1, ny, y
                // );
                match (r, n1) {
                    (0, n1) if n1 < 5 => approx_eq(ny, y, 8.0, 10f64.powi(-200))?,
                    (0, 5) => approx_eq(ny, y, 7.0, 10f64.powi(-200))?,
                    (1, n1) if n1 < 4 => approx_eq(ny, y, 8.0, 10f64.powi(-200))?,
                    (1, 4) => approx_eq(ny, y, 6.0, 10f64.powi(-200))?,
                    (1, 5) => approx_eq(ny, y, 5.0, 10f64.powi(-200))?,
                    (2, _) => approx_eq(ny, y, 4.0, 10f64.powi(-200))?,
                    (3, 0) | (3, 1) => approx_eq(ny, y, 2.0, 10f64.powi(-200))?,
                    (3, _) | (4, _) | (5, _) => (),
                    _ => unreachable!(),
                }
            }
        }

        Ok(())
    }
}
