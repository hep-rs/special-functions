//! Bessel functions

use std::convert::identity;

mod i0;
mod i1;
mod i2;
mod i3;
mod i4;
mod i5;
mod i6;
mod i7;
mod i8;
mod i9;

mod j0;
mod j1;
mod j2;
mod j3;
mod j4;
mod j5;
mod j6;
mod j7;
mod j8;
mod j9;

mod k0;
mod k1;
mod k1_on_k2;
mod k2;
mod k3;
mod k4;
mod k5;
mod k6;
mod k7;
mod k8;
mod k9;

mod y0;
mod y1;
mod y2;
mod y3;
mod y4;
mod y5;
mod y6;
mod y7;
mod y8;
mod y9;

/// Approximation of modified Bessel function `$I_0(x)$` for all real `$x$`.
pub fn i0(x: f64) -> f64 {
    if x < 0.0 {
        _i0(-x)
    } else {
        _i0(x)
    }
}
approx_fn! {
    fn _i0(mod = i0, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Approximation of modified Bessel function `$I_1(x)$` for all real `$x$`.
pub fn i1(x: f64) -> f64 {
    if x < 0.0 {
        -_i1(-x)
    } else {
        _i1(x)
    }
}
approx_fn! {
    fn _i1(mod = i1, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Approximation of modified Bessel function `$I_2(x)$` for all real `$x$`.
pub fn i2(x: f64) -> f64 {
    if x < 0.0 {
        _i2(-x)
    } else {
        _i2(x)
    }
}
approx_fn! {
    fn _i2(mod = i2, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Approximation of modified Bessel function `$I_3(x)$` for all real `$x$`.
pub fn i3(x: f64) -> f64 {
    if x < 0.0 {
        -_i3(-x)
    } else {
        _i3(x)
    }
}
approx_fn! {
    fn _i3(mod = i3, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Approximation of modified Bessel function `$I_4(x)$` for all real `$x$`.
pub fn i4(x: f64) -> f64 {
    if x < 0.0 {
        _i4(-x)
    } else {
        _i4(x)
    }
}
approx_fn! {
    fn _i4(mod = i4, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Approximation of modified Bessel function `$I_5(x)$` for all real `$x$`.
pub fn i5(x: f64) -> f64 {
    if x < 0.0 {
        -_i5(-x)
    } else {
        _i5(x)
    }
}
approx_fn! {
    fn _i5(mod = i5, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Approximation of modified Bessel function `$I_6(x)$` for all real `$x$`.
pub fn i6(x: f64) -> f64 {
    if x < 0.0 {
        _i6(-x)
    } else {
        _i6(x)
    }
}
approx_fn! {
    fn _i6(mod = i6, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Approximation of modified Bessel function `$I_7(x)$` for all real `$x$`.
pub fn i7(x: f64) -> f64 {
    if x < 0.0 {
        -_i7(-x)
    } else {
        _i7(x)
    }
}
approx_fn! {
    fn _i7(mod = i7, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Approximation of modified Bessel function `$I_8(x)$` for all real `$x$`.
pub fn i8(x: f64) -> f64 {
    if x < 0.0 {
        _i8(-x)
    } else {
        _i8(x)
    }
}
approx_fn! {
    fn _i8(mod = i8, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Approximation of modified Bessel function `$I_9(x)$` for all real `$x$`.
pub fn i9(x: f64) -> f64 {
    if x < 0.0 {
        -_i9(-x)
    } else {
        _i9(x)
    }
}
approx_fn! {
    fn _i9(mod = i9, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

/// Approximation of modified Bessel function `$J_0(x)$` for all `$x$`.
pub fn j0(x: f64) -> f64 {
    if x < 0.0 {
        _j0(-x)
    } else {
        _j0(x)
    }
}
approx_fn! {
    fn _j0(mod = j0, type = chebyshev, outer = identity, inner = identity);
}

/// Approximation of modified Bessel function `$J_1(x)$` for all `$x$`.
pub fn j1(x: f64) -> f64 {
    if x < 0.0 {
        -_j1(-x)
    } else {
        _j1(x)
    }
}
approx_fn! {
    fn _j1(mod = j1, type = chebyshev, outer = identity, inner = identity);
}

/// Approximation of modified Bessel function `$J_2(x)$` for all `$x$`.
pub fn j2(x: f64) -> f64 {
    if x < 0.0 {
        _j2(-x)
    } else {
        _j2(x)
    }
}
approx_fn! {
    fn _j2(mod = j2, type = chebyshev, outer = identity, inner = identity);
}

/// Approximation of modified Bessel function `$J_3(x)$` for all `$x$`.
pub fn j3(x: f64) -> f64 {
    if x < 0.0 {
        -_j3(-x)
    } else {
        _j3(x)
    }
}
approx_fn! {
    fn _j3(mod = j3, type = chebyshev, outer = identity, inner = identity);
}

/// Approximation of modified Bessel function `$J_4(x)$` for all `$x$`.
pub fn j4(x: f64) -> f64 {
    if x < 0.0 {
        _j4(-x)
    } else {
        _j4(x)
    }
}
approx_fn! {
    fn _j4(mod = j4, type = chebyshev, outer = identity, inner = identity);
}

/// Approximation of modified Bessel function `$J_5(x)$` for all `$x$`.
pub fn j5(x: f64) -> f64 {
    if x < 0.0 {
        -_j5(-x)
    } else {
        _j5(x)
    }
}
approx_fn! {
    fn _j5(mod = j5, type = chebyshev, outer = identity, inner = identity);
}

/// Approximation of modified Bessel function `$J_6(x)$` for all `$x$`.
pub fn j6(x: f64) -> f64 {
    if x < 0.0 {
        _j6(-x)
    } else {
        _j6(x)
    }
}
approx_fn! {
    fn _j6(mod = j6, type = chebyshev, outer = identity, inner = identity);
}

/// Approximation of modified Bessel function `$J_7(x)$` for all `$x$`.
pub fn j7(x: f64) -> f64 {
    if x < 0.0 {
        -_j7(-x)
    } else {
        _j7(x)
    }
}
approx_fn! {
    fn _j7(mod = j7, type = chebyshev, outer = identity, inner = identity);
}

/// Approximation of modified Bessel function `$J_8(x)$` for all `$x$`.
pub fn j8(x: f64) -> f64 {
    if x < 0.0 {
        _j8(-x)
    } else {
        _j8(x)
    }
}
approx_fn! {
    fn _j8(mod = j8, type = chebyshev, outer = identity, inner = identity);
}

/// Approximation of modified Bessel function `$J_9(x)$` for all `$x$`.
pub fn j9(x: f64) -> f64 {
    if x < 0.0 {
        -_j9(-x)
    } else {
        _j9(x)
    }
}
approx_fn! {
    fn _j9(mod = j9, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$K_0(x)$` for all `$x > 0$`."#]
    pub fn k0(mod = k0, type = chebyshev, outer = f64::exp, inner = f64::ln);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$K_1(x)$` for all `$x > 0$`."#]
    pub fn k1(mod = k1, type = chebyshev, outer = f64::exp, inner = f64::ln);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$K_2(x)$` for all `$x > 0$`."#]
    pub fn k2(mod = k2, type = chebyshev, outer = f64::exp, inner = f64::ln);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$K_3(x)$` for all `$x > 0$`."#]
    pub fn k3(mod = k3, type = chebyshev, outer = f64::exp, inner = f64::ln);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$K_4(x)$` for all `$x > 0$`."#]
    pub fn k4(mod = k4, type = chebyshev, outer = f64::exp, inner = f64::ln);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$K_5(x)$` for all `$x > 0$`."#]
    pub fn k5(mod = k5, type = chebyshev, outer = f64::exp, inner = f64::ln);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$K_6(x)$` for all `$x > 0$`."#]
    pub fn k6(mod = k6, type = chebyshev, outer = f64::exp, inner = f64::ln);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$K_7(x)$` for all `$x > 0$`."#]
    pub fn k7(mod = k7, type = chebyshev, outer = f64::exp, inner = f64::ln);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$K_8(x)$` for all `$x > 0$`."#]
    pub fn k8(mod = k8, type = chebyshev, outer = f64::exp, inner = f64::ln);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$K_9(x)$` for all `$x > 0$`."#]
    pub fn k9(mod = k9, type = chebyshev, outer = f64::exp, inner = f64::ln);
}
approx_fn! {
    #[doc = r#"Approximatino of the ratio of Bessel function `$K_1(x) / K_2(x)$` for all `$x > 0$`."#]
    pub fn k1_on_k2(mod = k1_on_k2, type = chebyshev, outer = f64::exp, inner = f64::ln);
}

approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$Y_0(x)$` for all `$x > 0$`."#]
    pub fn y0(mod = y0, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$Y_1(x)$` for all `$x > 0$`."#]
    pub fn y1(mod = y1, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$Y_2(x)$` for all `$x > 0$`."#]
    pub fn y2(mod = y2, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$Y_3(x)$` for all `$x > 0$`."#]
    pub fn y3(mod = y3, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$Y_4(x)$` for all `$x > 0$`."#]
    pub fn y4(mod = y4, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$Y_5(x)$` for all `$x > 0$`."#]
    pub fn y5(mod = y5, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$Y_6(x)$` for all `$x > 0$`."#]
    pub fn y6(mod = y6, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$Y_7(x)$` for all `$x > 0$`."#]
    pub fn y7(mod = y7, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$Y_8(x)$` for all `$x > 0$`."#]
    pub fn y8(mod = y8, type = chebyshev, outer = identity, inner = identity);
}
approx_fn! {
    #[doc = r#"Approximation of modified Bessel function `$Y_9(x)$` for all `$x > 0$`."#]
    pub fn y9(mod = y9, type = chebyshev, outer = identity, inner = identity);
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::{f64, fs::File};

    #[test]
    fn i() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/bessel/i.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);
        let f = &[
            super::i0,
            super::i1,
            super::i2,
            super::i3,
            super::i4,
            super::i5,
            super::i6,
            super::i7,
            super::i8,
            super::i9,
        ];

        for (row, result) in rdr.deserialize().enumerate() {
            let values: [f64; 11] = result?;
            let x = values[0];
            let y = &values[1..];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(x);
                    approx_eq(nyi, yi, 8.0, 10f64.powi(-200)).map_err(|err| {
                        println!(
                            "[{}] BesselI[{}, {:e}] = {:e} but expected {:e}.",
                            row, i, x, nyi, yi
                        );
                        err
                    })?;
                }
            }
        }

        Ok(())
    }

    #[test]
    fn j() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/bessel/j.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);
        let f = &[
            super::j0,
            super::j1,
            super::j2,
            super::j3,
            super::j4,
            super::j5,
            super::j6,
            super::j7,
            super::j8,
            super::j9,
        ];

        for (row, result) in rdr.deserialize().enumerate() {
            let values: [f64; 11] = result?;
            let x = values[0];
            let y = &values[1..];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(x);
                    approx_eq(nyi, yi, 5.0, 10f64.powi(-200)).map_err(|err| {
                        println!(
                            "[{}] BesselJ[{}, {:e}] = {:e} but expected {:e}.",
                            row, i, x, nyi, yi
                        );
                        err
                    })?;
                }
            }
        }

        Ok(())
    }

    #[test]
    fn k() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/bessel/k.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);
        let f = &[
            super::k0,
            super::k1,
            super::k2,
            super::k3,
            super::k4,
            super::k5,
            super::k6,
            super::k7,
            super::k8,
            super::k9,
        ];

        for (row, result) in rdr.deserialize().enumerate() {
            let values: [f64; 11] = result?;
            let x = values[0];
            let y = &values[1..];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(x);
                    approx_eq(nyi, yi, 8.0, 10f64.powi(-200)).map_err(|err| {
                        println!(
                            "[{}] BesselK[{}, {:e}] = {:e} but expected {:e}.",
                            row, i, x, nyi, yi
                        );
                        err
                    })?;
                }
            }
        }

        Ok(())
    }

    #[test]
    fn y() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/bessel/y.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);
        let f = &[
            super::y0,
            super::y1,
            super::y2,
            super::y3,
            super::y4,
            super::y5,
            super::y6,
            super::y7,
            super::y8,
            super::y9,
        ];

        for (row, result) in rdr.deserialize().enumerate() {
            let values: [f64; 11] = result?;
            let x = values[0];
            let y = &values[1..];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(x);
                    approx_eq(nyi, yi, 5.0, 10f64.powi(-200)).map_err(|err| {
                        println!(
                            "[{}] BesselJ[{}, {:e}] = {:e} but expected {:e}.",
                            row, i, x, nyi, yi
                        );
                        err
                    })?;
                }
            }
        }

        Ok(())
    }

    #[test]
    fn k1_on_k2() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/bessel/k1_on_k2.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);

        for (row, result) in rdr.deserialize().enumerate() {
            let (x, y): (f64, f64) = result?;

            if !y.is_nan() {
                let ny = super::k1_on_k2(x);
                approx_eq(ny, y, 8.0, 0.0).map_err(|err| {
                    println!(
                        "[{}] K₁ / K₂({:e}) = {:e} but expected {:e}.",
                        row, x, ny, y
                    );
                    err
                })?;
            }
        }

        Ok(())
    }
}
