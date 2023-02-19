//! Adapters to allow working with game data directly out of ZiPatch files.

mod lookup;
mod repository;
mod version;
mod zipatch;

pub use {
	repository::{Patch, PatchRepository},
	version::{Version, VersionSpecifier},
	zipatch::ZiPatch,
};
