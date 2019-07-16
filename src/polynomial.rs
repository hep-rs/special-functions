//! Utilities to handle polynomials

/// Evaluates an arbitrary single-variable polynomial at a particular point.
///
/// Given a list of coefficient \\(a = [a_0, a_1, \dots, a_n]\\), evaluates the
/// polynomial
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
    // a0 + x * (a1 + x * (a2 + x * (...)))
    a.iter().rev().fold(0.0, |ans, &ai| ai + x * ans)
}

/// Evaluates an arbitrary ratio of single-variable polynomials at a particular
/// point.  The coefficient of the polynomial in the numerator are given in `a`,
/// and `b` for the denominator.
///
/// If the list of denominator coefficients is empty, the denominator is set to
/// `1`.
///
/// # Warning
///
/// This function does not perform any checks on the coefficients.
#[inline]
pub fn polynomial_ratio(x: f64, a: &[f64], b: &[f64]) -> f64 {
    if b.is_empty() {
        polynomial(x, a)
    } else {
        polynomial(x, a) / polynomial(x, b)
    }
}

#[cfg(test)]
mod test {
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
mod bench {
    use test::{black_box, Bencher};

    use super::test::COEFFICIENTS;

    #[bench]
    fn len_9(b: &mut Bencher) {
        b.iter(|| {
            for x in -100..100 {
                let x = x as f64;
                black_box(super::polynomial(x, &COEFFICIENTS));
            }
        })
    }
}
