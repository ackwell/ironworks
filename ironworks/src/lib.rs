//! Modular toolkit for working with FFXIV data.

// Lint config
#![allow(clippy::module_inception)]
#![warn(missing_debug_implementations, missing_docs)]

mod error;
mod ironworks;
mod utility;

#[cfg(feature = "excel")]
pub mod excel;
#[cfg(feature = "ffxiv")]
pub mod ffxiv;
pub mod sestring;
#[cfg(feature = "sqpack")]
pub mod sqpack;

pub use {
	error::{Error, ErrorValue},
	ironworks::{Ironworks, Provider},
};
