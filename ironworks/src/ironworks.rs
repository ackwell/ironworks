use derivative::Derivative;

use crate::{
	error::{Error, ErrorValue, Result},
	file::File,
};

// TODO: This shares name with sqpack::resource. conceptually it's similar but also kinda not. thoughts?
pub trait Resource: 'static {
	fn version(&self, path: &str) -> Result<String>;
	fn file(&self, path: &str) -> Result<Vec<u8>>;
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Ironworks {
	#[derivative(Debug = "ignore")]
	providers: Vec<Box<dyn Resource>>,
	// todo: does this own the file cache, then?
}

impl Default for Ironworks {
	fn default() -> Self {
		Self::new()
	}
}

impl Ironworks {
	pub fn new() -> Self {
		Self {
			providers: Default::default(),
		}
	}

	// todo note about .rev()
	pub fn resource(mut self, provider: impl Resource) -> Self {
		self.providers.push(Box::new(provider));
		self
	}

	pub fn version(&self, path: &str) -> Result<String> {
		self.find_first(path, |provider, path| provider.version(path))
	}

	pub fn file<F: File>(&self, path: &str) -> Result<F> {
		let data = self.find_first(path, |provider, path| provider.file(path))?;
		F::read(data)
	}

	fn find_first<F, O>(&self, path: &str, f: F) -> Result<O>
	where
		F: Fn(&Box<dyn Resource>, &str) -> Result<O>,
	{
		self.providers
			.iter()
			.rev()
			.map(|provider| f(provider, path))
			.find(|result| !matches!(result, Err(Error::NotFound(ErrorValue::Path(_)))))
			.unwrap_or_else(|| Err(Error::NotFound(ErrorValue::Path(path.into()))))
	}
}
