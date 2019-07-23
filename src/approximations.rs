//! Approximations

/// Evaluates an arbitrary single-variable polynomial at a particular point.
///
/// Given an array of coefficients \\(a = [a_0, a_1, \dots, a_n]\\), evaluates
/// the polynomial
/// \\[
///     p(x) = a_0 + a_1 x + a_2 x\^2 + \dots + a_n x\^n
/// \\]
/// at the specified value of \\(x\\).
///
/// If the list of coefficients is empty, the function returns `0.0`.
///
/// # Warning
///
/// This function does not perform any checks on the coefficients.
#[inline]
pub fn polynomial(x: f64, a: &[f64]) -> f64 {
    // This expands the polynomial as:
    //
    // > a0 + x * (a1 + x * (a2 + x * (...)))
    //
    // This is significantly more efficient than computing the mathematically
    // equivalent
    //
    // > a0 + a1 * x + a2 * x.powi(2) + ...
    a.iter().rev().fold(0.0, |ans, &ai| ans.mul_add(x, ai))
}

/// Evaluates an arbitrary piecewise single-variable polynomial at a particular
/// point.
///
/// The splits divide the domain in closed-open intervals (except for the last
/// one which is closed-closed) and thus the length of `splits` must be one
/// longer than the list of coefficient arrays.  The [`polynomial`] function
/// used within each interval.
///
/// # Warning
///
/// For values outside the domain of the piecewise polynomial, the first or last
/// polynomial is used for extrapolation.
pub fn piecewise_polynomial(x: f64, a: &[&[f64]], splits: &[f64]) -> f64 {
    debug_assert_eq!(
        splits.len(),
        a.len() + 1,
        "The number of splits must be one longer than number of coefficient arrays."
    );

    unsafe {
        match splits.binary_search_by(|s| s.partial_cmp(&x).unwrap()) {
            Ok(idx) if idx == splits.len() - 1 => polynomial(x, a.get_unchecked(idx - 2)),
            Ok(idx) => polynomial(x, a.get_unchecked(idx)),
            Err(0) => {
                if cfg!(debug_assertions) {
                    log::warn!("Extrapolation is being used.");
                }
                polynomial(x, a.get_unchecked(0))
            }
            Err(idx) if idx == splits.len() => {
                if cfg!(debug_assertions) {
                    log::warn!("Extrapolation is being used.");
                }
                polynomial(x, a.get_unchecked(idx - 2))
            }
            Err(idx) => polynomial(x, a.get_unchecked(idx - 1)),
        }
    }
}

