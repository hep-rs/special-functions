//! Particle statistics

pub mod pave_absorptive;
pub mod statistics;

/// Kallen lambda function:
///
/// \\begin{equation}
///   \lambda(a, b, c) = a^2 + b^2 + c^2 - 2ab - 2ac - 2bc
/// \\end{equation}
///
/// # Example
///
/// ```
/// use special_functions::particle_physics::kallen_lambda;
///
/// assert_eq!(kallen_lambda(5.0, 2.0, 0.5), 2.25);
/// assert_eq!(kallen_lambda(1.0, 1.0, 1.0), -3.0);
/// ```
#[must_use]
pub fn kallen_lambda(a: f64, b: f64, c: f64) -> f64 {
    a.powi(2) + b.powi(2) + c.powi(2) - 2.0 * (a * b + a * c + b * c)
}

/// Square root of the Kallen lambda function:
///
/// \\begin{equation}
///   \lambda^{\frac{1}{2}}(a, b, c) = \sqrt{a^2 + b^2 + c^2 - 2ab - 2ac - 2bc}
/// \\end{equation}
///
/// This implementation is more precise than taking the square root of
/// [`kallen_lambda`] in cases where the arguments span several orders of
/// magnitude.
///
/// # Example
///
/// ```
/// use special_functions::particle_physics::{kallen_lambda, kallen_lambda_sqrt};
///
/// assert!((kallen_lambda_sqrt(5.0, 2.0, 0.5) - 1.5).abs() < 1e-14);
/// assert!((kallen_lambda(5.0, 2.0, 0.5).sqrt() - kallen_lambda_sqrt(5.0, 2.0, 0.5)).abs() < 1e-14);
/// assert!((kallen_lambda_sqrt(1.0, 1.0, 1.0) - 3f64.sqrt()).abs() < 1e-14);
/// ```
///
/// # Warning
///
/// The result returns the root of the *absolute value*, thus returning a result
/// even if `kallen_lambda` is negative.  It is up to the user to determine
/// whether this is valid or not.
#[must_use]
pub fn kallen_lambda_sqrt(a: f64, b: f64, c: f64) -> f64 {
    let max = if a > b { a } else { b };
    let max = if max > c { max } else { c };

    max * kallen_lambda(a / max, b / max, c / max).abs().sqrt()
}
