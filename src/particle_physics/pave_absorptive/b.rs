use crate::{
    other::binomial,
    particle_physics::{kallen_lambda, kallen_lambda_sqrt},
};
use std::f64::consts::PI;

/// Evaluate the Passarino-Veltman coefficient function:
///
/// \\begin{equation}
///   \boldsymbol{B}_{\underbrace{0\dots0}_{2r}\underbrace{1\dots1}_{n_1}}(s; m0, m1)
/// \\end{equation}
pub fn b(r: i32, n1: i32, s: f64, m0: f64, m1: f64) -> f64 {
    debug_assert!(m0 >= 0.0 && m1 >= 0.0, "masses must be non-negative.");

    // Result is vanishing in limit that all dimensionful arguments vanish
    if s <= (m0 + m1).powi(2) {
        return 0.0;
    }

    match (r, n1) {
        (0, 0) => scalar(s, m0, m1),
        (0, n1) => b1(n1, s, m0, m1),
        (r, n1) => {
            -(2.0 * (n1 + 1) as f64).recip()
                * ((s - m1.powi(2) + m0.powi(2)) * b(r - 1, n1 + 1, s, m0, m1)
                    + 2.0 * s * b(r - 1, n1 + 2, s, m0, m1))
        }
    }
}

/// Evaluate the scalar Passarino-Veltmann coefficient function:
///
/// \\begin{equation}
///   \boldsymbol{B}_{0}(s; m0, m1)
/// \\end{equation}
fn scalar(s: f64, m0: f64, m1: f64) -> f64 {
    disc(s, m0, m1)
}

/// Evaluate the Passarino-Veltman coefficient functions:
///
/// \\begin{equation}
///   \boldsymbol{B}_{\underbrace{1\dots1}_{n_1}}(s; m0, m1)
/// \\end{equation}
fn b1(n1: i32, s: f64, m0: f64, m1: f64) -> f64 {
    match n1 {
        0 => scalar(s, m0, m1),
        n => {
            let m0_2 = m0.powi(2);
            let m1_2 = m1.powi(2);
            let lambda = kallen_lambda(s, m0_2, m1_2);
            let f = s + m0_2 - m1_2;

            neg_power(n) / (n as f64 + 1.0)
                * (0..n / 2)
                    .map(|k| {
                        binomial(n + 1, 2 * k + 1)
                            * (f / (2.0 * s)).powi(n - 2 * k)
                            * (lambda / (4.0 * s.powi(2))).powi(k)
                            * disc(s, m0, m1)
                    })
                    .sum::<f64>()
        }
    }
}

fn disc(s: f64, m0: f64, m1: f64) -> f64 {
    PI * kallen_lambda_sqrt(s, m0.powi(2), m1.powi(2)) / s
}

fn neg_power(n: i32) -> f64 {
    if n % 2 == 0 {
        1.0
    } else {
        -1.0
    }
}
