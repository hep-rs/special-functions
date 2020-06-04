//! Basic functions.

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::f64;

    #[test]
    #[ignore]
    fn trig() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path("tests/data/basic/trig.csv")?;
        let f = &[f64::sin, f64::cos, f64::tan];

        for result in rdr.deserialize() {
            let values: [f64; 4] = result?;
            let x = values[0];
            let y = &values[1..];

            for i in 0..f.len() {
                let fi = f[i];
                let yi = y[i];

                if !yi.is_nan() {
                    let nyi = fi(x);
                    // println!("Trig{}({:e}) = {:e} [{:e}]", i, x, yi, nyi);
                    approx_eq(nyi, yi, 8.0, 10f64.powi(-200));
                }
            }
        }

        Ok(())
    }
}
