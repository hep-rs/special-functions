//! Other function approximations

use approx_fn;
use std::convert::identity;

mod harmonic_number;

approx_fn! {
    #[doc = "Approximation of the Harmonic number extended to all number \\(x > 0\\).

The \\(n\\)th harmonic number is defined as

\\begin{equation}
    H_n \\defeq \\sum_{i = 1}^n \\frac{1}{i}
\\end{equation}
"]
    (pub) fn harmonic_number(mod = harmonic_number, type = chebyshev, outer = identity, inner = identity);
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::f64;

    #[test]
    fn harmonic_number() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path("tests/data/other/harmonic_number.csv")?;
        let f = super::harmonic_number;

        for result in rdr.deserialize() {
            let (x, y): (f64, f64) = result?;

            if !y.is_nan() {
                let ny = f(x);
                println!("H({:e}) = {:e} [{:e}]", x, y, ny);
                approx_eq(ny, y, 10.0, 10f64.powi(-200));
            }
        }

        Ok(())
    }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod bench {
    use test::Bencher;

    #[bench]
    fn harmonic_number(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<_> = csv::Reader::from_path("tests/data/other/harmonic_number.csv")?
            .into_deserialize()
            .map(|x| {
                let x: [f64; 2] = x.unwrap();
                x[0]
            })
            .collect();

        b.iter(|| {
            for &x in &data {
                test::black_box(super::harmonic_number(x));
            }
        });

        Ok(())
    }
}
