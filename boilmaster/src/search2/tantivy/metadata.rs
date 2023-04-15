use std::{fs, path::Path, sync::RwLock};

use serde::{Deserialize, Serialize};
use tantivy::{
	collector::Count, directory::MmapDirectory, doc, query::TermQuery, schema, IndexReader,
	IndexWriter, ReloadPolicy, Term,
};

use crate::search2::error::Result;

const SHEET_KEY: &str = "sheet_key";
const METADATA: &str = "metadata";

#[derive(Serialize, Deserialize)]
pub struct Metadata {}

pub struct MetadataStore {
	reader: IndexReader,
	writer: RwLock<IndexWriter>,
}

impl MetadataStore {
	pub fn new(path: &Path) -> Result<Self> {
		let mut schema_builder = schema::SchemaBuilder::new();
		schema_builder.add_u64_field(SHEET_KEY, schema::INDEXED);
		schema_builder.add_json_field(METADATA, schema::STORED | schema::STRING);
		let schema = schema_builder.build();

		fs::create_dir_all(path)?;
		let directory = MmapDirectory::open(path)?;

		let index = tantivy::Index::open_or_create(directory, schema)?;

		let reader = index
			.reader_builder()
			.reload_policy(ReloadPolicy::OnCommit)
			.try_into()?;

		// Effectively can't go below 3MB without tantivy panicing.
		let writer = index.writer_with_num_threads(1, 3 * 1024 * 1024)?;

		Ok(Self {
			reader,
			writer: RwLock::new(writer),
		})
	}

	pub fn write(&self, entries: impl IntoIterator<Item = (u64, Metadata)>) -> Result<()> {
		// Execute the bulk of write logic on a read-lock, we only need to gain exclusive access for the commit.
		let writer = self.writer.read().expect("poisoned");
		let schema = writer.index().schema();
		let field_key = schema.get_field(SHEET_KEY).unwrap();
		let field_metadata = schema.get_field(METADATA).unwrap();

		// Insert all the entries. We delete by the key term first to make this act as an upsert.
		for (key, metadata) in entries {
			writer.delete_term(Term::from_field_u64(field_key, key));
			writer.add_document(doc!(
				field_key => key,
				field_metadata => serde_json::to_value(&metadata)?,
			))?;
		}

		drop(writer);

		// Entries have been added, wait for an opportunity to write and commit.
		self.writer.write().expect("poisoned").commit()?;

		Ok(())
	}

	pub fn exists(&self, key: u64) -> Result<bool> {
		let searcher = self.reader.searcher();
		let field = searcher.schema().get_field(SHEET_KEY).unwrap();
		let query = TermQuery::new(
			Term::from_field_u64(field, key),
			schema::IndexRecordOption::Basic,
		);
		let count = searcher.search(&query, &Count)?;

		Ok(count > 0)
	}
}
