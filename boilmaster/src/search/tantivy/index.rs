use std::{collections::HashMap, fs, path::Path};

use ironworks::{
	excel::{Field, Language, Row, Sheet},
	file::exh,
};
use tantivy::{
	collector::TopDocs,
	directory::MmapDirectory,
	query::{BooleanQuery, Occur, TermQuery},
	schema, Document, IndexReader, IndexSettings, ReloadPolicy, Term, UserOperation,
};

use crate::{
	search::{error::Result, internal_query::post, search::Executor},
	version::VersionKey,
};

use super::{
	provider::IndexResult,
	resolve::QueryResolver,
	schema::{build_schema, column_field_name, ROW_ID, SHEET_KEY, SUBROW_ID},
};

pub struct Index {
	index: tantivy::Index,
	reader: IndexReader,
}

impl Index {
	pub fn new(path: &Path, sheet: &Sheet<String>) -> Result<Self> {
		// Open the directory of this index, ensuring it exists
		fs::create_dir_all(path)?;
		let directory = MmapDirectory::open(path)?;

		let index = match tantivy::Index::exists(&directory)? {
			true => tantivy::Index::open(directory)?,
			false => {
				let schema = build_schema(&sheet.columns()?, &sheet.languages()?);
				tantivy::Index::create(directory, schema, IndexSettings::default())?
			}
		};

		let reader = index
			.reader_builder()
			.reload_policy(ReloadPolicy::OnCommit)
			.try_into()?;

		Ok(Self { index, reader })
	}

	pub fn ingest(&self, writer_memory: usize, sheets: &[(u64, Sheet<String>)]) -> Result<()> {
		let mut writer = self.index.writer(writer_memory)?;
		let schema = self.index.schema();

		for (key, sheet) in sheets {
			let documents = match sheet_documents(*key, sheet, &schema) {
				Ok(documents) => documents,
				Err(error) => {
					// NOTE: This skips the sheet but doesn't prevent it being added to the metadata store, which means it'll be skipped on any other bulk ingests. That's probably fine, I imagine a forced re-ingestion can be performed if required by removing the key from meta first.
					tracing::error!(sheet = %sheet.name(), %key, ?error, "failed to build documents");
					continue;
				}
			};
			writer.run(documents.map(UserOperation::Add))?;
		}

		writer.commit()?;
		writer.wait_merging_threads()?;

		Ok(())
	}

	pub fn search(
		&self,
		version: VersionKey,
		sheet_key: u64,
		boilmaster_query: &post::Node,
		executor: &Executor,
	) -> Result<impl Iterator<Item = IndexResult>> {
		let searcher = self.reader.searcher();
		let schema = searcher.schema();

		let field_sheet_key = schema.get_field(SHEET_KEY).unwrap();

		// Resolve the query into the final tantivy query, limited to the requested sheet key.
		let query_resolver = QueryResolver {
			version,
			schema,
			executor,
		};
		let tantivy_query = BooleanQuery::new(vec![
			(Occur::Must, query_resolver.resolve(boilmaster_query)?),
			(
				Occur::Must,
				Box::new(TermQuery::new(
					Term::from_field_u64(field_sheet_key, sheet_key),
					schema::IndexRecordOption::Basic,
				)),
			),
		]);

		// Execute the search.
		// TODO: this results in each individuial index having a limit, as opposed to the whole query itself - think about how to approach this.
		let top_docs = searcher
			.search(&tantivy_query, &TopDocs::with_limit(100))
			.map_err(anyhow::Error::from)?;

		// Map the results into usable IDs.
		let field_row_id = schema.get_field(ROW_ID).unwrap();
		let field_subrow_id = schema.get_field(SUBROW_ID).unwrap();

		let get_u64 = |doc: &Document, field: schema::Field| doc.get_first(field)?.as_u64();
		let ids = move |document: &Document| -> Option<(u32, u16)> {
			let row_id = get_u64(document, field_row_id)?.try_into().ok()?;
			let subrow_id = get_u64(document, field_subrow_id)?.try_into().ok()?;
			Some((row_id, subrow_id))
		};

		let results = top_docs.into_iter().map(move |(score, doc_address)| {
			// Assuming that a search result can't suddenly point to nothing.
			let document = searcher.doc(doc_address).unwrap();
			let (row_id, subrow_id) = ids(&document).unwrap();

			IndexResult {
				score,
				row_id,
				subrow_id,
			}
		});

		Ok(results)
	}
}

fn sheet_documents(
	key: u64,
	sheet: &Sheet<String>,
	schema: &schema::Schema,
) -> Result<impl ExactSizeIterator<Item = Document>> {
	tracing::info!(sheet = %sheet.name(), "ingesting");

	let columns = sheet.columns()?;
	let languages = sheet.languages()?;

	// TODO: This effectively results in reading the entire sheet dataset into memory, which seems pretty wasteful - but `writer.run` requires an `ExactSizeIterator`, and I've as-yet been unable to get a better performing stream-alike solution to function sanely.
	let mut documents = HashMap::<(u32, u16), Document>::new();

	for language in languages {
		for row in sheet.with().language(language).iter() {
			let document = documents
				.entry((row.row_id(), row.subrow_id()))
				.or_insert_with(Document::new);
			hydrate_row_document(document, row, &columns, language, schema)?;
		}
	}

	// Fill in the ID/key fields for all of the documents that were built.
	let field_sheet_key = schema.get_field(SHEET_KEY).unwrap();
	let field_row_id = schema.get_field(ROW_ID).unwrap();
	let field_subrow_id = schema.get_field(SUBROW_ID).unwrap();
	for ((row_id, subrow_id), document) in documents.iter_mut() {
		document.add_u64(field_sheet_key, key);
		document.add_u64(field_row_id, (*row_id).into());
		document.add_u64(field_subrow_id, (*subrow_id).into());
	}

	Ok(documents.into_values())
}

fn hydrate_row_document(
	document: &mut Document,
	row: Row,
	columns: &[exh::ColumnDefinition],
	language: Language,
	schema: &schema::Schema,
) -> Result<()> {
	for column in columns {
		let field = schema
			.get_field(&column_field_name(column, language))
			.unwrap();
		let value = row.field(column)?;
		// TODO: this feels pretty repetetive given the column kind schema build - is it avoidable or nah?
		use Field as F;
		match value {
			// TODO: need to make sure the ingested strings don't contain non-string payloads
			F::String(value) => document.add_text(field, value),

			F::I8(value) => document.add_i64(field, value.into()),
			F::I16(value) => document.add_i64(field, value.into()),
			F::I32(value) => document.add_i64(field, value.into()),
			F::I64(value) => document.add_i64(field, value),

			F::U8(value) => document.add_u64(field, value.into()),
			F::U16(value) => document.add_u64(field, value.into()),
			F::U32(value) => document.add_u64(field, value.into()),
			F::U64(value) => document.add_u64(field, value),

			F::F32(value) => document.add_f64(field, value.into()),

			F::Bool(value) => document.add_u64(field, value.into()),
		}
	}

	Ok(())
}
