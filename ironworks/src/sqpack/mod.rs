//! Tools for working with the SqPack package format.

mod file;
mod index;
mod resource;
mod sqpack;

pub use {
	resource::{Hierarchy, Resource},
	sqpack::SqPack,
};
