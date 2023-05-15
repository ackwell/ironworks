use std::{
	cmp::Ordering,
	collections::{hash_map::Entry, HashMap},
	fmt,
	hash::{Hash, Hasher},
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
use uuid::Uuid;

use crate::{
	search::{
		error::Result,
		internal_query::post,
		search::{Executor, SearchResult},
		Error,
	},
	version::VersionKey,
};

use super::{
	cursor::{self, Cursor, IndexCursor, StableHashMap},
	index::Index,
	metadata::{Metadata, MetadataStore},
};

pub enum SearchRequest {
	Query {
		version: VersionKey,
		queries: Vec<(String, post::Node)>,
	},
	Cursor(Uuid),
}

#[derive(Debug, Deserialize)]
pub struct Config {
	directory: RelativePathBuf,
	memory: usize,

	cursor: cursor::Config,
}

pub struct Provider {
	directory: PathBuf,
	memory: usize,

	sheet_index_map: RwLock<HashMap<SheetKey, IndexKey>>,
	sheet_name_map: RwLock<HashMap<SheetKey, (VersionKey, String)>>,

	indicies: RwLock<HashMap<IndexKey, Arc<Index>>>,
	metadata: Arc<MetadataStore>,
	cursors: cursor::Cache,
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
			cursors: cursor::Cache::new(config.cursor),
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
	) -> Result<HashMap<IndexKey, Vec<(SheetKey, Sheet<'static, String>)>>> {
		// Bucket sheets by their index and ensure that the indices exist.
		// TODO: this seems dumb, but it avoids locking the rwlock for write while ingestion is ongoing. think of a better approach.
		let mut sheet_index_map = self.sheet_index_map.write().expect("poisoned");
		let mut sheet_name_map = self.sheet_name_map.write().expect("poisoned");
		let mut indices = self.indicies.write().expect("poisoned");
		let mut buckets = HashMap::<IndexKey, Vec<(SheetKey, Sheet<String>)>>::new();
		let mut skipped = 0;
		for (version, sheet) in sheets {
			let sheet_name = sheet.name();
			let sheet_key = sheet_key(version, &sheet_name);
			let index_key = index_key(&sheet)?;

			// Ensure that the index for this sheet exists & is known.
			if let Entry::Vacant(entry) = indices.entry(index_key) {
				let index =
					Index::new(&self.directory.join(format!("sheets-{index_key}")), &sheet)?;
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
	) -> Result<(Vec<SearchResult>, Option<Uuid>)> {
		let cursor = match request {
			SearchRequest::Query { version, queries } => {
				Arc::new(self.bucket_queries(version, queries)?)
			}
			SearchRequest::Cursor(uuid) => self
				.cursors
				.get(uuid)
				.ok_or_else(|| Error::UnknownCursor(uuid))?,
		};

		let mut results = self.execute_search(&cursor, limit, executor)?;

		// If a limit is set and there's more results, trim down and set up a cursor.
		let mut cursor_key = None;
		if let Some(limit) = limit {
			cursor_key = self.paginate_results(&cursor, limit, &mut results);
		}

		Ok((
			results.into_iter().map(|(_, result)| result).collect(),
			cursor_key,
		))
	}

	fn bucket_queries(
		&self,
		version: VersionKey,
		queries: Vec<(String, post::Node)>,
	) -> Result<Cursor> {
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
				.or_insert_with(IndexCursor::default)
				.queries
				.push((sheet_key, query));
		}

		Ok(Cursor {
			version,
			indices: buckets,
		})
	}

	fn execute_search(
		&self,
		cursor: &Cursor,
		limit: Option<u32>,
		executor: &Executor<'_>,
	) -> Result<Vec<(IndexKey, SearchResult)>> {
		let sheet_name_map = self.sheet_name_map.read().expect("poisoned");

		// NOTE: This +1 is intentional - we request one more than we'll actually
		// return to make it trivial to distinguish when more results exist, even
		// when one index is suppling all data.
		let result_limit = limit.map(|value| value + 1);

		// Execute searches.
		// TODO: parellise?
		let indices = self.indicies.read().expect("poisoned");

		let mut results = cursor
			.indices
			.iter()
			// Fetch the index and perform the search.
			.map(|(index_key, index_cursor)| {
				let index = indices
					.get(index_key)
					.with_context(|| format!("no prepared index for {index_key}"))?;

				let results = index
					.search(cursor.version, index_cursor, result_limit, executor)?
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
						*index_key,
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

	fn paginate_results(
		&self,
		cursor: &Cursor,
		limit: u32,
		results: &mut Vec<(IndexKey, SearchResult)>,
	) -> Option<Uuid> {
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

		// Build the new cursor.
		// TODO: this is pretty clunky, consider a helper on cursor to do this?
		let new_cursor = Cursor {
			version: cursor.version,
			indices: cursor
				.indices
				.iter()
				.map(|(key, index_cursor)| {
					(
						*key,
						IndexCursor {
							queries: index_cursor.queries.clone(),
							offset: index_cursor.offset + offsets.get(key).copied().unwrap_or(0),
						},
					)
				})
				.collect(),
		};

		let key = self.cursors.insert(new_cursor);

		Some(key)
	}
}

// TODO: consider moving these to a seperate module
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SheetKey(u64);

impl fmt::Display for SheetKey {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_fmt(format_args!("{:016x}", self.0))
	}
}

impl From<SheetKey> for u64 {
	fn from(value: SheetKey) -> Self {
		value.0
	}
}

impl From<u64> for SheetKey {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

fn sheet_key(version: VersionKey, sheet_name: &str) -> SheetKey {
	let mut hasher = SeaHasher::new();
	version.hash(&mut hasher);
	sheet_name.hash(&mut hasher);
	SheetKey(hasher.finish())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexKey(u64);

impl fmt::Display for IndexKey {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_fmt(format_args!("{:016x}", self.0))
	}
}

fn index_key(sheet: &Sheet<String>) -> Result<IndexKey> {
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

	Ok(IndexKey(hasher.finish()))
}