/// Evaluates an arbitrary ratio of single-variable polynomials at a particular
/// point.  The coefficient of the polynomial in the numerator are given in `a`,
/// and `b` for the denominator.
///
/// # Implementation Details
///
/// If the array of denominator coefficients is empty, the denominator is
/// ignored entirely (or equivalently, is treated as being equal to `1.0`).
///
/// # Warning
///
/// This function does not perform any checks on the coefficients.
pub fn polynomial_ratio(x: f64, a: &[f64], b: &[f64]) -> f64 {
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
/// # Warning
///
/// For values outside the domain of the piecewise polynomial, the first or last
/// polynomial is used for extrapolation.
pub fn piecewise_polynomial_ratio(x: f64, a: &[&[f64]], b: &[&[f64]], splits: &[f64]) -> f64 {
    debug_assert_eq!(
        a.len(),
        b.len(),
        "The list of numerator and denominator coefficient arrays must be equal in length."
    );
    debug_assert_eq!(
        splits.len(),
        a.len() + 1,
        "The number of splits must be one longer than number of coefficient arrays."
    );

    unsafe {
        match splits.binary_search_by(|s| s.partial_cmp(&x).unwrap()) {
            Ok(idx) if idx == splits.len() - 1 => {
                polynomial_ratio(x, a.get_unchecked(idx - 2), b.get_unchecked(idx - 2))
            }
            Ok(idx) => polynomial_ratio(x, a.get_unchecked(idx), b.get_unchecked(idx)),
            Err(0) => {
                if cfg!(debug_assertions) {
                    log::warn!("Extrapolation is being used.");
                }
                polynomial_ratio(x, a.get_unchecked(0), b.get_unchecked(0))
            }
            Err(idx) if idx == splits.len() => {
                if cfg!(debug_assertions) {
                    log::warn!("Extrapolation is being used.");
                }
                polynomial_ratio(x, a.get_unchecked(idx - 2), b.get_unchecked(idx - 2))
            }
            Err(idx) => polynomial_ratio(x, a.get_unchecked(idx - 1), b.get_unchecked(idx - 1)),
        }
    }
}

#[macro_export]
macro_rules! approx_fn {
    (
        $(#[$outer:meta])*
        fn $fn:ident($mod:ident, ratio);
    ) => {
        $(#[$outer])*
        fn $fn(x: f64) -> f64 {
            $crate::approximations::piecewise_polynomial_ratio(
                x,
                &$mod::NUMERATORS,
                &$mod::DENOMINATORS,
                &$mod::SPLITS,
            ) + if x < k0::SPLITS[1] {
                $mod::lower(x)
            } else if x > $mod::SPLITS[$mod::SPLITS.len() - 2] {
                $mod::upper(x)
            } else {
                0.0
            }
        }
    };
    (
        $(#[$outer:meta])*
        pub fn $fn:ident($mod:ident, ratio);
    ) => {
        $(#[$outer])*
        pub fn $fn(x: f64) -> f64 {
            $crate::approximations::piecewise_polynomial_ratio(
                x,
                &$mod::NUMERATORS,
                &$mod::DENOMINATORS,
                &$mod::SPLITS,
            ) + if x < k0::SPLITS[1] {
                $mod::lower(x)
            } else if x > $mod::SPLITS[$mod::SPLITS.len() - 2] {
                $mod::upper(x)
            } else {
                0.0
            }
        }
    };

    (
        $(#[$outer:meta])*
        fn $fn:ident($mod:ident, poly);
    ) => {
        $(#[$outer])*
        fn $fn(x: f64) -> f64 {
            $crate::approximations::piecewise_polynomial(
                x,
                &$mod::NUMERATORS,
                &$mod::SPLITS,
            ) + if x < k0::SPLITS[1] {
                $mod::lower(x)
            } else if x > $mod::SPLITS[$mod::SPLITS.len() - 2] {
                $mod::upper(x)
            } else {
                0.0
            }
        }
    };
    (
        $(#[$outer:meta])*
        pub fn $fn:ident($mod:ident, poly);
    ) => {
        $(#[$outer])*
        pub fn $fn(x: f64) -> f64 {
            $crate::approximations::piecewise_polynomial(
                x,
                &$mod::NUMERATORS,
                &$mod::SPLITS,
            ) + if x < k0::SPLITS[1] {
                $mod::lower(x)
            } else if x > $mod::SPLITS[$mod::SPLITS.len() - 2] {
                $mod::upper(x)
            } else {
                0.0
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;

    pub(crate) const COEFFICIENTS: [f64; 9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

    fn f(x: f64) -> f64 {
        1.0 + 2.0 * x
            + 3.0 * x.powi(2)
            + 4.0 * x.powi(3)
            + 5.0 * x.powi(4)
            + 6.0 * x.powi(5)
            + 7.0 * x.powi(6)
            + 8.0 * x.powi(7)
            + 9.0 * x.powi(8)
    }

    macro_rules! poly_test {
        ($name:ident, $n:expr, $f:expr) => {
            #[test]
            fn $name() {
                for x in -100..100 {
                    let x = x as f64;
                    approx_eq(super::polynomial(x, &COEFFICIENTS[..$n]), $f(x), 14.0, 0.0);
                }
            }
        };
    }

    poly_test!(len_0, 0, |_| 0.0);
    poly_test!(len_1, 1, |_| 1.0);
    poly_test!(len_2, 2, |x| 1.0 + 2.0 * x);
    poly_test!(len_9, 9, f);

    macro_rules! ratio_test {
        ($name:ident, $n:expr, $m:expr, $f:expr) => {
            #[test]
            fn $name() {
                for x in -100..100 {
                    let x = x as f64;
                    approx_eq(
                        super::polynomial_ratio(x, &COEFFICIENTS[..$n], &COEFFICIENTS[..$m]),
                        $f(x),
                        14.0,
                        0.0,
                    );
                }
            }
        };
    }

    ratio_test!(ratio_0_0, 0, 0, |_| 0.0);
    ratio_test!(ratio_1_0, 1, 0, |_| 1.0);
    ratio_test!(ratio_2_0, 2, 0, |x| 1.0 + 2.0 * x);
    ratio_test!(ratio_9_0, 9, 0, f);
    ratio_test!(ratio_0_1, 0, 1, |_| 0.0);
    ratio_test!(ratio_1_1, 1, 1, |_| 1.0);
    ratio_test!(ratio_2_1, 2, 1, |x| 1.0 + 2.0 * x);
    ratio_test!(ratio_9_1, 9, 1, f);
    ratio_test!(ratio_0_2, 0, 2, |_| 0.0);
    ratio_test!(ratio_1_2, 1, 2, |x| 1.0 / (1.0 + 2.0 * x));
    ratio_test!(ratio_2_2, 2, 2, |_| 1.0);
    ratio_test!(ratio_9_2, 9, 2, |x| f(x) / (1.0 + 2.0 * x));
    ratio_test!(ratio_0_9, 0, 9, |_| 0.0);
    ratio_test!(ratio_1_9, 1, 9, |x| 1.0 / f(x));
    ratio_test!(ratio_2_9, 2, 9, |x| (1.0 + 2.0 * x) / f(x));
    ratio_test!(ratio_9_9, 9, 9, |_| 1.0);
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod benches {
    use test::{black_box, Bencher};

    use super::tests::COEFFICIENTS;

    #[bench]
    fn poly_9(b: &mut Bencher) {
        b.iter(|| {
            for x in -100..100 {
                let x = x as f64;
                black_box(super::polynomial(x, &COEFFICIENTS));
            }
        })
    }

    #[bench]
    fn ratio_0_0(b: &mut Bencher) {
        b.iter(|| {
            for x in -100..100 {
                let x = x as f64;
                black_box(super::polynomial_ratio(x, &[], &[]));
            }
        })
    }

    #[bench]
    fn ratio_9_0(b: &mut Bencher) {
        b.iter(|| {
            for x in -100..100 {
                let x = x as f64;
                black_box(super::polynomial_ratio(x, &COEFFICIENTS, &[]));
            }
        })
    }

    #[bench]
    fn ratio_0_9(b: &mut Bencher) {
        b.iter(|| {
            for x in -100..100 {
                let x = x as f64;
                black_box(super::polynomial_ratio(x, &[], &COEFFICIENTS));
            }
        })
    }

    #[bench]
    fn ratio_9_9(b: &mut Bencher) {
        b.iter(|| {
            for x in -100..100 {
                let x = x as f64;
                black_box(super::polynomial_ratio(x, &COEFFICIENTS, &COEFFICIENTS));
            }
        })
    }

}
