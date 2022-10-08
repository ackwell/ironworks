//! Modular toolkit for working with FFXIV data.

// Lint config
#![allow(clippy::module_inception)]
#![warn(missing_debug_implementations, missing_docs)]
// Doc config
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

mod error;
mod ironworks;
mod utility;

#[cfg(feature = "excel")]
pub mod excel;
#[cfg(feature = "ffxiv")]
pub mod ffxiv;
pub mod file;
pub mod sestring;
#[cfg(feature = "sqpack")]
pub mod sqpack;
#[cfg(feature = "zipatch")]
pub mod zipatch;

pub use {
	error::{Error, ErrorValue},
	ironworks::{Ironworks, Resource},
};
