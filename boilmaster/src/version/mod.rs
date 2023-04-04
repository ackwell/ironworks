mod manager;
mod patch;
mod persist;
mod thaliak;
mod version;

pub use {
	manager::{Config, Manager, PatchList},
	patch::Patch,
};
