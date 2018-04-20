//! Bessel functions
//!
//! At this stage, this only implements the function \\(K_2(x)\\).

use std::f64;

/// Approximation of modified Bessel function \\(K_2(x)\\).
///
/// The approximation is split in three regimes:
///
/// - \\(|x| \ll 1\\), where the Taylor series expansion around \\(x = 0\\) is
///   used;
/// - \\(|x| \approx 1\\), where a mini-max polynomial is used which ensures the
///   error is spread over the whole interval and overall bias is minimized.
///   (This is unlike a Taylor series where the error is minimized at a single
///   point and increases (sometimes rapidly) everywhere else);
/// - \\(|x| \gg 1\\), where the Taylor series expansion around \\(x = \infty\\)
///   is used.
pub fn k_2(x: f64) -> f64 {
    debug_assert!(x >= 0.0, "Argument of BesselK must be positive.");

    if x == 0.0 {
        return f64::INFINITY;
    }

    // The approximation is done in the log-transformed variable (that is, the
    // interpolation is of the function K₂(exp(x))).
    let xln = x.ln();

    if xln < 0.3 {
        let x2 = x.powi(2);

        -0.50000000000000000000 + x2 * (0.10824143945730155610 - 0.12500000000000000000 * xln)
            + 2.0 * x2.powi(-1)
            + x2.powi(2) * (0.015964564399219574120 - 0.010416666666666666667 * xln)
            + x2.powi(3) * (0.00062096294997561169124 - 0.00032552083333333333333 * xln)
            + x2.powi(4) * (0.000011796141758852787447 - 5.4253472222222222222e-6 * xln)
            + x2.powi(5) * (1.3465023364738628899e-7 - 5.6514033564814814815e-8 * xln)
            + x2.powi(6) * (1.0309882406219202048e-9 - 4.0367166832010582011e-10 * xln)
            + x2.powi(7) * (5.6763386749232758866e-12 - 2.1024566058338844797e-12 * xln)
    } else if xln < 3.4 {
        let num = -0.0000180238 * xln.powi(8) - 0.0000205089 * xln.powi(7)
            + 0.00064349 * xln.powi(6) - 0.0178769 * xln.powi(5)
            + 0.072644 * xln.powi(4) - 0.467429 * xln.powi(3)
            + 0.78293 * xln.powi(2) - 2.57649 * xln + 0.485409;
        let denom = 0.0000176583 * xln.powi(6) - 0.000523792 * xln.powi(5)
            + 0.00663565 * xln.powi(4) - 0.045622 * xln.powi(3)
            + 0.177564 * xln.powi(2) - 0.424493 * xln + 1.0;

        (num / denom).exp()
    } else {
        let ex = x.exp();
        if ex == f64::INFINITY {
            0.0
        } else {
            (1.4133050937925706925 - 0.64608232859088945941 * x + 0.39758912528670120579 * x.powi(2)
                - 0.38554096997498298743 * x.powi(3)
                + 1.0281092532666212998 * x.powi(4) + 2.3499640074665629710 * x.powi(5)
                + 1.2533141373155002512 * x.powi(6)) / (6.5 * xln).exp() / ex
        }
    }
}
