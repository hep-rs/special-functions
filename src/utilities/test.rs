/// Check whether two numbers are equal to each other within the specified
/// precision and absolute error.
///
/// Note that the absolute error really should be `0.0`.  Floating points
/// are designed to handle values across a very broad range and their
/// relative error really is more important.  Having said that, in this
/// situation small values will appear when we have a vanishing number
/// density and therefore it is reasonable to ignore large relative errors
/// for very small values as they ultimately have a small physical impact.
///
/// The precision is specified in decimal
/// significant figures.
pub(crate) fn approx_eq(a: f64, b: f64, precision: f64, abs: f64) {
    // If neither are numbers, they are not comparable
    if a.is_nan() {
        panic!("a is NaN.");
    }
    if b.is_nan() {
        panic!("b is NaN.");
    }

    // If they are already identical, return.  They could both be infinite
    // at this stage (which is fine)
    if a == b {
        debug!("a and b are identical");
        return;
    }

    // Since they are not identical, if either one is infinite, the other
    // must either be another infinity or be finite and therefore they are
    // not equal
    match (a.is_infinite(), b.is_infinite()) {
        (true, true) => panic!("a and b are different infinities."),
        (true, false) => panic!("a is infinite while b is finite."),
        (false, true) => panic!("b is infinite while a is finite."),
        (false, false) => (),
    }

    // Check if their absolute error is acceptable
    if (a - b).abs() < abs {
        debug!(
            "a and b are within the absolute error ({} < {}).",
            (a - b).abs(),
            abs
        );
        return;
    }

    // Scale numbers to be within the range (-10, 10) so that we can check
    // the significant figures.
    let avg = 0.5 * (a + b).abs();
    let scale = f64::powf(10.0, avg.log10().floor());

    let a_scaled = a / scale;
    let b_scaled = b / scale;

    let p = (a_scaled - b_scaled).abs();
    if p <= 10f64.powf(-precision) {
        debug!(
            "a and b have the necessary precision ({} ≥ {})",
            -p.log10(),
            precision
        );
    } else {
        panic!(
            "a and b do not have the necessary precision ({} !≥ {})",
            -p.log10(),
            precision
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64;

    #[test]
    #[should_panic]
    fn a_nan() {
        approx_eq(f64::NAN, 0.0, 10.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn b_nan() {
        approx_eq(1.0, f64::NAN, 10.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn a_b_nan() {
        approx_eq(f64::NAN, f64::NAN, 10.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn a_infinite() {
        approx_eq(f64::INFINITY, 0.0, 10.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn b_infinite() {
        approx_eq(0.0, f64::INFINITY, 10.0, 0.0);
    }

    #[test]
    fn a_b_infinite() {
        approx_eq(f64::INFINITY, f64::INFINITY, 10.0, 0.0);
        approx_eq(f64::NEG_INFINITY, f64::NEG_INFINITY, 10.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn a_b_diff_infinite() {
        approx_eq(f64::INFINITY, f64::NEG_INFINITY, 10.0, 0.0);
    }

    #[test]
    fn absolute_error() {
        approx_eq(1e-20, 2e-20, 10.0, 1e-10);
        approx_eq(-1e-20, 2e-20, 10.0, 1e-10);
        approx_eq(1e-20, -2e-20, 10.0, 1e-10);
        approx_eq(-1e-20, -2e-20, 10.0, 1e-10);
    }

    #[test]
    #[should_panic]
    fn absolute_error_panic() {
        approx_eq(1e-20, 2e-20, 10.0, 1e-30);
    }

    #[test]
    fn precision() {
        let eps = 0.05;
        approx_eq(1.0000000000000000, 1.1000000000000000, 1.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0100000000000000, 2.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0010000000000000, 3.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0001000000000000, 4.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000100000000000, 5.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000010000000000, 6.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000001000000000, 7.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000000100000000, 8.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000000010000000, 9.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000000001000000, 10.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000000000100000, 11.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000000000010000, 12.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000000000001000, 13.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000000000000100, 14.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000000000000010, 15.0 - eps, 0.0);
        approx_eq(1.0000000000000000, 1.0000000000000001, 16.0 - eps, 0.0);
    }

    #[test]
    #[should_panic]
    fn precision_panic() {
        approx_eq(1.0000000000000000, 1.0000000010000000, 10.0, 0.0);
    }
}
