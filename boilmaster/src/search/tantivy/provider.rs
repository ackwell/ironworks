use std::{
	cmp::Ordering,
	collections::{
		hash_map::{DefaultHasher, Entry},
		HashMap,
	},
	hash::{BuildHasherDefault, Hash, Hasher},
	path::PathBuf,
	sync::{Arc, RwLock},
};

use anyhow::Context;
use figment::value::magic::RelativePathBuf;
use ironworks::excel::Sheet;
use itertools::Itertools;
use seahash::SeaHasher;
use serde::Deserialize;
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::{
	search::{
		error::Result,
		internal_query::post,
		search::{Executor, SearchResult},
	},
	version::VersionKey,
};

use super::{
	index::Index,
	metadata::{Metadata, MetadataStore},
};

type StableHashMap<K, V> = HashMap<K, V, BuildHasherDefault<DefaultHasher>>;

pub enum SearchRequest {
	Query {
		version: VersionKey,
		queries: Vec<(String, post::Node)>,
	},
	Cursor,
}

#[derive(Debug, Deserialize)]
pub struct Config {
	directory: RelativePathBuf,
	memory: usize,
}

pub struct Provider {
	directory: PathBuf,
	memory: usize,

	sheet_index_map: RwLock<HashMap<u64, u64>>,
	sheet_name_map: RwLock<HashMap<u64, (VersionKey, String)>>,

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
			sheet_index_map: Default::default(),
			sheet_name_map: Default::default(),
			indicies: Default::default(),
			metadata,
		})
	}

	#[tracing::instrument(skip_all)]
	pub async fn ingest(
		self: Arc<Self>,
		cancel: CancellationToken,
		sheets: Vec<(VersionKey, Sheet<'static, String>)>,
	) -> Result<()> {
		let memory = self.memory;

		tracing::info!("prepare");
		let this = Arc::clone(&self);
		let buckets = tokio::task::spawn_blocking(move || this.prepare_indices(sheets)).await??;

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

	// TODO: this kind of mishmashes preparing indices and bucketing sheets into one process - might be worth splitting that behavior.
	#[allow(clippy::type_complexity)]
	fn prepare_indices(
		&self,
		sheets: impl IntoIterator<Item = (VersionKey, Sheet<'static, String>)>,
	) -> Result<HashMap<u64, Vec<(u64, Sheet<'static, String>)>>> {
		// Bucket sheets by their index and ensure that the indices exist.
		// TODO: this seems dumb, but it avoids locking the rwlock for write while ingestion is ongoing. think of a better approach.
		let mut sheet_index_map = self.sheet_index_map.write().expect("poisoned");
		let mut sheet_name_map = self.sheet_name_map.write().expect("poisoned");
		let mut indices = self.indicies.write().expect("poisoned");
		let mut buckets = HashMap::<u64, Vec<(u64, Sheet<String>)>>::new();
		let mut skipped = 0;
		for (version, sheet) in sheets {
			let sheet_name = sheet.name();
			let sheet_key = sheet_key(version, &sheet_name);
			let index_key = index_key(&sheet)?;

			// Ensure that the index for this sheet exists & is known.
			if let Entry::Vacant(entry) = indices.entry(index_key) {
				let index = Index::new(
					&self.directory.join(format!("sheets-{index_key:016x}")),
					&sheet,
				)?;
				entry.insert(Arc::new(index));
			}

			// Record the mappings for this sheet.
			sheet_index_map.insert(sheet_key, index_key);
			sheet_name_map.insert(sheet_key, (version, sheet_name));

			// If the sheet has already been ingested, skip adding it to the ingestion bucket.
			if self.metadata.exists(sheet_key)? {
				skipped += 1;
				continue;
			}

			buckets
				.entry(index_key)
				.or_insert_with(Vec::new)
				.push((sheet_key, sheet));
		}

		if skipped > 0 {
			tracing::debug!("skipped {skipped} already-ingested sheets");
		}

		Ok(buckets)
	}

	pub fn search(
		&self,
		request: SearchRequest,
		limit: Option<u32>,
		executor: &Executor<'_>,
	) -> Result<Vec<SearchResult>> {
		let (version, buckets) = match request {
			SearchRequest::Query { version, queries } => {
				(version, self.bucket_queries(version, queries)?)
			}
			SearchRequest::Cursor => todo!(),
		};

		let mut results = self.execute_search(version, buckets, limit, executor)?;

		// If a limit is set and there's more results, trim down and set up a cursor.
		if let Some(limit) = limit {
			let _todo_cursor = self.paginate_results(limit, &mut results);
		}

		Ok(results.into_iter().map(|(_, result)| result).collect())
	}

	fn bucket_queries(
		&self,
		version: VersionKey,
		queries: Vec<(String, post::Node)>,
	) -> Result<StableHashMap<u64, Vec<(u64, post::Node)>>> {
		let sheet_index_map = self.sheet_index_map.read().expect("poisoned");

		// Group queries by index, maintaining a reverse mapping for the sheet keys.
		// NOTE: this is overriding the default RandomState intentionally, to ensure that bucket ordering is consistent between queries.
		let mut buckets = StableHashMap::<_, _>::default();
		for (sheet_name, query) in queries {
			let sheet_key = sheet_key(version, &sheet_name);
			let index_key = sheet_index_map
				.get(&sheet_key)
				.with_context(|| format!("no index mapping for {sheet_name} @ {version}"))?;
			buckets
				.entry(*index_key)
				.or_insert_with(Vec::new)
				.push((sheet_key, query));
		}

		Ok(buckets)
	}

	fn execute_search(
		&self,
		version: VersionKey,
		buckets: StableHashMap<u64, Vec<(u64, post::Node)>>,
		limit: Option<u32>,
		executor: &Executor<'_>,
	) -> Result<Vec<(u64, SearchResult)>> {
		let sheet_name_map = self.sheet_name_map.read().expect("poisoned");

		// NOTE: This +1 is intentional - we request one more than we'll actually
		// return to make it trivial to distinguish when more results exist, even
		// when one index is suppling all data.
		let result_limit = limit.map(|value| value + 1);

		// Execute searches.
		// TODO: parellise?
		let indices = self.indicies.read().expect("poisoned");

		let mut results = buckets
			.into_iter()
			// Fetch the index and perform the search.
			.map(|(index_key, queries)| {
				let index = indices
					.get(&index_key)
					.with_context(|| format!("no prepared index for {index_key}"))?;

				let results = index
					.search(version, queries, result_limit, executor)?
					.map(move |result| (index_key, result));

				Ok(results)
			})
			// Flatten results from all the indices.
			.flatten_ok()
			// Replace sheet keys with resolved sheet names.
			.map(|maybe_result| {
				maybe_result.and_then(|(index_key, result)| {
					let (_, name) = sheet_name_map.get(&result.sheet_key).with_context(|| {
						format!(
							"sheet key {} missing from sheet name mapping",
							result.sheet_key
						)
					})?;

					Ok((
						index_key,
						SearchResult {
							sheet: name.clone(),
							score: result.score,
							row_id: result.row_id,
							subrow_id: result.subrow_id,
						},
					))
				})
			})
			.collect::<Result<Vec<_>>>()?;

		// The results produced by the above are effectively grouped by index - sort them by their scores.
		results.sort_by(|a, b| b.1.score.partial_cmp(&a.1.score).unwrap_or(Ordering::Equal));

		Ok(results)
	}

	fn paginate_results(&self, limit: u32, results: &mut Vec<(u64, SearchResult)>) -> Option<()> {
		// If the results fit within the limit, there's nothing to do.
		let limit_usize = usize::try_from(limit).unwrap();
		if results.len() <= limit_usize {
			return None;
		}

		// Truncate the results to the limit.
		results.truncate(limit_usize);
		results.shrink_to_fit();

		// Count the results per index.
		let offsets = results.iter().counts_by(|item| item.0);

		tracing::info!("recorded counts {offsets:#?}");

		Some(())
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
