use std::collections::HashMap;

use anyhow::Result;
use ironworks_schema::Schema;

use super::saint_coinach::SaintCoinach;

pub trait Source {
	fn version(&self, version: Option<&str>) -> Result<Box<dyn Schema + '_>>;
}

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
}

fn boxed(x: impl Source + 'static) -> Box<dyn Source> {
	Box::new(x)
}
