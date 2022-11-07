// Temp
#![allow(missing_docs, clippy::new_without_default)]

mod lookup;
mod repository;
mod temp_sqpack;
mod version;
mod zipatch;

pub use {
	repository::PatchRepository,
	version::{Version, VersionSpecifier},
	zipatch::ZiPatch,
};
