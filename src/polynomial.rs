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

#[cfg(test)]
mod test {
    use utilities::test::*;

    pub(crate) const COEFFICIENTS: [f64; 9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

    #[test]
    fn len_0() {
        for x in -100..100 {
            let x = x as f64;
            approx_eq(super::polynomial(x, &COEFFICIENTS[..0]), 0.0, 14.0, 0.0);
        }
    }

    #[test]
    fn len_1() {
        for x in -100..100 {
            let x = x as f64;
            approx_eq(super::polynomial(x, &COEFFICIENTS[..1]), 1.0, 14.0, 0.0);
        }
    }

    #[test]
    fn len_2() {
        for x in -100..100 {
            let x = x as f64;
            approx_eq(
                super::polynomial(x, &COEFFICIENTS[..2]),
                1.0 + 2.0 * x,
                14.0,
                0.0,
            );
        }
    }

    #[test]
    fn len_3() {
        for x in -100..100 {
            let x = x as f64;
            approx_eq(
                super::polynomial(x, &COEFFICIENTS[..3]),
                1.0 + 2.0 * x + 3.0 * x.powi(2),
                14.0,
                0.0,
            );
        }
    }

    #[test]
    fn len_9() {
        for x in -100..100 {
            let x = x as f64;
            approx_eq(
                super::polynomial(x, &COEFFICIENTS),
                1.0 + 2.0 * x + 3.0 * x.powi(2) + 4.0 * x.powi(3) + 5.0 * x.powi(4)
                    + 6.0 * x.powi(5) + 7.0 * x.powi(6) + 8.0 * x.powi(7)
                    + 9.0 * x.powi(8),
                14.0,
                0.0,
            );
        }
    }
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
