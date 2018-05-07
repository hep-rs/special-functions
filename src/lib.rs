//! Library providing pure rust implementation of various special functions,
//! with particular focus to high-energy particle physics.
//!
//! This library will attempt to provide full 64-bit precision by default.  In
//! circumstances where such precision may be particularly slow to achieve, a
//! less accurate implementation may be provided.
//!
//! **This library is still undergoing development.**
//!
//! [![Crates.io](https://img.shields.io/crates/v/hep-boltzmann-solver.svg)](https://crates.io/crates/hep-boltzmann-solver)
//! [![Travis](https://img.shields.io/travis/hep-rs/boltzmann-solver/master.svg)](https://travis-ci.org/hep-rs/boltzmann-solver)
//! [![Codecov](https://img.shields.io/codecov/c/github/hep-rs/boltzmann-solver/master.svg)](https://codecov.io/gh/hep-rs/boltzmann-solver)
//!
//! Licensed under [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html).
//!
//! The documentation makes use of [MathJax](https://www.mathjax.org/) in order
//! to display mathematics.  A version of the documentation with MathJax
//! integrated is available
//! [here](https://hep.rs/special-functions/special-functions).

#![cfg_attr(feature = "nightly", feature(test))]
#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]

#[macro_use]
extern crate log;

#[cfg(test)]
extern crate csv;
#[cfg(feature = "nightly")]
extern crate test;

pub mod polylog;
pub mod bessel;
pub mod polynomial;
pub mod interpolation;

mod utilities;
