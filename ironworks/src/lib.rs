//! Modular toolkit for working with FFXIV data.

// Lint config
#![allow(clippy::module_inception)]
#![warn(missing_debug_implementations, missing_docs)]

mod error;
#[cfg(feature = "sqpack")]
pub mod sqpack;

pub use error::Error;
