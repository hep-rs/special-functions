#![allow(missing_docs)]

mod approximations;
mod bessel;
mod other;

use criterion::{criterion_group, criterion_main};

criterion_group!(
    benches,
    approximations::polynomial,
    approximations::polynomial_ratio,
    approximations::chebyshev,
    bessel::bessel_i,
    bessel::bessel_j,
    bessel::bessel_k,
    bessel::bessel_y,
    other::gamma,
    other::harmonic_number,
    other::binomial,
    other::polylog,
);
criterion_main!(benches);
