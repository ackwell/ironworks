use std::{
	collections::{HashMap, HashSet},
	path::PathBuf,
	sync::{Arc, RwLock},
};

use futures::{stream::FuturesUnordered, StreamExt};
use ironworks::excel;
use ironworks_schema::Schema;

use crate::{
	data::Version as DataVersion,
	utility::{anyhow::Anyhow, warnings::Warnings},
};

use super::{
	error::SearchError,
	index::{Index, IndexResult, Ingester},
	query::{post, pre, Normalizer},
};

#[derive(Debug)]
pub struct SearchResult {
	pub score: f32,
	// TODO: `String` here necessitates a copy of the sheet name for every result, which seems wasteful. consider using a ref or cow for this - can probably tie the lifetime to the version which is possibly okay.
	pub sheet: String,
	pub row_id: u32,
	pub subrow_id: u16,
}

pub struct Version {
	path: PathBuf,

	indices: RwLock<Option<Arc<HashMap<String, Index>>>>,
}

impl Version {
	pub(super) fn new(path: PathBuf) -> Self {
		Self {
			path,
			indices: Default::default(),
		}
	}

	pub(super) async fn ingest(
		self: Arc<Self>,
		ingester: &Ingester,
		data: &DataVersion,
	) -> Result<(), SearchError> {
		let excel = data.excel();

		// NOTE: should probably record which sheets contain strings so we can immediately ignore the rest when there's a query string

		// TODO: on zipatch-backed data instances, accessing .list() could block for quite some time - how do i want to handle that?
		// Create a group of futures; one for each sheet that (should) exist in the index - indexes will be ingested if they do not yet exist.
		let list = excel.list().anyhow()?;
		let mut futures = list
			.iter()
			.map(|sheet_name| {
				let excel = excel.clone();
				let this = self.clone();

				async move {
					let sheet = match excel.sheet(sheet_name.to_string()) {
						Ok(v) => v,
						Err(err) => anyhow::bail!(err),
					};

					let index = Index::ingest(
						ingester,
						this.path.join(sheet_name.replace('/', "!DIR!")),
						sheet,
					)
					.await?;
					Ok((sheet_name, index))
				}
			})
			.collect::<FuturesUnordered<_>>();

		// Pull in all the futures as they complete, adding to the index map.
		let mut indices = HashMap::new();

		while let Some(result) = futures.next().await {
			// TODO: Error handling - a failure here probably implies a failed ingestion, which is Not Good.
			let (sheet_name, index) = result.expect("Ingestion failure, this is bad.");
			indices.insert(sheet_name.to_string(), index);
		}

		*self.indices.write().unwrap() = Some(indices.into());

		tracing::info!("search ingestion complete");

		Ok(())
	}

	// TODO: continuation?
	pub fn search(
		&self,
		query: &pre::Node,
		sheet_filter: Option<HashSet<String>>,
		excel: &excel::Excel,
		schema: &dyn Schema,
	) -> Result<Warnings<Vec<SearchResult>>, SearchError> {
		let option = self.indices.read().expect("TODO error poisoned");
		let indices = option
			.as_ref()
			.expect("TODO handle case where indices are not eady yet");

		let normalizer = Normalizer::new(excel, schema);

		// This effectively creates a snapshot of the indices at the time of creation.
		let executor = Executor {
			indices: indices.clone(),
		};

		// Get an iterator for each of the indexes, lifting any errors from the initial search execution.
		// TODO: this can possibly be run in parallel to prevent queries that hit a lot of top-level sheets from blowing out response times
		let index_results = indices
			.keys()
			// Filter to the requested indexes if any sheet filer is specified.
			.filter(|name| {
				sheet_filter
					.as_ref()
					.map_or(true, |sheets| sheets.contains(name.as_str()))
			})
			// Execute the query on each matching index
			.map(|name| {
				// Normalise the query for each requested index using the provided schema.
				let normalized_query = normalizer.normalize(query, name)?;

				// Execute the query, tagging the results with the sheet the result is from.
				let results = executor.search(name, &normalized_query)?;
				let tagged_results = results.map(|result| SearchResult {
					score: result.score,
					sheet: name.to_owned(),
					row_id: result.row_id,
					subrow_id: result.subrow_id,
				});
				Ok(tagged_results)
			})
			.try_fold(Warnings::new(vec![]), |warnings, result| match result {
				// Successful search results can be pushed to the inner vector in the warnings.
				Ok(results) => Ok(warnings.map(|mut vec| {
					vec.push(results);
					vec
				})),
				// Failures should short circuit completely.
				Err(error @ SearchError::Failure(_)) => Err(error),
				// Query mismatches will be raised for most sheets, and aren't particularly meaningful for end-users. Skip.
				// TODO: ... right? i mean, it kind of sucks to not be able to say "oi this field doesn't exist" but... idk.
				Err(SearchError::QueryMismatch(_)) => Ok(warnings),
				// Other errors can be raised as warnings without halting the process.
				// TODO: find some way to tag this with the sheet name because at the moment the warnings are entirely unactionable.
				Err(error) => Ok(warnings.with_warning(error.to_string())),
			})?;

		// TODO: a zero-length array here implies all indices were query mismatches, or no index was queried at all. disambiguate and error out.
		// TODO: following the introduction of warnings; that's not quite right - it might all have ended up as warnings, too. While that's possibly _fine_ for i.e. a multi-sheet query, for a _single_ sheet query, it might be more-sane to raise as a top-level error. Think about it a bit, because... yeah. That's not exactly _consistent_ but maybe it's expected?

		// TODO: this just groups by index, effectively - should probably sort by score at this point
		// Merge the results from each index into a single vector.
		let results = index_results.map(|vec| vec.into_iter().flatten().collect::<Vec<_>>());

		Ok(results)
	}
}

// TODO: can probably store the number of search executions on this to feed into rate limiting
pub struct Executor {
	indices: Arc<HashMap<String, Index>>,
}

impl Executor {
	pub fn search(
		&self,
		sheet: &str,
		query: &post::Node,
	) -> Result<impl Iterator<Item = IndexResult>, SearchError> {
		let index = self
			.indices
			.get(sheet)
			.expect("TODO: error handling. this should probably be a hard fail?");

		index.search(self, query)
	}
}
