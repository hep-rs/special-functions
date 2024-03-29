mod explicit;

use crate::particle_physics::kallen_lambda_sqrt;
use std::f64;

/// Internal parameters shared within the evaluation of the C function.
pub(crate) struct Parameters {
    s1: f64,
    s12: f64,
    s2: f64,
    m0: f64,
    m0_2: f64,
    m1: f64,
    m1_2: f64,
    m2: f64,
    m2_2: f64,
    lambda_s12_sqrt: f64,
    lambda_m01_sqrt: f64,
    lambda_m12_sqrt: f64,
    lambda_m02_sqrt: f64,
}

impl Parameters {
    /// Create a new instance of internal parameters.  The arguments are defined
    /// in the same way as the coefficient function.
    fn new(s1: f64, s12: f64, s2: f64, m0: f64, m1: f64, m2: f64) -> Self {
        let m0_2 = m0.powi(2);
        let m1_2 = m1.powi(2);
        let m2_2 = m2.powi(2);
        Self {
            s1,
            s12,
            s2,
            m0,
            m0_2,
            m1,
            m1_2,
            m2,
            m2_2,
            lambda_s12_sqrt: kallen_lambda_sqrt(s1, s12, s2),
            lambda_m01_sqrt: kallen_lambda_sqrt(s1, m0_2, m1_2),
            lambda_m12_sqrt: kallen_lambda_sqrt(s12, m1_2, m2_2),
            lambda_m02_sqrt: kallen_lambda_sqrt(s2, m0_2, m2_2),
        }
    }
}
/// Absorptive part of the Passarin-Veltman coefficient function
/// `$\boldsymbol{C}$`.
///
/// ```math
/// \boldsymbol{C}_{\underbrace{0\dots0}_{2r}\underbrace{1\dots1}_{n_1}\underbrace{2\dots2}_{n_2}}(s_1, s_{12}, s_2; m0, m1, m_2)
/// ```
///
/// This is implemented using explicit expressions exported from the Mathematica
/// package [Package-X](https://packagex.hepforge.org/)
/// ([arXiv:1503.01469](https://arxiv.org/abs/1503.01469)), and has been
/// implemented for all combinations of `$r$`, `$n_1$` and `$n_2$` in `$\{0, 1,
/// 2\}$`.
///
/// Note that higher order functions will generally be less accurate as the code
/// does not check for numerical stability.
#[allow(clippy::too_many_arguments)]
pub fn c(r: i32, n1: i32, n2: i32, s1: f64, s12: f64, s2: f64, m0: f64, m1: f64, m2: f64) -> f64 {
    debug_assert!(n1 >= 0 && n2 >= 0, "n1 and n2 must be non-negative.");
    debug_assert!(
        m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0,
        "masses must be non-negative."
    );

    if s1 <= (m0 + m1).powi(2) && s2 <= (m0 + m2).powi(2) && s12 <= (m1 + m2).powi(2) {
        return 0.0;
    }

    // Always have `n1 >= n2`
    if n2 > n1 {
        let param = Parameters::new(s2, s12, s1, m0, m2, m1);
        c_internal(r, n2, n1, &param)
    } else {
        let param = Parameters::new(s1, s12, s2, m0, m1, m2);
        c_internal(r, n1, n2, &param)
    }
}

/// Internal implementation of the coefficient function which uses precomputed
/// internal variables shared across recursive calls.
///
/// All checks for the validity of input parameters should be already done.
/// Furthermore, re-ordering of the arguments such that n1 >= n2 should have
/// already been done.
fn c_internal(r: i32, n1: i32, n2: i32, param: &Parameters) -> f64 {
    match (r, n1, n2) {
        (0, 0, 0) => explicit::c000(param),
        (0, 1, 0) => explicit::c010(param),
        (0, 1, 1) => explicit::c011(param),
        (0, 2, 0) => explicit::c020(param),
        (0, 2, 1) => explicit::c021(param),
        (0, 2, 2) => explicit::c022(param),
        (1, 0, 0) => explicit::c100(param),
        (1, 1, 0) => explicit::c110(param),
        (1, 1, 1) => explicit::c111(param),
        (1, 2, 0) => explicit::c120(param),
        (1, 2, 1) => explicit::c121(param),
        (1, 2, 2) => explicit::c122(param),
        (2, 0, 0) => explicit::c200(param),
        (2, 1, 0) => explicit::c210(param),
        (2, 1, 1) => explicit::c211(param),
        (2, 2, 0) => explicit::c220(param),
        (2, 2, 1) => explicit::c221(param),
        (2, 2, 2) => explicit::c222(param),
        (_, n1, n2) if n2 > n1 => unreachable!(),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::{f64, fs::File};

    #[test]
    #[ignore]
    fn c() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/particle_physics/pave_absorptive/c.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);
        let f = super::c;

        for (row, result) in rdr.deserialize().enumerate() {
            let (r, n1, n2, s1, s12, s2, m0, m1, m2, y): (
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
            ) = result?;
            let r = r as i32;
            let n1 = n1 as i32;
            let n2 = n2 as i32;

            if !y.is_nan() {
                let ny = f(r, n1, n2, s1, s12, s2, m0, m1, m2);
                approx_eq(ny, y, 4.0, 10f64.powi(-200)).map_err(|err| {
                    println!(
                        "[{}] C({}, {}, {}, {:e}, {:e}, {:e}, {:e}, {:e}, {:e}) = {:e} but expected {:e}.",
                        row, r, n1, n2, s1, s12, s2, m0, m1, m2, ny, y
                    );
                    err
                })?;
            }
        }

        Ok(())
    }
}
