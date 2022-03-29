use crate::error::{Error, Result};

use super::resource::Resource;

/// Representation of a group of SqPack package files forming a single data set.
#[derive(Debug)]
pub struct SqPack<R> {
	resource: R,
}

impl<R: Resource> SqPack<R> {
	/// Build a representation of SqPack packages. The provided resource will be
	/// queried for lookups as required to fulfil SqPack requests.
	pub fn new(resource: R) -> Self {
		Self { resource }
	}

	// TODO: name
	/// Read the file at `path` from SqPack.
	pub fn read(&self, path: &str) -> Result<()> {
		// TODO: realistically - we're working with the repo and cat as black boxes at this point. do they need to be strings still, or can we use u8 for this side of the eq. and do all the string shit on the resource side?
		let (repository, category) = self.resource.path_metadata(path).ok_or(Error::NotFound)?;

		// TODO: cache reader
		let reader = Reader::new(repository, category);
		println!("{reader:#?}");

		Ok(())
	}
}

// TODO: this should be in another file
// TODO: name - it's effectively a repo+category abstraction?
#[derive(Debug)]
struct Reader {
	repository: u8,
	category: u8,
}

impl Reader {
	fn new(repository: u8, category: u8) -> Self {
		Self {
			repository,
			category,
		}
	}
}
