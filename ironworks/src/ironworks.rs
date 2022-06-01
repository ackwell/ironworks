use derivative::Derivative;
use itertools::Itertools;

use crate::{
	error::{Error, ErrorValue, Result},
	file::File,
};

/// An entry in a file list.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ListEntry {
	/// Kind of this entry.
	pub kind: EntryKind,
	/// Relative path of this entry within the parent.
	pub path: String,
}

/// Kind of entry in a list.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EntryKind {
	File,
	Directory,
}

// TODO: This shares name with sqpack::resource. conceptually it's similar but also kinda not. thoughts?
/// Resource layer that can provide data to an ironworks instance.
pub trait Resource: Send + Sync + 'static {
	/// Get the version string for the file at `path`. A return value of
	/// `Err(Error::NotFound(ErrorValue::Path(_)))` will result in lookups
	/// continuing to the next resource.
	fn version(&self, path: &str) -> Result<String>;

	/// Get the raw byte data for the file at `path`. A return value of
	/// `Err(Error::NotFound(ErrorValue::Path(_)))` will result in lookups
	/// continuing to the next resource.
	fn file(&self, path: &str) -> Result<Vec<u8>>;

	/// List the contents of the specified `path`.
	fn list(&self, path: &str) -> Box<dyn Iterator<Item = ListEntry>>;
}

/// Core ironworks struct. Add one or more resources to query files.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Ironworks {
	#[derivative(Debug = "ignore")]
	resources: Vec<Box<dyn Resource>>,
	// todo: does this own the file cache, then?
}

impl Default for Ironworks {
	fn default() -> Self {
		Self::new()
	}
}

impl Ironworks {
	/// Build a new instance of ironworks.
	pub fn new() -> Self {
		Self {
			resources: Default::default(),
		}
	}

	/// Add a resource to search for files. Resources are searched last-first; the
	/// last resource added to ironworks that provides a requested path will be
	/// the resource that is utilised.
	pub fn add_resource(&mut self, resource: impl Resource) {
		self.resources.push(Box::new(resource));
	}

	/// Add a resource to search for files. Resources are searched last-first; the
	/// last resource added to ironworks that provides a requested path will be
	/// the resource that is utilised.
	#[must_use]
	pub fn with_resource(mut self, resource: impl Resource) -> Self {
		self.resources.push(Box::new(resource));
		self
	}

	/// Get the version string for the file at `path`.
	pub fn version(&self, path: &str) -> Result<String> {
		self.find_first(path, |resource| resource.version(path))
	}

	/// Read the file at `path`, using file type F to parse. To retrieve the file
	/// as raw bytes, pass `Vec<u8>` to F.
	pub fn file<F: File>(&self, path: &str) -> Result<F> {
		let data = self.find_first(path, |resource| resource.file(path))?;
		F::read(data)
	}

	/// List the content of the specified `path`.
	pub fn list<'a>(&'a self, path: &'a str) -> impl Iterator<Item = ListEntry> + 'a {
		self.resources
			.iter()
			.rev()
			.flat_map(|resource| resource.list(path))
			.unique()
	}

	fn find_first<F, O>(&self, path: &str, f: F) -> Result<O>
	where
		F: Fn(&Box<dyn Resource>) -> Result<O>,
	{
		self.resources
			.iter()
			.rev()
			.map(f)
			.find(|result| !matches!(result, Err(Error::NotFound(ErrorValue::Path(_)))))
			.unwrap_or_else(|| Err(Error::NotFound(ErrorValue::Path(path.into()))))
	}
}
