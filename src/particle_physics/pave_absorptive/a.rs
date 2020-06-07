/// Absorptive part of the Passarin-Veltman coefficient function
///
/// \\begin{equation}
///   \boldsymbol{A}_{\underbrace{0\dots0}_{2r}}(m)
/// \\end{equation}
///
/// Note that this is always 0, but is implemented for the sake of consistency.
pub fn a(r: i32, m: f64) -> f64 {
    debug_assert!(r >= 0, "r must be positive");
    debug_assert!(m >= 0.0, "m must be positive");

    0.0
}

#[cfg(test)]
mod tests {}
