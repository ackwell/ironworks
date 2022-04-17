//! FFXIV specialisations and utilities for ironworks.

mod fs;

#[cfg(feature = "sqpack")]
mod sqpack;

pub use fs::FsResource;

#[cfg(feature = "sqpack")]
pub use sqpack::{Language, SqpackResource};
