/// Evaluate the absorptive part of Passarin-Veltman coefficient function
///
/// \\begin{equation}
///   \boldsymbol{A}_{\underbrace{0\dots0}_{2r}}(m)
/// \\end{equation}
pub fn a(r: i32, m: f64) -> f64 {
    debug_assert!(r >= 0, "r must be positive");
    debug_assert!(m >= 0.0, "m must be positive");

    0.0
}
