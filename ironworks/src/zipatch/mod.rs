//! Adapters to allow working with game data directly out of ZiPatch files.

mod lookup;
mod repository;
mod section;
mod utility;
mod view;
mod zipatch;

pub use {
	repository::{Patch, PatchRepository},
	view::View,
	zipatch::ZiPatch,
};
