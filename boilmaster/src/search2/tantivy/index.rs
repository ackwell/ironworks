use std::{collections::HashMap, fs, path::Path};

use ironworks::{
	excel::{Field, Language, Row, Sheet},
	file::exh,
};
use tantivy::{
	directory::MmapDirectory, schema, Document, IndexReader, IndexSettings, ReloadPolicy,
	UserOperation,
};

use crate::search2::error::Result;

use super::schema::{build_schema, column_field_name, ROW_ID, SHEET_KEY, SUBROW_ID};

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
			.reload_policy(ReloadPolicy::Manual)
			.try_into()?;

		Ok(Self { index, reader })
	}

	pub fn ingest(&self, writer_memory: usize, sheets: &[(u64, Sheet<String>)]) -> Result<()> {
		let mut writer = self.index.writer(writer_memory)?;

		for (discriminant, sheet) in sheets {
			let documents = self.sheet_documents(*discriminant, sheet)?;
			writer.run(documents.map(UserOperation::Add))?;
		}

		writer.commit()?;
		writer.wait_merging_threads()?;

		Ok(())
	}

	fn sheet_documents(
		&self,
		discriminator: u64,
		sheet: &Sheet<String>,
	) -> Result<impl ExactSizeIterator<Item = Document>> {
		tracing::info!(sheet = %sheet.name(), "ingesting");

		let columns = sheet.columns()?;
		let languages = sheet.languages()?;

		let schema = self.index.schema();

		// TODO: This effectively results in reading the entire sheet dataset into memory, which seems pretty wasteful - but `writer.run` requires an `ExactSizeIterator`, and I've as-yet been unable to get a better performing stream-alike solution to function sanely.
		let mut documents = HashMap::<(u32, u16), Document>::new();

		for language in languages {
			for row in sheet.with().language(language).iter() {
				let document = documents
					.entry((row.row_id(), row.subrow_id()))
					.or_insert_with(Document::new);
				hydrate_row_document(document, row, &columns, language, &schema)?;
			}
		}

		// Fill in the ID/key fields for all of the documents that were built.
		let field_sheet_key = schema.get_field(SHEET_KEY).unwrap();
		let field_row_id = schema.get_field(ROW_ID).unwrap();
		let field_subrow_id = schema.get_field(SUBROW_ID).unwrap();
		for ((row_id, subrow_id), document) in documents.iter_mut() {
			document.add_u64(field_sheet_key, discriminator);
			document.add_u64(field_row_id, (*row_id).into());
			document.add_u64(field_subrow_id, (*subrow_id).into());
		}

		Ok(documents.into_values())
	}
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
