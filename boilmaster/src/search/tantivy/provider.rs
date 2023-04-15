use std::{
	collections::{hash_map::Entry, HashMap},
	hash::{Hash, Hasher},
	path::PathBuf,
	sync::{Arc, RwLock},
};

use anyhow::Context;
use figment::value::magic::RelativePathBuf;
use ironworks::excel::Sheet;
use seahash::SeaHasher;
use serde::Deserialize;
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::{
	search::{error::Result, internal_query::post, search::Executor},
	version::VersionKey,
};

use super::{
	index::Index,
	metadata::{Metadata, MetadataStore},
};

#[derive(Debug, Deserialize)]
pub struct Config {
	directory: RelativePathBuf,
	memory: usize,
}

// TODO: This, and the overall interface of the Provider, should be shared in the main search namespace.
#[derive(Debug)]
pub struct IndexResult {
	pub score: f32,
	pub row_id: u32,
	pub subrow_id: u16,
}

pub struct Provider {
	directory: PathBuf,
	memory: usize,

	sheet_map: RwLock<HashMap<u64, u64>>,
	indicies: RwLock<HashMap<u64, Arc<Index>>>,
	metadata: Arc<MetadataStore>,
}

impl Provider {
	pub fn new(config: Config) -> Result<Self> {
		let directory = config.directory.relative();
		let metadata = Arc::new(MetadataStore::new(&directory.join("metadata"))?);
		Ok(Self {
			directory,
			memory: config.memory,
			sheet_map: Default::default(),
			indicies: Default::default(),
			metadata,
		})
	}

	#[tracing::instrument(skip_all)]
	pub async fn ingest(
		&self,
		cancel: CancellationToken,
		sheets: impl IntoIterator<Item = (VersionKey, Sheet<'static, String>)>,
	) -> Result<()> {
		let memory = self.memory;

		// Bucket sheets by their index and ensure that the indices exist.
		// TODO: this seems dumb, but it avoids locking the rwlock for write while ingestion is ongoing. think of a better approach.
		tracing::info!("prepare");
		let mut sheet_map = self.sheet_map.write().expect("poisoned");
		let mut indices = self.indicies.write().expect("poisoned");
		let mut buckets = HashMap::<u64, Vec<(u64, Sheet<String>)>>::new();
		for (version, sheet) in sheets {
			let sheet_key = sheet_key(version, &sheet.name());
			let index_key = index_key(&sheet)?;

			// Ensure that the index for this sheet exists & is known.
			if let Entry::Vacant(entry) = indices.entry(index_key) {
				let index = Index::new(
					&self.directory.join(format!("sheets-{index_key:x}")),
					&sheet,
				)?;
				entry.insert(Arc::new(index));
			}

			// Record the index mapping for this sheet.
			sheet_map.insert(sheet_key, index_key);

			// If the sheet has already been ingested, skip adding it to the ingestion bucket.
			if self.metadata.exists(sheet_key)? {
				tracing::debug!(name = %sheet.name(), key = sheet_key, "exists, skipping");
				continue;
			}

			buckets
				.entry(index_key)
				.or_insert_with(Vec::new)
				.push((sheet_key, sheet));
		}
		drop(sheet_map);
		drop(indices);

		// Run ingestion
		// TODO: consider permitting concurrency here
		tracing::info!("execute");
		let indices = self.indicies.read().expect("poisoned");
		for (key, sheets) in buckets {
			let index = indices.get(&key).expect("ensured").clone();
			let metadata = self.metadata.clone();
			select! {
			  _ = cancel.cancelled() => { break }
			  result = tokio::task::spawn_blocking(move || -> Result<_> {
					index.ingest(memory, &sheets)?;
					metadata.write(sheets.into_iter().map(|(key, _sheet)| (key, Metadata{})))?;
					Ok(())
				}) => { result?? }
			}
		}

		tracing::info!("complete");
		Ok(())
	}

	pub fn search(
		&self,
		version: VersionKey,
		sheet_name: &str,
		query: &post::Node,
		executor: &Executor<'_>,
	) -> Result<impl Iterator<Item = IndexResult>> {
		let sheet_map = self.sheet_map.read().expect("poisoned");
		let indicies = self.indicies.read().expect("poisoned");

		let sheet_key = sheet_key(version, sheet_name);
		let index = sheet_map
			.get(&sheet_key)
			.and_then(|key| indicies.get(key))
			.with_context(|| format!("no index found for {sheet_name} @ {version}"))?;

		index.search(version, sheet_key, query, executor)
	}
}

fn sheet_key(version: VersionKey, sheet_name: &str) -> u64 {
	let mut hasher = SeaHasher::new();
	version.hash(&mut hasher);
	sheet_name.hash(&mut hasher);
	hasher.finish()
}

fn index_key(sheet: &Sheet<String>) -> Result<u64> {
	// TODO: consider using fixed seeds?
	let mut hasher = SeaHasher::new();
	sheet.kind()?.hash(&mut hasher);

	let mut languages = sheet.languages()?;
	languages.sort_by_key(|language| u8::from(*language));
	languages.hash(&mut hasher);

	// TODO: this encodes the offsets of the columns as well as their kind (and position due to the vec) - technically the actual offset is irrelevant, so would be good to ignore it, but doing so would require decoupling column names from offsets, which I can't do without changes to a lot of stuff in search query resolution. i'm not convinced that different offset layouts for the same structure are going to be common enough to bother.
	let mut columns = sheet.columns()?;
	columns.sort_by_key(|column| column.offset());
	columns.hash(&mut hasher);

	Ok(hasher.finish())
}
