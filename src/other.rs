//! Miscellaneous functions

use std::convert::identity;

pub mod polylog;

mod gamma;
mod harmonic_number;

approx_fn! {
    #[doc = r#"Approximation of the Harmonic number extended to all number `$x > 0$`.

The `$n$`th harmonic number is defined as

```math
H_n \defeq \sum_{i = 1}^n \frac{1}{i}
```
"#]
    (pub) fn harmonic_number(mod = harmonic_number, type = chebyshev, outer = identity, inner = identity);
}

approx_fn! {
    #[doc = r#"Approximatino of the gamma function for `$x > 0$`."#]
    (pub) fn gamma(mod = gamma, type = chebyshev, outer = identity, inner = identity);
}

/// Binomial coefficient
///
/// ```math
/// \binom{n}{k}
/// ```
pub fn binomial(n: i32, k: i32) -> f64 {
    if k > n || k < 0 {
        0.0
    } else if (n - k) < k {
        (1..=(n - k))
            .map(|i| (n + 1 - i) as f64 / i as f64)
            .product()
    } else {
        (1..=k).map(|i| (n + 1 - i) as f64 / i as f64).product()
    }
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::{f64, fs::File};

    #[test]
    fn harmonic_number() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/other/harmonic_number.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(ruzstd::StreamingDecoder::new(&mut f)?);
        let f = super::harmonic_number;

        for result in rdr.deserialize() {
            let (x, y): (f64, f64) = result?;

            if !y.is_nan() {
                let ny = f(x);
                // println!("H({:e}) = {:e} [{:e}]", x, ny, y);
                approx_eq(ny, y, 8.0, 10f64.powi(-200))?;
            }
        }

        Ok(())
    }

    #[test]
    fn gamma() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/other/gamma.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(ruzstd::StreamingDecoder::new(&mut f)?);
        let f = super::gamma;

        for result in rdr.deserialize() {
            let (x, y): (f64, f64) = result?;

            if !y.is_nan() {
                let ny = f(x);
                // println!("Γ({:e}) = {:e} [{:e}]", x, ny, y);
                approx_eq(ny, y, 8.0, 10f64.powi(-200))?;
            }
        }

        Ok(())
    }

    #[test]
    fn binomial() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/other/binomial.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(ruzstd::StreamingDecoder::new(&mut f)?);
        let f = super::binomial;

        for result in rdr.deserialize() {
            let (n, k, y): (f64, f64, f64) = result?;
            let n = n as i32;
            let k = k as i32;

            if !y.is_nan() {
                let ny = f(n, k);
                // println!("Binom({}, {}) = {:e} [{:e}]", n, k, ny, y);
                approx_eq(ny, y, 8.0, 10f64.powi(-200))?;
            }
        }

        Ok(())
    }
}
