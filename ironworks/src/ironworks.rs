use derivative::Derivative;

use crate::error::{Error, ErrorValue, Result};

// TODO: This shares name with sqpack::resource. conceptually it's similar but also kinda not. thoughts?
pub trait Resource: 'static {
	fn file(&self, path: &str) -> Result<Vec<u8>>;
}

pub trait File {
	// todo: might need an error type?
	fn read(data: Vec<u8>) -> Result<Self>
	where
		Self: Sized;
}

impl File for Vec<u8> {
	fn read(data: Vec<u8>) -> Result<Self> {
		Ok(data)
	}
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

	// todo version?

	pub fn file<F: File>(&self, path: &str) -> Result<F> {
		let data = self
			.providers
			.iter()
			.rev()
			.map(|provider| provider.file(path))
			.find(|result| !matches!(result, Err(Error::NotFound(ErrorValue::Path(_)))))
			.unwrap_or_else(|| Err(Error::NotFound(ErrorValue::Path(path.into()))))?;

		F::read(data)
	}
}
