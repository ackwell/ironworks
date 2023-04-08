use std::{
	collections::{HashMap, HashSet},
	path::PathBuf,
	sync::{Arc, RwLock},
};

use anyhow::anyhow;
use figment::value::magic::RelativePathBuf;
use futures::future::try_join_all;
use serde::Deserialize;
use tokio::select;
use tokio_util::sync::CancellationToken;

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

	pub async fn start(&self, cancel: CancellationToken, data: &Data) -> Result<(), SearchError> {
		// TODO: I suspect this pattern for listening to a channel and holding a cancel token will be pretty reusable.
		let mut receiver = data.subscribe();
		self.ingest_new(cancel.child_token(), receiver.borrow().clone(), data)
			.await?;

		loop {
			select! {
				Ok(_) = receiver.changed() => {
					self.ingest_new(cancel.child_token(), receiver.borrow().clone(), data).await?
				}
				_ = cancel.cancelled() => break,
			}
		}

		Ok(())
	}

	async fn ingest_new(
		&self,
		cancel: CancellationToken,
		versions: Vec<VersionKey>,
		data: &Data,
	) -> Result<(), SearchError> {
		let known_keys = self
			.versions
			.read()
			.expect("poisoned")
			.keys()
			.cloned()
			.collect::<HashSet<_>>();

		let ingestions = versions
			.into_iter()
			.filter(|key| !known_keys.contains(key))
			.map(|key| self.ingest(cancel.child_token(), data, key));

		// TODO: consider if .ingest should own writing to self.versions or if it should return the Version and let it be added here. main downside of doing so is that in a theoretical high-count version ingestion, versions completed early would not be available until all versions are complete.
		try_join_all(ingestions).await?;

		Ok(())
	}

	// TODO: name. ensure_ingested()? is it worth naming it to flag that ingestion may not occur?
	#[tracing::instrument(skip_all, fields(%version))]
	async fn ingest(
		&self,
		cancel: CancellationToken,
		data: &Data,
		version: VersionKey,
	) -> Result<(), SearchError> {
		tracing::info!("ingestion starting");

		let data_version = data
			.version(&version)
			.ok_or_else(|| anyhow!("{version} could not be resolved to a data version"))?;
		let search_version = Arc::new(Version::new(self.index_directory.join(version.to_string())));

		tokio::select! {
			_ = cancel.cancelled() => {},
			result = search_version.clone().ingest(&self.ingester, &data_version) => { result? },
		}

		self.versions
			.write()
			.expect("poisoned")
			.insert(version.clone(), search_version);

		tracing::info!("ingestion complete");

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
