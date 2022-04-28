use derivative::Derivative;

use crate::error::{Error, ErrorValue, Result};

// call this resource? not sure
pub trait Provider: 'static {
	fn file(&self, path: &str) -> Result<Vec<u8>>;
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Ironworks {
	#[derivative(Debug = "ignore")]
	providers: Vec<Box<dyn Provider>>,
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

	// todo note about rev
	pub fn provider_nameme(mut self, provider: impl Provider) -> Self {
		self.providers.push(Box::new(provider));
		self
	}

	// todo version?

	pub fn file(&self, path: &str) -> Result<Vec<u8>> {
		self.providers
			.iter()
			.rev()
			.map(|provider| provider.file(path))
			// todo check the notfound type?
			.find(|result| !matches!(result, Err(Error::NotFound(_))))
			// TODO: this should be a "path" errorvalue and filepath/sqpackpath can probs be removed
			.unwrap_or_else(|| Err(Error::NotFound(ErrorValue::Other(path.into()))))
	}
}
