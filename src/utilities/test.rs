//! Utility function for testing

use std::{convert, default, error, fmt};

pub(crate) struct FloatEqError {
    err: String,
}

impl default::Default for FloatEqError {
    fn default() -> Self {
        FloatEqError {
            err: "Default error.".to_string(),
        }
    }
}

impl fmt::Debug for FloatEqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl fmt::Display for FloatEqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}
impl error::Error for FloatEqError {}

impl<T> convert::From<T> for FloatEqError
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        Self {
            err: s.as_ref().to_string(),
        }
    }
}

/// Check whether two numbers are equal to each other within the specified
/// relative and and absolute error.
///
/// Note that the absolute error really should be `0.0`.  Floating points are
/// designed to handle values across a very broad range and their relative error
/// really is more important.  Having said that, in this situation small values
/// will appear when we have a vanishing number density and therefore it is
/// reasonable to ignore large relative errors for very small values as they
/// ultimately have a small physical impact.
///
/// The relative error is specified in decimal significant figures.
pub(crate) fn approx_eq(a: f64, b: f64, eps_rel: f64, eps_abs: f64) -> Result<(), FloatEqError> {
    // If neither are numbers, they are not comparable
    if a.is_nan() {
        return Err("a is NaN.".into());
    }
    if b.is_nan() {
        return Err("b is NaN.".into());
    }

    // If they are already identical, return.  They could both be infinite
    // at this stage (which is fine)
    if a == b {
        log::debug!("a and b are identical");
        return Ok(());
    }

    // Since they are not identical, if either one is infinite, the other
    // must either be another infinity or be finite and therefore they are
    // not equal
    match (a.is_infinite(), b.is_infinite()) {
        (true, true) => return Err("a and b are different infinities.".into()),
        (true, false) => return Err("a is infinite while b is finite.".into()),
        (false, true) => return Err("b is infinite while a is finite.".into()),
        (false, false) => (),
    }

    // Check if their absolute error is acceptable
    if (a - b).abs() < eps_abs {
        log::debug!(
            "a and b are within the absolute error ({} < {}).",
            (a - b).abs(),
            eps_abs
        );
        return Ok(());
    }

    // Scale numbers to be within the range (-10, 10) so that we can check
    // the significant figures.
    let avg = 0.5 * (a + b).abs();
    let scale = f64::powf(10.0, avg.log10().floor());

    let a_scaled = a / scale;
    let b_scaled = b / scale;

    let p = (a_scaled - b_scaled).abs();
    if p <= 10_f64.powf(-eps_rel) {
        log::debug!(
            "a ({:e}) and b ({:e}) have the necessary precision ({:.3} ≥ {:.3})",
            a,
            b,
            -p.log10(),
            eps_rel
        );
        Ok(())
    } else {
        Err(format!(
            "a ({:e}) and b ({:e}) do not have the necessary precision ({:.3} !≥ {:.3})",
            a,
            b,
            -p.log10(),
            eps_rel
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::approx_eq;
    use std::{error, f64};

    #[test]
    fn a_nan() -> Result<(), Box<dyn error::Error>> {
        approx_eq(f64::NAN, 0.0, 10.0, 0.0).unwrap_err();
        Ok(())
    }

    #[test]
    fn b_nan() -> Result<(), Box<dyn error::Error>> {
        approx_eq(1.0, f64::NAN, 10.0, 0.0).unwrap_err();
        Ok(())
    }

    #[test]
    fn a_b_nan() -> Result<(), Box<dyn error::Error>> {
        approx_eq(f64::NAN, f64::NAN, 10.0, 0.0).unwrap_err();
        Ok(())
    }

    #[test]
    fn a_infinite() -> Result<(), Box<dyn error::Error>> {
        approx_eq(f64::INFINITY, 0.0, 10.0, 0.0).unwrap_err();
        Ok(())
    }

    #[test]
    fn b_infinite() -> Result<(), Box<dyn error::Error>> {
        approx_eq(0.0, f64::INFINITY, 10.0, 0.0).unwrap_err();
        Ok(())
    }

    #[test]
    fn a_b_infinite() -> Result<(), Box<dyn error::Error>> {
        approx_eq(f64::INFINITY, f64::INFINITY, 10.0, 0.0)?;
        approx_eq(f64::NEG_INFINITY, f64::NEG_INFINITY, 10.0, 0.0)?;
        Ok(())
    }

    #[test]
    fn a_b_diff_infinite() -> Result<(), Box<dyn error::Error>> {
        approx_eq(f64::INFINITY, f64::NEG_INFINITY, 10.0, 0.0).unwrap_err();
        Ok(())
    }

    #[test]
    fn absolute_error() -> Result<(), Box<dyn error::Error>> {
        approx_eq(1e-20, 2e-20, 10.0, 1e-10)?;
        approx_eq(-1e-20, 2e-20, 10.0, 1e-10)?;
        approx_eq(1e-20, -2e-20, 10.0, 1e-10)?;
        approx_eq(-1e-20, -2e-20, 10.0, 1e-10)?;
        Ok(())
    }

    #[test]
    fn absolute_error_panic() -> Result<(), Box<dyn error::Error>> {
        approx_eq(1e-20, 2e-20, 10.0, 1e-30).unwrap_err();
        Ok(())
    }

    #[test]
    fn precision() -> Result<(), Box<dyn error::Error>> {
        let eps = 0.05;
        approx_eq(1.0, 1.1, 1.0 - eps, 0.0)?;
        approx_eq(1.0, 1.01, 2.0 - eps, 0.0)?;
        approx_eq(1.0, 1.001, 3.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_1, 4.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_01, 5.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_001, 6.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_000_1, 7.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_000_01, 8.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_000_001, 9.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_000_000_1, 10.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_000_000_01, 11.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_000_000_001, 12.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_000_000_000_1, 13.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_000_000_000_01, 14.0 - eps, 0.0)?;
        approx_eq(1.0, 1.000_000_000_000_001, 15.0 - eps, 0.0)?;
        approx_eq(1.0, 1.0, 16.0 - eps, 0.0)?;
        Ok(())
    }

    #[test]
    fn precision_panic() -> Result<(), Box<dyn error::Error>> {
        approx_eq(1.0, 1.000_000_001, 10.0, 0.0).unwrap_err();
        Ok(())
    }
}
