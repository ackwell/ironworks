use std::collections::HashMap;

use ironworks_schema::Schema;
use serde::Deserialize;

use super::{error::Error, saint_coinach, Specifier};

pub trait Source: Send + Sync {
	fn version(&self, version: Option<&str>) -> Result<Box<dyn Schema + '_>, Error>;
}

#[derive(Debug, Deserialize)]
pub struct Config {
	default: Specifier,

	saint_coinach: saint_coinach::Config,
}

// TODO: need a way to handle updating the repo
pub struct Provider {
	default: Specifier,
	sources: HashMap<&'static str, Box<dyn Source>>,
}

impl Provider {
	pub fn new(config: Config) -> Result<Self, Error> {
		// TODO: at the moment this will hard fail if any source fails - should i make sources soft fail?
		Ok(Self {
			default: config.default,
			sources: HashMap::from([(
				"saint-coinach",
				boxed(saint_coinach::SaintCoinach::new(config.saint_coinach)?),
			)]),
		})
	}

	pub fn schema(&self, specifier: Option<&Specifier>) -> Result<Box<dyn Schema + '_>, Error> {
		let specifier = specifier.unwrap_or(&self.default);

		let source = self
			.sources
			.get(specifier.source.as_str())
			.ok_or_else(|| Error::UnknownSource(specifier.source.clone()))?;
		source.version(specifier.version.as_deref())
	}
}

fn boxed(x: impl Source + 'static) -> Box<dyn Source> {
	Box::new(x)
}
