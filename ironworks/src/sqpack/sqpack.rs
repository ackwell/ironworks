use std::fmt::Debug;

use crate::{
	error::{Error, ErrorValue, Result},
	utility::{HashMapCache, HashMapCacheExt},
};

use super::{file::File, index::Index, resource::Resource};

/// Representation of a group of SqPack package files forming a single data set.
#[derive(Debug)]
pub struct SqPack<R> {
	resource: R,

	indexes: HashMapCache<(u8, u8), Index>,
}

impl<R: Resource> SqPack<R> {
	/// Build a representation of SqPack packages. The provided resource will be
	/// queried for lookups as required to fulfil SqPack requests.
	pub fn new(resource: R) -> Self {
		Self {
			resource,

			indexes: Default::default(),
		}
	}

	/// Get the version string for the file at `path`.
	pub fn version(&self, path: &str) -> Result<String> {
		let (repository, _) = self.path_metadata(&path.to_lowercase())?;
		self.resource.version(repository)
	}

	/// Read the file at `path` from SqPack.
	pub fn file(&self, path: &str) -> Result<File<R::Dat>> {
		// SqPack paths are always lower case.
		let path = path.to_lowercase();

		// Look up the location of the requested path.
		let (repository, category) = self.path_metadata(&path)?;

		let location = self
			.indexes
			.try_get_or_insert((repository, category), || {
				Index::new(repository, category, &self.resource)
			})?
			.find(&path)?;

		// Build a File representation.
		let dat = self
			.resource
			.dat(repository, category, location.chunk, location.data_file)?;

		// TODO: Cache files? Will need to think about ownership and shared cursor
		// positions if we do that. Maybe an internal structure for dealing with
		// cached binary data, and then a cloneable "position" structure that isn't cached?
		File::new(dat, location.offset)
	}

	fn path_metadata(&self, path: &str) -> Result<(u8, u8)> {
		self.resource
			.path_metadata(path)
			.ok_or_else(|| Error::NotFound(ErrorValue::SqpackPath(path.to_string())))
	}
}
