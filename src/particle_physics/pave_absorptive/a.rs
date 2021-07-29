/// Absorptive part of the Passarin-Veltman coefficient function
/// `$\boldsymbol{A}$`.
///
/// ```math
/// \boldsymbol{A}_{\underbrace{0\dots0}_{2r}}(m)
/// ```
///
/// Note that this is always 0 for all input values.  This is implemented for
/// the sake of completeness with the other functions.
pub fn a(r: i32, m: f64) -> f64 {
    debug_assert!(r >= 0, "r must be positive");
    debug_assert!(m >= 0.0, "m must be positive");

    0.0
}

#[cfg(test)]
mod tests {
    use crate::utilities::test::*;
    use std::{f64, fs::File};

    #[test]
    fn a() -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open("tests/data/particle_physics/pave_absorptive/a.csv.zst")?;
        let mut rdr = csv::Reader::from_reader(zstd::Decoder::new(&mut f)?);
        let f = super::a;

        for result in rdr.deserialize() {
            let (r, m0, y): (f64, f64, f64) = result?;
            let r = r as i32;

            if !y.is_nan() {
                let ny = f(r, m0);
                // println!("A({}, {:e}) = {:e} [{:e}]", r, m0, ny, y);
                approx_eq(ny, y, 8.0, 10f64.powi(-200))?;
            }
        }

        Ok(())
    }
}
