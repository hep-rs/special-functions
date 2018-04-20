//! Numerical approximations for special functions.

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

mod utilities;
