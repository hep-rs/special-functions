//! Basic functions.

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::{f64, fs::File};

    #[test]
    fn trig() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/basic/trig.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);
        let f = &[
            f64::sin,
            f64::cos,
            f64::tan,
            f64::sinh,
            f64::cosh,
            f64::tanh,
        ];

        for result in rdr.deserialize() {
            let values: [f64; 7] = result?;
            let x = values[0];
            let y = &values[1..];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(x);
                    approx_eq(nyi, yi, 16.0, 1e-200)?;
                }
            }
        }

        Ok(())
    }
}
