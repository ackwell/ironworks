use std::{
	collections::{HashMap, HashSet},
	path::PathBuf,
	sync::{Arc, RwLock},
};

use futures::{stream::FuturesUnordered, StreamExt};
use ironworks::file::exh;

use crate::data::Version as DataVersion;

use super::{
	error::SearchError,
	index::{Index, IndexResult, Ingester},
	query::post::{Group, Leaf, Node, Occur, Operation, Relation, Value},
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
		let list = excel.list().map_err(anyhow::Error::from)?;
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

		Ok(())
	}

	// TODO: index specifier?
	// TODO: non-string-query filters
	// TODO: continuation?
	pub fn search(&self, query: &str) -> Result<Vec<SearchResult>, SearchError> {
		let option = self.indices.read().expect("TODO error poisoned");
		let indices = option
			.as_ref()
			.expect("TODO handle case where indices are not eady yet");

		// TODO: arg...?
		let sheet_filter: Option<HashSet<String>> = Some(HashSet::from(["Item".into()]));
		// let sheet_filter: Option<HashSet<String>> = None;

		// TODO: arg
		// let query_node = Node::Leaf(Leaf {
		// 	offset: 138,
		// 	operation: Operation::Equal(Value::UInt(635)),
		// });
		// TODO: WHEN REMOVING THIS< REMOVE THE COLDEF CTOR
		let query_node = Node::Group(Group {
			clauses: vec![
				(
					Occur::Must,
					Node::Leaf(Leaf {
						column: exh::ColumnDefinition::TEMP_new(exh::ColumnKind::Int16, 0x36),
						operation: Operation::Equal(Value::U64(358)),
					}),
				),
				(
					Occur::Must,
					Node::Leaf(Leaf {
						column: exh::ColumnDefinition::TEMP_new(exh::ColumnKind::Int16, 0x38),
						operation: Operation::Equal(Value::U64(389)),
					}),
				),
				(
					Occur::Must,
					Node::Leaf(Leaf {
						column: exh::ColumnDefinition::TEMP_new(exh::ColumnKind::UInt8, 0x55),
						operation: Operation::Relation(Relation {
							target: "ClassJob".into(),
							condition: None,
							query: Box::new(Node::Leaf(Leaf {
								column: exh::ColumnDefinition::TEMP_new(
									exh::ColumnKind::UInt8,
									0x58,
								),
								operation: Operation::Equal(Value::U64(22)),
							})),
						}),
					}),
				),
			],
		});

		// This effectively creates a snapshot of the indices at the time of creation.
		let executor = Executor {
			indices: indices.clone(),
		};

		// Get an iterator for each of the indexes, lifting any errors from the initial search execution.
		let index_results = indices
			.iter()
			// Filter to the requested indexes if any sheet filer is specified.
			.filter(|(name, _)| {
				sheet_filter
					.as_ref()
					.map_or(true, |sheets| sheets.contains(name.as_str()))
			})
			// Execute the query on each matching index
			.map(|(name, index)| {
				// let results = index.search(&query_node)?;
				let results = executor.search(name, &query_node)?;
				let tagged_results = results.map(|result| SearchResult {
					score: result.score,
					sheet: name.to_owned(),
					row_id: result.row_id,
					subrow_id: result.subrow_id,
				});
				Ok(tagged_results)
			})
			.collect::<Result<Vec<_>, SearchError>>()?;

		// TODO: this just groups by index, effectively - should probably sort by score at this point
		// Merge the results from each index into a single vector.
		let results = index_results.into_iter().flatten().collect::<Vec<_>>();

		Ok(results)
	}
}

pub struct Executor {
	indices: Arc<HashMap<String, Index>>,
}

impl Executor {
	pub fn search(
		&self,
		sheet: &str,
		query: &Node,
	) -> Result<impl Iterator<Item = IndexResult>, SearchError> {
		let index = self
			.indices
			.get(sheet)
			.expect("TODO: error handling. this should probably be a hard fail?");

		index.search(self, query)
	}
}
