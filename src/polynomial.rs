//! Utilities to handle polynomials

/// Evaluates an arbitrary single-variable polynomial at a particular point.
///
/// Given a list of coefficient \\(c = [c_0, c_1, \dots, c_n]\\), evaluates the
/// polynomial
/// \\[
///     p(x) = c_0 + c_1 x + c_2 x\^2 + \dots + c_n x\^n
/// \\]
/// at the specified value of \\(x\\).
///
/// If the list of coefficients is empty, the function returns `0.0`.
///
/// # Warning
///
/// This function does not perform any checks on the coefficients.
#[inline]
pub fn polynomial(x: f64, c: &[f64]) -> f64 {
    // This expands the polynomial as:
    //
    // c0 + x * (c1 + x * (c2 + x * (...)))
    c.iter().rev().fold(0.0, |ans, &ci| ci + x * ans)
}
}
