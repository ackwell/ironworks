use std::{fmt::Debug, rc::Rc};

use crate::error::{Error, Result};

use super::{index::Index, resource::Resource};

/// Representation of a group of SqPack package files forming a single data set.
#[derive(Debug)]
pub struct SqPack<R> {
	resource: Rc<R>,
}

impl<R: Resource> SqPack<R> {
	/// Build a representation of SqPack packages. The provided resource will be
	/// queried for lookups as required to fulfil SqPack requests.
	pub fn new(resource: R) -> Self {
		Self {
			resource: resource.into(),
		}
	}

	// TODO: name
	/// Read the file at `path` from SqPack.
	pub fn read(&self, path: &str) -> Result<File<R::Dat>> {
		let (repository, category) = self.resource.path_metadata(path).ok_or(Error::NotFound)?;

		// TODO: cache reader
		let reader = Reader::new(repository, category, self.resource.clone())?;
		reader.read(path)
	}
}

// TODO: this should be in another file
// TODO: name - it's effectively a repo+category abstraction?
// TODO: If this doesn't grow much more, realistically it can be inlined into the main sqpack struct
#[derive(Debug)]
struct Reader<R> {
	repository: u8,
	category: u8,

	index: Index,
	resource: Rc<R>,
}

impl<R: Resource> Reader<R> {
	fn new(repository: u8, category: u8, resource: Rc<R>) -> Result<Self> {
		// Eagerly build index
		let index = Index::new(repository, category, resource.as_ref())?;

		Ok(Self {
			repository,
			category,
			index,
			resource,
		})
	}

	// TODO: name?
	fn read(&self, path: &str) -> Result<File<R::Dat>> {
		let location = self.index.find(path)?;

		let dat = self.resource.dat(
			self.repository,
			self.category,
			location.chunk,
			location.data_file,
		)?;

		File::new(dat, location.offset)
	}
}
