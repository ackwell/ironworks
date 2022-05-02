use derivative::Derivative;

use crate::{
	error::{Error, ErrorValue, Result},
	file::File,
};

// TODO: This shares name with sqpack::resource. conceptually it's similar but also kinda not. thoughts?
/// Resource layer that can provide data to an ironworks instance.
pub trait Resource: 'static {
	/// Get the version string for the file at `path`. A return value of
	/// `Err(Error::NotFound(ErrorValue::Path(_)))` will result in lookups
	/// continuing to the next resource.
	fn version(&self, path: &str) -> Result<String>;

	/// Get the raw byte data for the file at `path`. A return value of
	/// `Err(Error::NotFound(ErrorValue::Path(_)))` will result in lookups
	/// continuing to the next resource.
	fn file(&self, path: &str) -> Result<Vec<u8>>;
}

/// Core ironworks struct. Add one or more resources to query files.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Ironworks {
	#[derivative(Debug = "ignore")]
	resource: Vec<Box<dyn Resource>>,
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
			resource: Default::default(),
		}
	}

	/// Add a resource to search for files. Resources are searched last-first; the
	/// last resource added to ironworks that provides a requested path will be
	/// the resource that is utilised.
	pub fn resource(mut self, resource: impl Resource) -> Self {
		self.resource.push(Box::new(resource));
		self
	}

	/// Get the version string for the file at `path`.
	pub fn version(&self, path: &str) -> Result<String> {
		self.find_first(path, |provider, path| provider.version(path))
	}

	/// Read the file at `path`, using file type F to parse. To retrieve the file
	/// as raw bytes, pass `Vec<u8>` to F.
	pub fn file<F: File>(&self, path: &str) -> Result<F> {
		let data = self.find_first(path, |provider, path| provider.file(path))?;
		F::read(data)
	}

	fn find_first<F, O>(&self, path: &str, f: F) -> Result<O>
	where
		F: Fn(&Box<dyn Resource>, &str) -> Result<O>,
	{
		self.resource
			.iter()
			.rev()
			.map(|provider| f(provider, path))
			.find(|result| !matches!(result, Err(Error::NotFound(ErrorValue::Path(_)))))
			.unwrap_or_else(|| Err(Error::NotFound(ErrorValue::Path(path.into()))))
	}
}
