#![allow(missing_docs)]

mod approximations;
mod bessel;

use criterion::{criterion_group, criterion_main};

criterion_group!(
    benches,
    approximations::polynomial,
    approximations::polynomial_ratio,
    approximations::chebyshev,
    bessel::bessel_i_sorted,
    bessel::bessel_i_random,
    bessel::bessel_j_sorted,
    bessel::bessel_j_random,
    bessel::bessel_k_sorted,
    bessel::bessel_k_random,
    bessel::bessel_y_sorted,
    bessel::bessel_y_random
);
criterion_main!(benches);
