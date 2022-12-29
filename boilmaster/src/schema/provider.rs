use std::collections::HashMap;

use anyhow::{Context, Result};
use ironworks_schema::Schema;
use serde::Deserialize;

use super::{saint_coinach, Specifier};

pub trait Source: Send + Sync {
	fn version(&self, version: Option<&str>) -> Result<Box<dyn Schema + '_>>;
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
	pub fn new(config: Config) -> Result<Self> {
		// TODO: at the moment this will hard fail if any source fails - should i make sources soft fail?
		Ok(Self {
			default: config.default,
			sources: HashMap::from([(
				"saint-coinach",
				boxed(saint_coinach::SaintCoinach::new(config.saint_coinach)?),
			)]),
		})
	}

	pub fn schema(&self, specifier: Option<&Specifier>) -> Result<Box<dyn Schema + '_>> {
		let specifier = specifier.unwrap_or(&self.default);

		let source = self
			.sources
			.get(specifier.source.as_str())
			// TODO: this should be exposed to consumers.
			.context("unknown schema source")?;
		source.version(specifier.version.as_deref())
	}
}

fn boxed(x: impl Source + 'static) -> Box<dyn Source> {
	Box::new(x)
}
