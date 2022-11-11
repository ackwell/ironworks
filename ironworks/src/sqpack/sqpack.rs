use std::{fmt::Debug, sync::Arc};

use crate::{
	error::{Error, ErrorValue, Result},
	ironworks::FileStream,
	sqpack,
	utility::{HashMapCache, HashMapCacheExt},
	Resource,
};

use super::{file::File, index::Index};

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
		self.resource
			.path_metadata(path)
			.ok_or_else(|| Error::NotFound(ErrorValue::Path(path.to_string())))
	}
}

// TODO: work out the resource story for this because it's gonna get cluttery if im not careful
impl<R> Resource for SqPack<R>
where
	R: sqpack::Resource + Send + Sync + 'static,
{
	fn version(&self, path: &str) -> Result<String> {
		self.version(path)
	}

	fn file(&self, path: &str) -> Result<Box<dyn FileStream>> {
		Ok(Box::new(self.file(path)?))
	}
}
