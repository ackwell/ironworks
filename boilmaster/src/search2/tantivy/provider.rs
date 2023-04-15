use std::{
	collections::{hash_map::Entry, HashMap},
	hash::{Hash, Hasher},
	path::PathBuf,
	sync::Arc,
};

use figment::value::magic::RelativePathBuf;
use ironworks::excel::Sheet;
use seahash::SeaHasher;
use serde::Deserialize;
use tokio::{select, sync::RwLock};
use tokio_util::sync::CancellationToken;

use crate::{
	search2::{error::Result, internal_query::post},
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

pub struct Provider {
	directory: PathBuf,
	memory: usize,

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
		let mut indices = self.indicies.write().await;
		let mut buckets = HashMap::<u64, Vec<(u64, Sheet<String>)>>::new();
		for (version, sheet) in sheets {
			let (index_key, discriminator) = grouping_keys(version, &sheet)?;

			if self.metadata.exists(discriminator)? {
				tracing::debug!(name = %sheet.name(), key = discriminator, "exists, skipping");
				continue;
			}

			if let Entry::Vacant(entry) = indices.entry(index_key) {
				let index = Index::new(
					&self.directory.join(format!("sheets-{index_key:x}")),
					&sheet,
				)?;
				entry.insert(Arc::new(index));
			}

			buckets
				.entry(index_key)
				.or_insert_with(Vec::new)
				.push((discriminator, sheet));
		}
		drop(indices);

		// Run ingestion
		// TODO: consider permitting concurrency here
		tracing::info!("execute");
		let indices = self.indicies.read().await;
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

	pub fn search(&self, query: &post::Node) {
		tracing::info!("would query tantivy {query:#?}");
	}
}

fn grouping_keys(version: VersionKey, sheet: &Sheet<String>) -> Result<(u64, u64)> {
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

	let index_key = hasher.finish();

	let mut hasher = SeaHasher::new();
	version.hash(&mut hasher);
	sheet.name().hash(&mut hasher);
	let discriminator = hasher.finish();

	Ok((index_key, discriminator))
}
