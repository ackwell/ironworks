use std::collections::HashMap;

use anyhow::{Context, Result};
use ironworks_schema::Schema;

use super::saint_coinach::SaintCoinach;

pub trait Source: Send + Sync {
	fn version(&self, version: Option<&str>) -> Result<Box<dyn Schema + '_>>;
}

// TODO: need a way to handle updating the repo
pub struct Provider {
	sources: HashMap<&'static str, Box<dyn Source>>,
}

impl Provider {
	pub fn new() -> Result<Self> {
		// TODO: at the moment this will hard fail if any source fails - should i make sources soft fail?
		Ok(Self {
			sources: HashMap::from([("saint-coinach", boxed(SaintCoinach::new()?))]),
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
