use std::collections::HashMap;

use anyhow::Result;
use ironworks::{
	excel::{Field, Language, Row, Sheet},
	file::exh,
};
use serde::Deserialize;
use tantivy::{directory::MmapDirectory, schema, Document, Index, IndexSettings, UserOperation};
use tokio::sync::Semaphore;

use crate::search::index::tokenize::register_tokenizers;

use super::schema::{build_sheet_schema, column_field_name, ROW_ID, SUBROW_ID};

#[derive(Debug, Deserialize)]
pub struct IngestConfig {
	concurrency: usize,
	memory: usize,
}

#[derive(Debug)]
pub struct Ingester {
	semaphore: Semaphore,
	writer_memory: usize,
}

impl Ingester {
	pub fn new(config: IngestConfig) -> Self {
		Self {
			semaphore: Semaphore::new(config.concurrency),
			// Memory limit represents the total available across all writers.
			writer_memory: config.memory / config.concurrency,
		}
	}

	pub async fn ingest_sheet(
		&self,
		sheet: Sheet<'static, String>,
		directory: MmapDirectory,
	) -> Result<Index> {
		// Wait for a semaphore permit to be available - this limits the number of parallel ingestions that can take place.
		let permit = self.semaphore.acquire().await.unwrap();

		// TODO: this should probably span the function so i get an end point
		tracing::info!("ingesting {}", sheet.name());

		let writer_memory = self.writer_memory;

		// We have a permit - initiate a blocking task to ingest the sheet.
		let index = tokio::task::spawn_blocking(move || -> Result<_> {
			// TODO: seperate building the index from ingesting into it
			let columns = sheet.columns()?;
			let languages = sheet.languages()?;

			let index = Index::create(
				directory,
				build_sheet_schema(&columns, &languages),
				IndexSettings::default(),
			)?;
			register_tokenizers(&index);

			let mut writer = index.writer(writer_memory)?;
			let schema = index.schema();

			let mut documents = HashMap::<(u32, u16), Document>::new();

			for language in languages {
				for row in sheet.with().language(language).iter() {
					let document = documents
						.entry((row.row_id(), row.subrow_id()))
						.or_insert_with(Document::new);
					hydrate_row_document(document, row, &columns, language, &schema)?;
				}
			}

			// TODO: if there's any failures at all (i.e. iw read errors) during ingestion, the writer should be rolled back to ensure a theoretical retry is able to work on a clean deck.
			writer.run(documents.into_values().map(UserOperation::Add))?;

			writer.commit()?;

			Ok(index)
		})
		.await
		.unwrap()?;

		drop(permit);

		Ok(index)
	}
}

fn hydrate_row_document(
	document: &mut Document,
	row: Row,
	columns: &[exh::ColumnDefinition],
	language: Language,
	schema: &schema::Schema,
) -> Result<()> {
	document.add_u64(schema.get_field(ROW_ID).unwrap(), (row.row_id()).into());
	document.add_u64(
		schema.get_field(SUBROW_ID).unwrap(),
		(row.subrow_id()).into(),
	);

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
