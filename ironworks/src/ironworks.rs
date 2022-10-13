use std::io::{Read, Seek};

use derivative::Derivative;

use crate::{
	error::{Error, ErrorValue, Result},
	file::File,
};

/// Representation of a file stream read from a resource.
pub trait FileStream: Read + Seek + Send + Sync + 'static {}
impl<T> FileStream for T where T: Read + Seek + Send + Sync + 'static {}

// TODO: This shares name with sqpack::resource. conceptually it's similar but also kinda not. thoughts?
/// Resource layer that can provide data to an ironworks instance.
pub trait Resource: Send + Sync + 'static {
	/// Get the version string for the file at `path`. A return value of
	/// `Err(Error::NotFound(ErrorValue::Path(_)))` will result in lookups
	/// continuing to the next resource.
	fn version(&self, path: &str) -> Result<String>;

	/// Get a data stream for the file at `path`. A return value of
	/// `Err(Error::NotFound(ErrorValue::Path(_)))` will result in lookups
	/// continuing to the next resource.
	fn file(&self, path: &str) -> Result<Box<dyn FileStream>>;
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
		let stream = self.find_first(path, |resource| resource.file(path))?;
		F::read(stream)
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
