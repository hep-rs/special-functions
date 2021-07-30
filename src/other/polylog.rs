//! Polylogarithms functions

mod li2;
mod li3;
mod li4;
mod li5;
mod li6;
mod li7;
mod li8;
mod li9;

/// Approximation of polylogarithm function `$\Li_0(x)$` for all `$x < 1$`.
pub fn li0(x: f64) -> f64 {
    x / (1.0 - x)
}

/// Approximation of polylogarithm function `$\Li_1(x)$` for all `$x < 1$`.
pub fn li1(x: f64) -> f64 {
    -(-x).ln_1p()
}

/// Approximation of polylogarithm function `$\Li_2(x)$` for all `$x < 1$`.
pub fn li2(x: f64) -> f64 {
    li2::eval(x)
}

/// Approximation of polylogarithm function `$\Li_3(x)$` for all `$x < 1$`.
pub fn li3(x: f64) -> f64 {
    li3::eval(x)
}

/// Approximation of polylogarithm function `$\Li_4(x)$` for all `$x < 1$`.
pub fn li4(x: f64) -> f64 {
    li4::eval(x)
}

/// Approximation of polylogarithm function `$\Li_5(x)$` for all `$x < 1$`.
pub fn li5(x: f64) -> f64 {
    li5::eval(x)
}

/// Approximation of polylogarithm function `$\Li_6(x)$` for all `$x < 1$`.
pub fn li6(x: f64) -> f64 {
    li6::eval(x)
}

/// Approximation of polylogarithm function `$\Li_7(x)$` for all `$x < 1$`.
pub fn li7(x: f64) -> f64 {
    li7::eval(x)
}

/// Approximation of polylogarithm function `$\Li_8(x)$` for all `$x < 1$`.
pub fn li8(x: f64) -> f64 {
    li8::eval(x)
}

/// Approximation of polylogarithm function `$\Li_9(x)$` for all `$x < 1$`.
pub fn li9(x: f64) -> f64 {
    li9::eval(x)
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::{f64, fs::File};

    #[test]
    fn li() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/other/polylog.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);

        let f = &[
            super::li0,
            super::li1,
            super::li2,
            super::li3,
            super::li4,
            super::li5,
            super::li6,
            super::li7,
            super::li8,
            super::li9,
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
                    approx_eq(nyi, yi, if i == 1 { 5.0 } else { 7.0 }, 10f64.powi(-300)).map_err(
                        |err| {
                            println!(
                                "[{}] Li{}({:e}) = {:e} but expected {:e}.",
                                row, i, x, nyi, yi
                            );
                            err
                        },
                    )?;
                }
            }
        }

        Ok(())
    }
}
