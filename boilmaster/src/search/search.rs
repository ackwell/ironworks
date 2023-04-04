use std::{
	collections::HashMap,
	path::PathBuf,
	sync::{Arc, RwLock},
};

use anyhow::anyhow;
use figment::value::magic::RelativePathBuf;
use futures::Future;
use serde::Deserialize;

use crate::{data::Data, version::VersionKey};

use super::{
	error::SearchError,
	index::{IngestConfig, Ingester},
	version::Version,
};

#[derive(Debug, Deserialize)]
pub struct Config {
	ingest: IngestConfig,

	index: IndexConfig,
}

#[derive(Debug, Deserialize)]
struct IndexConfig {
	directory: RelativePathBuf,
}

pub struct Search {
	ingester: Ingester,

	index_directory: PathBuf,

	versions: RwLock<HashMap<VersionKey, Arc<Version>>>,
}

impl Search {
	#[allow(clippy::new_without_default)]
	pub fn new(config: Config) -> Self {
		Self {
			ingester: Ingester::new(config.ingest),
			index_directory: config.index.directory.relative(),
			versions: Default::default(),
		}
	}

	// TODO: name. ensure_ingested()? is it worth naming it to flag that ingestion may not occur?
	pub async fn ingest(
		self: Arc<Self>,
		shutdown: impl Future<Output = ()>,
		data: &Data,
		version: &VersionKey,
	) -> Result<(), SearchError> {
		let data_version = data
			.version(version)
			.ok_or_else(|| anyhow!("{version} could not be resolved to a data version"))?;
		let search_version = Arc::new(Version::new(self.index_directory.join(version)));

		tokio::select! {
			_ = shutdown => {},
			result = search_version.clone().ingest(&self.ingester, &data_version) => { result? },
		}

		self.versions
			.write()
			.expect("poisoned")
			.insert(version.clone(), search_version);

		Ok(())
	}

	pub fn version(&self, version: &VersionKey) -> Option<Arc<Version>> {
		self.versions
			.read()
			.expect("poisoned")
			.get(version)
			.cloned()
	}
}
