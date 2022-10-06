use std::{fs, path::Path};

use anyhow::Result;
use ironworks::excel::Excel;
use tantivy::{
	collector::TopDocs, directory::MmapDirectory, query::QueryParser, schema::FieldType,
	ReloadPolicy,
};

use super::ingest::ingest_sheet;

pub struct Index {
	sheet_name: String,
	// Do i actually need a reference to the index at all?
	index: tantivy::Index,
	reader: tantivy::IndexReader,
}

impl Index {
	pub fn new(sheet_name: &str, version_path: &Path, excel: &Excel) -> Result<Self> {
		let path = version_path.join(sheet_name);
		fs::create_dir_all(&path)?;
		let directory = MmapDirectory::open(&path)?;

		// TODO: this should likely be the part of the path that gets joined onto the base or similar
		let index = match tantivy::Index::exists(&directory)? {
			// NOTE: this assumes that the schema will be effectively immutable
			true => {
				tracing::debug!("Opening index {path:?}");
				tantivy::Index::open(directory)?
			}
			false => {
				tracing::debug!("Building index {path:?}");
				let sheet = excel.sheet(sheet_name)?;
				// TODO: this should do... something. retry? i don't know. if any step of ingestion fails. A failed ingest is pretty bad.
				// TODO: maybe rayon for executing the blocking init?
				ingest_sheet(&sheet, directory)
					.expect("TODO: error handling for ingestion failures")
			}
		};

		let reader = index
			.reader_builder()
			// TODO: this is set to manual 'cus technically an index is never updated in this setup. is that sane? i dunno?
			.reload_policy(ReloadPolicy::Manual)
			.try_into()?;

		Ok(Self {
			sheet_name: sheet_name.to_string(),
			index,
			reader,
		})
	}

	pub fn search(&self, query_string: &str) -> Result<Vec<(f32, u64)>> {
		let searcher = self.reader.searcher();

		// this is cloning the schema every search - should i just store a single copy of it on the struct?
		let schema = self.index.schema();

		// so immediate complication to deal with; need to specify the fields to search if the user (lmao as if) doesn't specify any. we... techncially want to search _every_thing. or, at least every string thing? idfk. all strings makes sense i guess?
		let fields = schema
			.fields()
			.filter(|(_field, entry)| matches!(entry.field_type(), FieldType::Str(_)))
			.map(|(field, _entry)| field)
			.collect::<Vec<_>>();

		let query_parser = QueryParser::for_index(&self.index, fields);
		let query = query_parser.parse_query(query_string)?;

		let top_docs = searcher.search(&query, &TopDocs::with_limit(100))?;
		let todo_result = top_docs
			.into_iter()
			.map(|(score, doc_address)| {
				let doc = searcher.doc(doc_address).expect(
					"TODO: error handling. is there any reasonable expectation this will fail?",
				);
				// TODO: error handling; schema field lookup can probably omve outside loop
				let id = doc
					.get_first(schema.get_field("row_id").unwrap())
					.unwrap()
					.as_u64()
					.unwrap();
				// todo: this should probably convert rowid to u32, and also obtain u16 subrow id i assume?
				(score, id)
			})
			.collect::<Vec<_>>();

		Ok(todo_result)
	}
}
