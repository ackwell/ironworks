mod key;
mod manager;
mod patch;
mod persist;
mod thaliak;
mod version;

pub use {
	key::VersionKey,
	manager::{Config, Manager, PatchList},
	patch::Patch,
};
