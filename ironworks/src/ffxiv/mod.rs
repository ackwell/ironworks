//! FFXIV specialisations and utilities for ironworks.

mod fs;
mod mapper;

pub use {
	fs::FsResource,
	mapper::{Language, Mapper},
};
