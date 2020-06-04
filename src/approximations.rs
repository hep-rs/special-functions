//! Approximations

pub mod linear;

/// Evaluates an arbitrary single-variable polynomial at a particular point.
///
/// Given an array of coefficients \\(c = [c_0, c_1, \dots, c_n]\\), evaluates
/// the polynomial
/// \\begin{equation}
///     p(x) = c_0 + c_1 x + c_2 x\^2 + \dots + c_n x\^n
/// \\end{equation}
/// at the specified value of \\(x\\).
///
/// If the list of coefficients is empty, the function returns `0.0`.
///
/// This function does not perform any checks on the coefficients.
#[inline]
pub fn polynomial(x: f64, c: &[f64]) -> f64 {
    // The Clenshaw algorithm relation is:
    //
    // >b(k, x) := a(k) + x b(k + 1, x)
    //
    // with the final value being simply b(0, x).
    c.iter().rev().fold(0.0, |ans, &ci| ans.mul_add(x, ci))
}

/// Evaluates an arbitrary piecewise single-variable polynomial at a particular
/// point.
///
/// The splits divide the domain in closed-open intervals (except for the last
/// one which is closed-closed) and thus the length of `splits` must be one
/// longer than the list of coefficient arrays.  The [`polynomial`] function
/// used within each interval.
///
/// For values outside the domain of the piecewise polynomial, the first or last
/// polynomial is used for extrapolation.
pub fn piecewise_polynomial(x: f64, c: &[&[f64]], splits: &[f64]) -> f64 {
    debug_assert_eq!(
        splits.len(),
        c.len() + 1,
        "The number of splits must be one longer than number of coefficient arrays."
    );

    unsafe {
        match splits.binary_search_by(|s| s.partial_cmp(&x).unwrap()) {
            Ok(idx) if idx == splits.len() - 1 => polynomial(x, c.get_unchecked(idx - 2)),
            Ok(idx) => polynomial(x, c.get_unchecked(idx)),
            Err(0) => {
                if cfg!(debug_assertions) {
                    log::warn!("Extrapolation is being used.");
                }
                polynomial(x, c.get_unchecked(0))
            }
            Err(idx) if idx == splits.len() => {
                if cfg!(debug_assertions) {
                    log::warn!("Extrapolation is being used.");
                }
                polynomial(x, c.get_unchecked(idx - 2))
            }
            Err(idx) => polynomial(x, c.get_unchecked(idx - 1)),
        }
    }
}

/// Evaluates an arbitrary ratio of single-variable polynomials at a particular
/// point.  The coefficient of the polynomial in the numerator are given in `a`,
/// and `b` for the denominator.
///
/// If the array of denominator coefficients is empty, the denominator is
/// ignored entirely (or equivalently, is treated as being equal to `1.0`).
///
/// This function does not perform any checks on the coefficients.
pub fn polynomial_ratio(x: f64, (a, b): (&[f64], &[f64])) -> f64 {
    if b.is_empty() {
        polynomial(x, a)
    } else {
        polynomial(x, a) / polynomial(x, b)
    }
}

/// Evaluates an arbitrary piecewise ratio of single-variable polynomial at a
/// particular point.
///
/// The splits divide the domain in closed-open intervals (except for the last
/// one which is closed-closed) and thus the length of `splits` must be one
/// longer than the two list of coefficient arrays.  The [`polynomial_ratio`]
/// function used within each interval.
///
/// For values outside the domain of the piecewise polynomial, the first or last
/// polynomial is used for extrapolation.
pub fn piecewise_polynomial_ratio(x: f64, c: &[(&[f64], &[f64])], splits: &[f64]) -> f64 {
    debug_assert_eq!(
        splits.len(),
        c.len() + 1,
        "The number of splits must be one longer than number of coefficient arrays."
    );

    unsafe {
        match splits.binary_search_by(|s| s.partial_cmp(&x).unwrap()) {
            Ok(idx) if idx == splits.len() - 1 => polynomial_ratio(x, *c.get_unchecked(idx - 2)),
            Ok(idx) => polynomial_ratio(x, *c.get_unchecked(idx)),
            Err(0) => {
                if cfg!(debug_assertions) {
                    log::warn!("Extrapolation is being used.");
                }
                polynomial_ratio(x, *c.get_unchecked(0))
            }
            Err(idx) if idx == splits.len() => {
                if cfg!(debug_assertions) {
                    log::warn!("Extrapolation is being used.");
                }
                polynomial_ratio(x, *c.get_unchecked(idx - 2))
            }
            Err(idx) => polynomial_ratio(x, *c.get_unchecked(idx - 1)),
        }
    }
}

