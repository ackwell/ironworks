use std::{
	cell::RefCell,
	collections::{hash_map::Entry, HashMap},
	fmt::Debug,
	rc::Rc,
};

use crate::error::{Error, ErrorValue, Result};

use super::{file::File, index::Index, resource::Resource};

/// Representation of a group of SqPack package files forming a single data set.
#[derive(Debug)]
pub struct SqPack<R> {
	resource: R,

	indexes: RefCell<HashMap<(u8, u8), Rc<Index>>>,
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

	// TODO: name
	/// Read the file at `path` from SqPack.
	pub fn read(&self, path: &str) -> Result<File<R::Dat>> {
		// Look up the location of the requested path.
		let (repository, category) = self
			.resource
			.path_metadata(path)
			.ok_or_else(|| Error::NotFound(ErrorValue::SqpackPath(path.into())))?;

		let location = self.index(repository, category)?.find(path)?;

		// Build a File representation.
		let dat = self
			.resource
			.dat(repository, category, location.chunk, location.data_file)?;

		// TODO: Cache files? Will need to think about ownership and shared cursor
		// positions if we do that. Maybe an internal structure for dealing with
		// cached binary data, and then a cloneable "position" structure that isn't cached?
		File::new(dat, location.offset)
	}

	fn index(&self, repository: u8, category: u8) -> Result<Rc<Index>> {
		// TODO: maybe try_borrow_mut?
		let mut indexes = self.indexes.borrow_mut();

		let index = match indexes.entry((repository, category)) {
			Entry::Occupied(value) => value.get().clone(),
			Entry::Vacant(value) => {
				let index = Index::new(repository, category, &self.resource)?;
				value.insert(index.into()).clone()
			}
		};

		Ok(index)
	}
}
