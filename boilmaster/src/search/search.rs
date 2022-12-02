use std::sync::{Arc, Mutex};

use anyhow::Result;
use figment::value::magic::RelativePathBuf;
use futures::Future;
use serde::Deserialize;

use crate::data::Data;

use super::version::Version;

#[derive(Debug, Deserialize)]
pub struct Config {
	// TODO: the underscore in this could make env var based config ugly. is there multiple config items for indexes? if there is, might be worth splitting index/ingestion config and using index.directory
	index_directory: RelativePathBuf,
}

pub struct Search {
	config: Config,

	// TODO: this should be a map of version keys to search::Version instances
	temp_version: Mutex<Option<Arc<Version>>>,
}

impl Search {
	#[allow(clippy::new_without_default)]
	pub fn new(config: Config) -> Self {
		Self {
			config,
			temp_version: Default::default(),
		}
	}

	// TODO: name. ensure_ingested()? is it worth naming it to flag that ingestion may not occur?
	pub async fn ingest(
		self: Arc<Self>,
		shutdown: impl Future<Output = ()>,
		data: &Data,
		version: Option<&str>,
	) -> Result<()> {
		let path = self.config.index_directory.relative();

		let data_version = data.version(version);
		let search_version = Arc::new(Version::new(path.join(version.unwrap_or("__NONE"))));

		tokio::select! {
			_ = shutdown => {},
			result = search_version.clone().ingest(data_version) => { result? },
		}

		let mut guard = self.temp_version.lock().unwrap();
		*guard = Some(search_version);

		Ok(())
	}

	pub fn version(&self, version: Option<&str>) -> Option<Arc<Version>> {
		// TODO: actual version handling
		if version.is_some() {
			todo!("search version handling");
		}

		self.temp_version.lock().unwrap().clone()
	}
}
