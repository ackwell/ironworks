use std::{fmt::Debug, rc::Rc};

use crate::error::{Error, Result};

use super::{index::Index, resource::Resource};

/// Representation of a group of SqPack package files forming a single data set.
#[derive(Debug)]
pub struct SqPack<R> {
	resource: Rc<R>,
}

impl<R: Resource + Debug> SqPack<R> {
	/// Build a representation of SqPack packages. The provided resource will be
	/// queried for lookups as required to fulfil SqPack requests.
	pub fn new(resource: R) -> Self {
		Self {
			resource: resource.into(),
		}
	}

	// TODO: name
	/// Read the file at `path` from SqPack.
	pub fn read(&self, path: &str) -> Result<()> {
		let (repository, category) = self.resource.path_metadata(path).ok_or(Error::NotFound)?;

		// TODO: cache reader
		let reader = Reader::new(repository, category, self.resource.clone())?;
		reader.read(path);

		Ok(())
	}
}

// TODO: this should be in another file
// TODO: name - it's effectively a repo+category abstraction?
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
	}
}
