use crate::{
    other::binomial,
    particle_physics::{kallen_lambda_sqrt, pave_absorptive as pave},
};

/// Evaluate the Passarin-Veltman coefficient function
///
/// \\begin{equation}
///   \boldsymbol{C}_{\underbrace{0\dots0}_{2r}\underbrace{1\dots1}_{n_1}\underbrace{2\dots2}_{n_2}}(s_1, s_{12}, s_2; m0, m1, m_2)
/// \\end{equation}
#[allow(clippy::too_many_arguments, non_snake_case)]
pub fn c(r: i32, n1: i32, n2: i32, s1: f64, s12: f64, s2: f64, m0: f64, m1: f64, m2: f64) -> f64 {
    debug_assert!(
        m0 >= 0.0 && m1 >= 0.0 && m1 >= 0.0,
        "masses must be non-negative."
    );

    if s1 <= (m0 + m1).powi(2) && s2 <= (m0 + m2).powi(2) && s12 <= (m1 + m2).powi(2) {
        return 0.0;
    }

    // Deal with the scalar case immediately as there is no reduction necessary.
    if r == 0 && n1 == 0 && n2 == 0 {
        return scalar(s1, s12, s2, m0, m1, m2);
    }

    // Always have `n1 >= n2`
    if n2 > n1 {
        return c(r, n2, n1, s2, s12, s1, m0, m2, m1);
    }

    // Precompute squared masses
    let m0_2 = m0.powi(2);
    let m1_2 = m1.powi(2);
    let m2_2 = m2.powi(2);

    let f = [s1 - m1_2 + m0_2, s2 - m2_2 + m0_2];
    let Z = [[s1, s12], [s12, s2]];
    let det_Z = Z[0][0] * Z[1][1] - Z[0][1] * Z[1][0];
    let coZ = [[s2, -s12], [-s12, s1]];
    // let det_coZ = coZ[0][0] * coZ[1][1] - coZ[0][1] * coZ[1][0];
    // let X0 = [s2 * f[0] - s12 * f[1], -s12 * f[0] + s1 * f[1]];

    // If Gramian matrix is non-singular
    if det_Z > 1e-50 {
        match (r, n1, n2) {
            (r, 0, 0) => {
                1.0 / (2.0 * r as f64) * (bd0(r - 1, 0, 0, s12, m1, m2))
                    + 2.0 * m0_2 * c(r - 1, 1, 0, s1, s12, s2, m0, m1, m2)
                    + f[1] * c(r - 1, 0, 2, s1, s12, s2, m0, m1, m2)
            }
            (_r, 0, _n2) => {
                // Covered by the ordering of n1 and n2
                unreachable!()
            }
            (r, n1, n2) => {
                1.0 / (2.0 * det_Z)
                    * (0..=1)
                        .map(|k| {
                            // let k_bar = 3 - k;
                            let nk = if k == 0 { n1 } else { n2 };
                            (0..=1)
                                .map(|j| {
                                    coZ[j][k]
                                        * (if j == k && nk == 1 {
                                            if k == 1 {
                                                pave::b(r, n1, s2, m0, m2)
                                            } else {
                                                pave::b(r, n1, s1, m0, m1)
                                            }
                                        } else {
                                            0.0
                                        } - bd0(r, n1 - 1, n2, s12, m1, m2)
                                            - f[k] * c(r, n1 - 1, n2, s1, s12, s2, m0, m1, m2)
                                            - 2.0
                                                * (nk - if j == k { 1 } else { 0 }) as f64
                                                * c(r + 1, n1 - 1, n2, s1, s12, s2, m0, m1, m2))
                                })
                                .sum::<f64>()
                        })
                        .sum::<f64>()
            }
        }
    } else {
        unimplemented!()
    }
}

fn scalar(s1: f64, s12: f64, s2: f64, m0: f64, m1: f64, m2: f64) -> f64 {
    let m0_2 = m0.powi(2);
    let m1_2 = m1.powi(2);
    let m2_2 = m2.powi(2);
    let lambda_s = kallen_lambda_sqrt(s1, s12, s2);

    let mut result = 0.0;

    if s1 > (m0 + m1).powi(2) {
        let lambda = kallen_lambda_sqrt(s1, m0_2, m1_2) * lambda_s;
        let m0_factor = m0_2 * (s1 + s12 - s2);
        let m1_factor = m1_2 * (s1 - s12 + s2);
        let s_factor = s1 * (-2.0 * m2_2 - s1 + s12 + s2);

        let numerator = lambda + m0_factor + m1_factor + s_factor;
        let denominator = -lambda + m0_factor + m1_factor + s_factor;

        result += (numerator / denominator).ln();
    }

    if s2 > (m0 + m2).powi(2) {
        let lambda = kallen_lambda_sqrt(s2, m0_2, m2_2) * lambda_s;
        let m0_factor = m0_2 * (-s1 + s12 + s2);
        let m2_factor = m2_2 * (s1 - s12 + s2);
        let s_factor = s2 * (-2.0 * m1_2 + s1 + s12 - s2);

        let numerator = lambda + m0_factor + m2_factor + s_factor;
        let denominator = -lambda + m0_factor + m2_factor + s_factor;

        result += (numerator / denominator).ln();
    }

    if s12 > (m1 + m2).powi(2) {
        let lambda = kallen_lambda_sqrt(s12, m1_2, m2_2) * lambda_s;
        let m1_factor = m1_2 * (-s1 + s12 + s2);
        let m2_factor = m2_2 * (s1 + s12 - s2);
        let s_factor = s12 * (-2.0 * m0_2 + s1 - s12 + s2);

        let numerator = lambda + m1_factor + m2_factor + s_factor;
        let denominator = -lambda + m1_factor + m2_factor + s_factor;

        result += (numerator / denominator).ln();
    }

    result / lambda_s
}

fn bd0(r: i32, n1: i32, n2: i32, s: f64, m1: f64, m2: f64) -> f64 {
    neg_power(n1)
        * (0..=n1)
            .map(|j| binomial(n1, j) * pave::b(r, n2 + j, s, m1, m2))
            .sum::<f64>()
}

fn neg_power(n: i32) -> f64 {
    if n % 2 == 0 {
        1.0
    } else {
        -1.0
    }
}
