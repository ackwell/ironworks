use std::{
	collections::{hash_map::Entry, HashMap},
	hash::{Hash, Hasher},
	sync::Arc,
};

use figment::value::magic::RelativePathBuf;
use ironworks::excel::Sheet;
use seahash::SeaHasher;
use serde::Deserialize;
use tokio::{select, sync::RwLock};
use tokio_util::sync::CancellationToken;

use crate::{search2::error::Result, version::VersionKey};

use super::index::Index;

#[derive(Debug, Deserialize)]
pub struct Config {
	directory: RelativePathBuf,
	memory: usize,
}

pub struct Provider {
	config: Config,
	indicies: RwLock<HashMap<u64, Arc<Index>>>,
}

impl Provider {
	pub fn new(config: Config) -> Self {
		Self {
			config,
			indicies: Default::default(),
		}
	}

	pub async fn ingest(
		&self,
		cancel: CancellationToken,
		sheets: impl IntoIterator<Item = (VersionKey, Sheet<'static, String>)>,
	) -> Result<()> {
		let memory = self.config.memory;
		let path = self.config.directory.relative();

		// Bucket sheets by their index and ensure that the indices exist.
		// TODO: this seems dumb, but it avoids locking the rwlock for write while ingestion is ongoing. think of a better approach.
		let mut indices = self.indicies.write().await;
		let mut buckets = HashMap::<u64, Vec<(u64, Sheet<String>)>>::new();
		for (version, sheet) in sheets {
			let (index_key, discriminator) = grouping_keys(version, &sheet)?;

			if let Entry::Vacant(entry) = indices.entry(index_key) {
				let index = Index::new(&path.join(format!("{index_key:x}")), &sheet)?;
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
		let indices = self.indicies.read().await;
		for (key, sheets) in buckets {
			let index = indices.get(&key).expect("ensured").clone();
			select! {
			  _ = cancel.cancelled() => { break }
			  result = tokio::task::spawn_blocking(move || index.ingest(memory, &sheets)) => { result?? }
			}
		}

		Ok(())
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
