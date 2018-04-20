//! Polynomials
//!
//! Functions to deal with polynomials.

/// Given a list of coefficient \\(c = [c_0, c_1, \dots, c_n]\\), evaluates the
/// polynomial
/// \\[
///     p(x) = c_0 + c_1 x + c_2 x\^2 + \dots + c_n x\^n
/// \\]
/// at the specified value of \\(x\\).
#[inline]
pub fn polynomial(x: f64, c: &[f64]) -> f64 {
    debug_assert!(
        !c.is_empty(),
        "List of coefficients must have at least one element."
    );

    // This expands the polynomial as:
    //
    // c0 + x * (c1 + x * (c2 + x * (...)))
    let n = c.len();
    c.iter()
        .rev()
        .skip(1)
        .fold(c[n - 1], |ans, &ci| ci + x * ans)
}
