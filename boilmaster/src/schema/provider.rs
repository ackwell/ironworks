use std::collections::HashMap;

use anyhow::{Context, Result};
use ironworks_schema::Schema;
use serde::Deserialize;

use super::saint_coinach;

pub trait Source: Send + Sync {
	fn version(&self, version: Option<&str>) -> Result<Box<dyn Schema + '_>>;
}

#[derive(Debug, Deserialize)]
pub struct Config {
	saint_coinach: saint_coinach::Config,
}

// TODO: need a way to handle updating the repo
pub struct Provider {
	sources: HashMap<&'static str, Box<dyn Source>>,
}

impl Provider {
	pub fn new(config: Config) -> Result<Self> {
		// TODO: at the moment this will hard fail if any source fails - should i make sources soft fail?
		Ok(Self {
			sources: HashMap::from([(
				"saint-coinach",
				boxed(saint_coinach::SaintCoinach::new(config.saint_coinach)?),
			)]),
		})
	}

	pub fn schema(&self, source_name: &str, version: Option<&str>) -> Result<Box<dyn Schema + '_>> {
		let source = self
			.sources
			.get(source_name)
			.context("unknown schema source")?;
		source.version(version)
	}
}

fn boxed(x: impl Source + 'static) -> Box<dyn Source> {
	Box::new(x)
}
