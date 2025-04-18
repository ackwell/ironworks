use std::{fmt::Debug, sync::Arc};

use crate::{
	sqpack,
	utility::{HashMapCache, HashMapCacheExt},
};

use super::{
	error::{Error, Result},
	file::File,
	index::Index,
};

const CATEGORIES: &[Option<&str>] = &[
	/* 0x00 */ Some("common"),
	/* 0x01 */ Some("bgcommon"),
	/* 0x02 */ Some("bg"),
	/* 0x03 */ Some("cut"),
	/* 0x04 */ Some("chara"),
	/* 0x05 */ Some("shader"),
	/* 0x06 */ Some("ui"),
	/* 0x07 */ Some("sound"),
	/* 0x08 */ Some("vfx"),
	/* 0x09 */ Some("ui_script"),
	/* 0x0a */ Some("exd"),
	/* 0x0b */ Some("game_script"),
	/* 0x0c */ Some("music"),
	/* 0x0d */ None,
	/* 0x0e */ None,
	/* 0x0f */ None,
	/* 0x10 */ None,
	/* 0x11 */ None,
	/* 0x12 */ Some("_sqpack_test"),
	/* 0x13 */ Some("_debug"),
];

// While this is pretty trivially computed, even just going to ex9 gives us a lead time of a good 10 years or so.
const REPOSITORIES: &[&str] = &[
	"ffxiv", "ex1", "ex2", "ex3", "ex4", "ex5", "ex6", "ex7", "ex8", "ex9",
];

/// Representation of a group of SqPack package files forming a single data set.
#[derive(Debug)]
pub struct SqPack<R> {
	resource: Arc<R>,

	indexes: HashMapCache<(u8, u8), Index<R>>,
}

impl<R: sqpack::Resource> SqPack<R> {
	/// Build a representation of SqPack packages. The provided resource will be
	/// queried for lookups as required to fulfil SqPack requests.
	pub fn new(resource: R) -> Self {
		Self {
			resource: resource.into(),

			indexes: Default::default(),
		}
	}

	/// Get the version string for the file at `path`.
	pub fn version(&self, path: &str) -> Result<String> {
		let (repository, _) = self.path_metadata(&path.to_lowercase())?;
		self.resource.version(repository)
	}

	/// Read the file at `path` from SqPack.
	pub fn file(&self, path: &str) -> Result<File<R::File>> {
		// SqPack paths are always lower case.
		let path = path.to_lowercase();

		// Look up the location of the requested path.
		let (repository, category) = self.path_metadata(&path)?;

		let location = self
			.indexes
			.try_get_or_insert((repository, category), || {
				Index::new(repository, category, self.resource.clone())
			})?
			.find(&path)?;

		// Build a File representation.
		let dat = self.resource.file(repository, category, location)?;

		// TODO: Cache files? Tempted to say it's the IW struct's responsibility. Is it even possible here with streams?
		File::new(dat)
	}

	fn path_metadata(&self, path: &str) -> Result<(u8, u8)> {
		// NOTE: This could be technically-faster by doing that cursed logic the
		// game does, checking the first 3 characters for category and such - but I
		// think this is cleaner; especially to read.

		let mut split = path.split('/');
		let (Some(category_segment), Some(repository_segment)) = (split.next(), split.next())
		else {
			return Err(Error::PathInvalid(
				"SqPack paths must contain at least two segments".into(),
			));
		};

		let repository = REPOSITORIES
			.iter()
			.position(|&repository| repository == repository_segment)
			.unwrap_or(0);

		let category = CATEGORIES
			.iter()
			.position(|&category| category == Some(category_segment))
			.ok_or_else(|| {
				Error::PathInvalid(format!("unknown SqPack category \"{category_segment}\""))
			})?;

		Ok((repository.try_into().unwrap(), category.try_into().unwrap()))
	}
}