/// Evaluates a series of Chebyshev functions at x.
///
/// The argument should lie in the interval `[a, b]` (the rescaling to the
/// Chebyshev functions domain is done by the function), though this is *not*
/// checked by the function, allowing for extrapolation outside of the domain.
///
/// If the list of coefficients is empty, `0.0` is returned.
///
/// This function does not perform any checks on the coefficients.
pub fn chebyshev(x: f64, c: &[f64], a: f64, b: f64) -> f64 {
    if let Some(c0) = c.first() {
        let x = (2.0 * x - a - b) / (b - a);
        let x2 = 2.0 * x;

        // The Clenshaw algorithm relation is:
        //
        // > b(k, x) := c(k) + 2 x b(k + 1, x) - b(k + 2, x)
        //
        // with the final value being
        //
        // c(0) + x * b(1, x) - b(2, x)
        let (b1, b2) = c
            .iter()
            .skip(1)
            .rev()
            .fold((0.0, 0.0), |(b, bp), &ci| (ci + x2 * b - bp, b));

        c0 + x * b1 - b2
    } else {
        0.0
    }
}

/// Evaluates an arbitrary piecewise Chebyshev function at a particular point.
///
/// The splits divide the domain in closed-open intervals (except for the last
/// one which is closed-closed) and thus the length of `splits` must be one
/// longer than the two list of coefficient arrays.  The [`chebyshev`] function
/// used within each interval.
///
/// For values outside the domain of the piecewise polynomial, the first or last
/// polynomial is used for extrapolation.
pub fn piecewise_chebyshev(x: f64, c: &[&[f64]], splits: &[f64]) -> f64 {
    debug_assert_eq!(
        splits.len(),
        c.len() + 1,
        "The number of splits must be one longer than number of coefficient arrays."
    );

    unsafe {
        match splits.binary_search_by(|s| s.partial_cmp(&x).unwrap()) {
            Ok(idx) if idx == splits.len() - 1 => chebyshev(
                x,
                c.get_unchecked(idx - 2),
                *splits.get_unchecked(idx - 2),
                x,
            ),
            Ok(idx) => chebyshev(x, c.get_unchecked(idx), *splits.get_unchecked(idx - 1), x),
            Err(0) => {
                if cfg!(debug_assertions) {
                    log::warn!("Extrapolation is being used.");
                }
                chebyshev(
                    x,
                    c.get_unchecked(0),
                    *splits.get_unchecked(0),
                    *splits.get_unchecked(1),
                )
            }
            Err(idx) if idx == splits.len() => {
                if cfg!(debug_assertions) {
                    log::warn!("Extrapolation is being used.");
                }
                chebyshev(
                    x,
                    c.get_unchecked(idx - 2),
                    *splits.get_unchecked(idx - 2),
                    *splits.get_unchecked(idx - 1),
                )
            }
            Err(idx) => chebyshev(
                x,
                c.get_unchecked(idx - 1),
                *splits.get_unchecked(idx - 1),
                *splits.get_unchecked(idx),
            ),
        }
    }
}

