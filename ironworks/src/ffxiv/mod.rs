//! FFXIV specialisations and utilities for ironworks.

mod fs;

mod mapper;
#[cfg(feature = "sqpack")]
mod sqpack;

#[cfg(feature = "sqpack")]
pub use sqpack::SqPackResource;
pub use {
	fs::FsResource,
	mapper::{Language, Mapper},
};
