//! Library providing pure rust implementation of various special functions,
//! with particular focus to high-energy particle physics.
//!
//! This library will attempt to provide full 64-bit precision by default.  In
//! circumstances where such precision may be particularly slow to achieve, a
//! less accurate implementation may be provided.
//!
//! **This library is still undergoing development.**
//!
//! [![Crates.io](https://img.shields.io/crates/v/hep-special-functions.svg)](https://crates.io/crates/hep-special-functions)
//! [![Travis](https://img.shields.io/travis/hep-rs/special-functions/master.svg)](https://travis-ci.org/hep-rs/special-functions)
//! [![Codecov](https://img.shields.io/codecov/c/github/hep-rs/special-functions/master.svg)](https://codecov.io/gh/hep-rs/special-functions)
//!
//! Licensed under [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html).

#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(feature = "nightly")]
extern crate test;

#[macro_use]
pub mod approximations;
pub mod basic;
pub mod bessel;
pub mod other;
pub mod particle_physics;
mod utilities;
