mod explicit;

use crate::particle_physics::kallen_lambda_sqrt;
use std::f64;

/// Internal parameters shared within the evaluation of the D function.
pub(crate) struct Parameters {
    s1: f64,
    s2: f64,
    s3: f64,
    s4: f64,
    s12: f64,
    s23: f64,
    m0: f64,
    m0_2: f64,
    m1: f64,
    m1_2: f64,
    m2: f64,
    m2_2: f64,
    m3: f64,
    m3_2: f64,
    lambda_m01_sqrt: f64,
    lambda_m02_sqrt: f64,
    lambda_m03_sqrt: f64,
    lambda_m12_sqrt: f64,
    lambda_m13_sqrt: f64,
    lambda_m23_sqrt: f64,
    lambda_s12_sqrt: f64,
    lambda_s14_sqrt: f64,
    lambda_s34_sqrt: f64,
    lambda_s23_sqrt: f64,
}

impl Parameters {
    /// Create a new instance of internal parameters.  The arguments are defined
    /// in the same way as the coefficient function.
    #[allow(clippy::too_many_arguments)]
    fn new(
        s1: f64,
        s2: f64,
        s3: f64,
        s4: f64,
        s12: f64,
        s23: f64,
        m0: f64,
        m1: f64,
        m2: f64,
        m3: f64,
    ) -> Self {
        let m0_2 = m0.powi(2);
        let m1_2 = m1.powi(2);
        let m2_2 = m2.powi(2);
        let m3_2 = m3.powi(2);
        Self {
            s1,
            s2,
            s3,
            s4,
            s12,
            s23,
            m0,
            m0_2,
            m1,
            m1_2,
            m2,
            m2_2,
            m3,
            m3_2,
            lambda_m01_sqrt: kallen_lambda_sqrt(m0_2, m1_2, s1),
            lambda_m02_sqrt: kallen_lambda_sqrt(m0_2, m2_2, s12),
            lambda_m03_sqrt: kallen_lambda_sqrt(m0_2, m3_2, s4),
            lambda_m12_sqrt: kallen_lambda_sqrt(m1_2, m2_2, s2),
            lambda_m13_sqrt: kallen_lambda_sqrt(m1_2, m3_2, s23),
            lambda_m23_sqrt: kallen_lambda_sqrt(m2_2, m3_2, s3),
            lambda_s12_sqrt: kallen_lambda_sqrt(s1, s12, s2),
            lambda_s14_sqrt: kallen_lambda_sqrt(s1, s23, s4),
            lambda_s34_sqrt: kallen_lambda_sqrt(s12, s3, s4),
            lambda_s23_sqrt: kallen_lambda_sqrt(s2, s23, s3),
        }
    }
}

/// Absorptive part of the Passarin-Veltman coefficient function
/// `$\boldsymbol{C}$`.
///
/// ```math
/// \boldsymbol{D}_{\underbrace{0\dots0}_{2r}\underbrace{1\dots1}_{n_1}\underbrace{2\dots2}_{n_2}\underbrace{3\dots3}_{n_3}}(s_1, s_2, s_3, s_4; s_{12}, s_{23}; m0, m1, m_2, m4)
/// ```
///
/// This is implemented using explicit expressions exported from the Mathematica
/// package [Package-X](https://packagex.hepforge.org/)
/// ([arXiv:1503.01469](https://arxiv.org/abs/1503.01469)), and has been
/// implemented for the scalar loop function, and where one of `$r$`, `$n_1$`,
/// `$n_2$` or `$n_3$` is 1.
///
/// Note that higher order functions will generally be less accurate as the code
/// does not check for numerical stability.
#[allow(clippy::too_many_arguments)]
pub fn d(
    r: i32,
    n1: i32,
    n2: i32,
    n3: i32,
    s1: f64,
    s2: f64,
    s3: f64,
    s4: f64,
    s12: f64,
    s23: f64,
    m0: f64,
    m1: f64,
    m2: f64,
    m3: f64,
) -> f64 {
    debug_assert!(
        n1 >= 0 && n2 >= 0 && n3 >= 0,
        "n1, n2 and n3 must be non-negative."
    );
    debug_assert!(
        m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 && m3 >= 0.0,
        "masses must be non-negative."
    );

    if s1 <= (m0 + m1).powi(2)
        && s2 <= (m1 + m2).powi(2)
        && s3 <= (m2 + m3).powi(2)
        && s4 <= (m0 + m3).powi(2)
        && s12 <= (m0 + m2).powi(2)
        && s23 <= (m1 + m3).powi(2)
    {
        return 0.0;
    }

    let param = Parameters::new(s1, s2, s3, s4, s12, s23, m0, m1, m2, m3);
    d_internal(r, n1, n2, n3, &param)
}

/// Internal implementation of the coefficient function which uses precomputed
/// internal variables shared across recursive calls.
///
/// All checks for the validity of input parameters should be already done.
fn d_internal(r: i32, n1: i32, n2: i32, n3: i32, param: &Parameters) -> f64 {
    match (r, n1, n2, n3) {
        (0, 0, 0, 0) => explicit::d0000(&param),
        (0, 0, 0, 1) => explicit::d0001(&param),
        (0, 0, 1, 0) => explicit::d0010(&param),
        (0, 1, 0, 0) => explicit::d0100(&param),
        (1, 0, 0, 0) => explicit::d1000(&param),
        _ => unimplemented!(),
    }
}
#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::{f64, fs::File};

    #[test]
    #[ignore]
    fn d() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/particle_physics/pave_absorptive/d.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);
        let f = super::d;

        for (row, result) in rdr.deserialize().enumerate() {
            #[allow(clippy::type_complexity)]
            let (r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3, y): (
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
                f64,
                f64,
                f64,
                f64,
                f64,
            ) = result?;
            let r = r as i32;
            let n1 = n1 as i32;
            let n2 = n2 as i32;
            let n3 = n3 as i32;

            if !y.is_nan() {
                let ny = f(r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3);
                approx_eq(ny, y, 1.0, 10f64.powi(-200)).map_err(|err| {
                    println!(
                        "[{}] D({}, {}, {}, {}, {:e}, {:e}, {:e}, {:e}, {:e}, {:e}, {:e}, {:e}, {:e}, {:e}) = {:e} but expected {:e}.",
                        row, r, n1, n2, n3, s1, s2, s3, s4, s12, s23, m0, m1, m2, m3, ny, y
                    );
                    err
                })?
            }
        }

        Ok(())
    }
}