/// Create a function from a module containing all the information for the split.
///
/// The module must contain the following:
///
/// - `fn lower(x: f64) -> f64` which provides a (generally Taylor series)
///   approximation near the lower bound of the domain.  This function is used
///   for values of `x` smaller than the first split.
/// - `fn upper(x: f64) -> f64` which provides a (generally Taylor series)
///   approximation near the upper bound of the domain.  This functinon is used
///   for values of `x` larger than the last split.
/// - `const COEFFICIENTS: &[...]` contains a list of list of coefficients, as
///   used in the relevant `piecewise` function.
/// - `const SPLITS: &[f64]` contains a list of splits.  Within the splits, the
///   relevant `piecewise` function is used.
///
/// The macro is used as:
///
/// ```ignore
/// approx_fn! {
///     fn foo(mod = path::to::module, type = approx_type, outer = fn, inner = fn);
/// }
/// ```
///
/// The `approx_type` can be either one of `polynomial`, `polynomial_ratio` or
/// `chebyshev`.  The `outer` and `inner` are names of functions which apply to
/// the result and argument before they go in the piecewise approximation.
/// These *do not* apply to `lower` and `upper`.
#[macro_export]
macro_rules! approx_fn {
    (
        $(#[$outer:meta])*
        fn $fn:ident(mod = $mod:ident, type = $t:tt, outer = $o:path, inner = $i:path)$(;)?
    ) => {
        approx_fn!{
            $(#[$outer])*
            () $fn(mod = $mod, type = $t, outer = $o, inner = $i);
        }
    };

    (
        $(#[$outer:meta])*
        pub fn $fn:ident(mod = $mod:ident, type = $t:tt, outer = $o:path, inner = $i:path)$(;)?
    ) => {
        approx_fn!{
            $(#[$outer])*
            (pub) $fn(mod = $mod, type = $t, outer = $o, inner = $i);
        }
    };

    (
        $(#[$outer:meta])*
        pub(crate) fn $fn:ident(mod = $mod:ident, type = $t:tt, outer = $o:path, inner = $i:path)$(;)?
    ) => {
        approx_fn!{
            $(#[$outer])*
            (pub(crate)) $fn(mod = $mod, type = $t, outer = $o, inner = $i);
        }
    };

    // Polynomial approximation
    (
        $(#[$outer:meta])*
        ($($vis:tt)*) fn $fn:ident(mod = $mod:ident, type = polynomial, outer = $o:path, inner = $i:path);
    ) => {
        $(#[$outer])*
        $($vis)* fn $fn(x: f64) -> f64 {
            let ix = $i(x);

            if ix < *$mod::SPLITS.first().unwrap() {
                $mod::lower(x)
            } else if ix > *$mod::SPLITS.last().unwrap() {
                $mod::upper(x)
            } else {
                $o($crate::approximations::piecewise_polynomial(
                    ix,
                    &$mod::COEFFICIENTS,
                    &$mod::SPLITS,
                ))
            }
        }
    };

    // Polynomial ratio approximation
    (
        $(#[$outer:meta])*
        ($($vis:tt)*) fn $fn:ident(mod = $mod:ident, type = ratio, outer = $o:path, inner = $i:path);
    ) => {
        $(#[$outer])*
        $($vis)* fn $fn(x: f64) -> f64 {
            let ix = $i(x);

            if ix < *$mod::SPLITS.first().unwrap() {
                $mod::lower(x)
            } else if ix > *$mod::SPLITS.last().unwrap() {
                $mod::upper(x)
            } else {
                $o($crate::approximations::piecewise_polynomial_ratio(
                    ix,
                    &$mod::COEFFICIENTS,
                    &$mod::SPLITS,
                ))
            }
        }
    };

    // Chebyshev approximation
    (
        $(#[$outer:meta])*
        ($($vis:tt)*) fn $fn:ident(mod = $mod:ident, type = chebyshev, outer = $o:path, inner = $i:path);
    ) => {
        $(#[$outer])*
        $($vis)* fn $fn(x: f64) -> f64 {
            let ix = $i(x);

            if ix < *$mod::SPLITS.first().unwrap() {
                $mod::lower(x)
            } else if ix > *$mod::SPLITS.last().unwrap() {
                $mod::upper(x)
            } else {
                $o($crate::approximations::piecewise_chebyshev(
                    ix,
                    &$mod::COEFFICIENTS,
                    &$mod::SPLITS,
                ))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;

    pub(crate) const COEFFICIENTS: [f64; 10] = [
        -0.069_933_1,
        0.632_915,
        0.909_728,
        0.890_992,
        0.119_426,
        0.075_200_9,
        0.223_267,
        0.185_299,
        0.017_051_7,
        -0.640_204,
    ];

    fn f(n: usize, x: f64) -> f64 {
        match n {
            0 => 0.0,
            1 => -0.069_933_1,
            2 => 0.632_915 * x - 0.069_933_1,
            3 => 0.909_728 * x.powi(2) + 0.632_915 * x - 0.069_933_1,
            4 => 0.890_992 * x.powi(3) + 0.909_728 * x.powi(2) + 0.632_915 * x - 0.069_933_1,
            5 => {
                0.119_426 * x.powi(4)
                    + 0.890_992 * x.powi(3)
                    + 0.909_728 * x.powi(2)
                    + 0.632_915 * x
                    - 0.069_933_1
            }
            6 => {
                0.075_200_9 * x.powi(5)
                    + 0.119_426 * x.powi(4)
                    + 0.890_992 * x.powi(3)
                    + 0.909_728 * x.powi(2)
                    + 0.632_915 * x
                    - 0.069_933_1
            }
            7 => {
                0.223_267 * x.powi(6)
                    + 0.075_200_9 * x.powi(5)
                    + 0.119_426 * x.powi(4)
                    + 0.890_992 * x.powi(3)
                    + 0.909_728 * x.powi(2)
                    + 0.632_915 * x
                    - 0.069_933_1
            }
            8 => {
                0.185_299 * x.powi(7)
                    + 0.223_267 * x.powi(6)
                    + 0.075_200_9 * x.powi(5)
                    + 0.119_426 * x.powi(4)
                    + 0.890_992 * x.powi(3)
                    + 0.909_728 * x.powi(2)
                    + 0.632_915 * x
                    - 0.069_933_1
            }
            9 => {
                0.017_051_7 * x.powi(8)
                    + 0.185_299 * x.powi(7)
                    + 0.223_267 * x.powi(6)
                    + 0.075_200_9 * x.powi(5)
                    + 0.119_426 * x.powi(4)
                    + 0.890_992 * x.powi(3)
                    + 0.909_728 * x.powi(2)
                    + 0.632_915 * x
                    - 0.069_933_1
            }
            10 => {
                -0.640_204 * x.powi(9)
                    + 0.017_051_7 * x.powi(8)
                    + 0.185_299 * x.powi(7)
                    + 0.223_267 * x.powi(6)
                    + 0.075_200_9 * x.powi(5)
                    + 0.119_426 * x.powi(4)
                    + 0.890_992 * x.powi(3)
                    + 0.909_728 * x.powi(2)
                    + 0.632_915 * x
                    - 0.069_933_1
            }
            _ => unimplemented!(),
        }
    }

    #[test]
    fn polynomial() {
        for n in 0..=COEFFICIENTS.len() {
            for x in -100..=100 {
                let x = x as f64;
                approx_eq(super::polynomial(x, &COEFFICIENTS[..n]), f(n, x), 8.0, 0.0);
            }
        }
    }

    #[test]
    fn ratio() {
        for n in 0..=COEFFICIENTS.len() {
            for m in 1..=COEFFICIENTS.len() {
                for x in -100..=100 {
                    let x = x as f64;
                    approx_eq(
                        super::polynomial_ratio(x, (&COEFFICIENTS[..n], &COEFFICIENTS[..m])),
                        f(n, x) / f(m, x),
                        8.0,
                        0.0,
                    );
                }
            }
        }
    }

    fn f_chebyshev(n: usize, x: f64) -> f64 {
        match n {
            0 => 1.0,
            1 => x,
            2 => 2.0 * x.powi(2) - 1.0,
            3 => 4.0 * x.powi(3) - 3.0 * x,
            4 => 8.0 * x.powi(4) - 8.0 * x.powi(2) + 1.0,
            5 => 16.0 * x.powi(5) - 20.0 * x.powi(3) + 5.0 * x,
            6 => 32.0 * x.powi(6) - 48.0 * x.powi(4) + 18.0 * x.powi(2) - 1.0,
            7 => 64.0 * x.powi(7) - 112.0 * x.powi(5) + 56.0 * x.powi(3) - 7.0 * x,
            8 => 128.0 * x.powi(8) - 256.0 * x.powi(6) + 160.0 * x.powi(4) - 32.0 * x.powi(2) + 1.0,
            9 => {
                256.0 * x.powi(9) - 576.0 * x.powi(7) + 432.0 * x.powi(5) - 120.0 * x.powi(3)
                    + 9.0 * x
            }
            10 => {
                512.0 * x.powi(10) - 1280.0 * x.powi(8) + 1120.0 * x.powi(6) - 400.0 * x.powi(4)
                    + 50.0 * x.powi(2)
                    - 1.0
            }
            _ => unimplemented!(),
        }
    }

    #[test]
    fn chebyshev() {
        for n in 0..=10 {
            let c: Vec<_> = std::iter::repeat(0.0)
                .take(n)
                .chain(std::iter::repeat(1.0).take(1))
                .collect();

            for x in -100..=100 {
                let x = (x as f64) / 100.0;
                approx_eq(
                    super::chebyshev(x, &c, -1.0, 1.0),
                    f_chebyshev(n, x),
                    8.0,
                    0.0,
                );
            }
        }
    }
}
