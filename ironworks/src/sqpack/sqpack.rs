use std::{convert::Infallible, fmt::Debug};

use crate::{
	error::{Error, ErrorValue, Result},
	ironworks::{EntryKind, ListEntry, Resource},
	sqpack,
	utility::{HashMapCache, HashMapCacheExt, OptionCache, OptionCacheExt},
};

use super::{file, index::Index, Hierarchy};

/// Representation of a group of SqPack package files forming a single data set.
#[derive(Debug)]
pub struct SqPack<R, K> {
	resource: R,

	indexes: HashMapCache<K, Index>,
	hierarchy: OptionCache<Vec<Hierarchy<K>>>,
}

impl<R: sqpack::Resource> SqPack<R, R::PathMetadata> {
	/// Build a representation of SqPack packages. The provided resource will be
	/// queried for lookups as required to fulfil SqPack requests.
	pub fn new(resource: R) -> Self {
		Self {
			resource,

			indexes: Default::default(),
			hierarchy: Default::default(),
		}
	}

	/// Get the version string for the file at `path`.
	pub fn version(&self, path: &str) -> Result<String> {
		let path_metadata = self.path_metadata(&path.to_lowercase())?;
		self.resource.version(&path_metadata)
	}

	/// Read the file at `path` from SqPack.
	pub fn file(&self, path: &str) -> Result<Vec<u8>> {
		// SqPack paths are always lower case.
		let path = path.to_lowercase();

		// Look up the location of the requested path.
		let path_metadata = self.path_metadata(&path)?;

		let location = self
			.indexes
			.try_get_or_insert(path_metadata.clone(), || {
				Index::new(&path_metadata, &self.resource)
			})?
			.find(&path)?;

		// Build a File representation.
		let dat = self
			.resource
			.dat(&path_metadata, location.chunk, location.data_file)?;

		// TODO: Cache files? Tempted to say it's the IW struct's responsibility.
		file::read(dat, location.offset)
	}

	/// List the contents of the specified `path`.
	pub fn list(&self, path: &str) -> Vec<ListEntry> {
		let hierarchy = self
			.hierarchy
			.try_get_or_insert(|| -> Result<_, Infallible> { Ok(self.resource.hierarchy()) })
			.unwrap();

		// TODO: this isn't... nice. what's the best way to represent this?
		// If there's a requested path, drill into the hierarchy to the appropriate location.
		let mut current = hierarchy.as_ref().iter().collect::<Vec<_>>();
		if !path.is_empty() {
			for segment in path.split('/') {
				current = current
					.into_iter()
					.filter_map(|node| match node {
						Hierarchy::Group(name, children) if name == segment => Some(children),
						_ => None,
					})
					.flatten()
					.collect::<Vec<_>>();

				if current.is_empty() {
					break;
				}
			}
		}

		current
			.into_iter()
			.map(|node| match node {
				Hierarchy::Item(_) => ListEntry {
					kind: EntryKind::File,
					// TODO: list out the hashes in the path meta at this point.
					path: "#TODO".into(),
				},
				Hierarchy::Group(name, _) => ListEntry {
					kind: EntryKind::Directory,
					path: name.into(),
				},
			})
			.collect()
	}

	fn path_metadata(&self, path: &str) -> Result<R::PathMetadata> {
		self.resource
			.path_metadata(path)
			.ok_or_else(|| Error::NotFound(ErrorValue::Path(path.to_string())))
	}
}

// TODO: work out the resource story for this because it's gonna get cluttery if im not careful
impl<R> Resource for SqPack<R, R::PathMetadata>
where
	R: sqpack::Resource + Send + Sync + 'static,
	R::PathMetadata: Send + Sync,
{
	fn version(&self, path: &str) -> Result<String> {
		self.version(path)
	}

	fn file(&self, path: &str) -> Result<Vec<u8>> {
		self.file(path)
	}

	fn list(&self, path: &str) -> Vec<ListEntry> {
		self.list(path)
	}
}
