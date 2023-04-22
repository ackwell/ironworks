use std::sync::Arc;

use anyhow::Context;

use crate::{data, version::VersionKey};

use super::{error::Result, format::Format};

pub struct Service {
	data: Arc<data::Data>,
}

impl Service {
	pub fn new(data: Arc<data::Data>) -> Self {
		Self { data }
	}

	pub fn convert(&self, version: VersionKey, path: &str, format: Format) -> Result<Vec<u8>> {
		// TODO: presumably this is where caching would be resolved

		let data_version = self
			.data
			.version(version)
			.with_context(|| format!("data for {version} not ready"))?;

		let converter = format.converter();
		converter.convert(&data_version, path, format)
	}
}
