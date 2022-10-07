use anyhow::Result;
use ironworks::{
	excel::{Field, Row, Sheet},
	file::exh,
};
use tantivy::{directory::MmapDirectory, schema, Document, Index, IndexSettings};

pub fn ingest_sheet(sheet: &Sheet<&str>, directory: MmapDirectory) -> Result<tantivy::Index> {
	let columns = sheet.columns()?;

	let index = Index::create(
		directory,
		build_sheet_schema(&columns)?,
		IndexSettings::default(),
	)?;

	// Allocating 5mb for writer ingestion.
	// TODO: Is this per index? (yes it is) A bulk version ingestion might be problematic @ 5mb/ea, test this behavior. there's ~6.5k sheets, so in the unlikely case all of them are being written at the same time, that's a gnarly amount of ram.
	// TODO: probably should be configurable anyway.
	// TODO: realistically, ingestion needs to be a queue system
	let mut writer = index.writer(5 * 1024 * 1024)?;

	// TODO: if there's any failures at all (i.e. iw read errors) during ingestion, the writer should be rolled back to ensure a theoretical retry is able to work on a clean deck.
	for row in sheet.iter() {
		let document = build_row_document(row, &columns, &index.schema())?;

		// this can block; which suggests to me that at minimum, ingesting should be done on the side.
		writer.add_document(document)?;
	}

	writer.commit()?;

	Ok(index)
}

fn build_sheet_schema(columns: &[exh::ColumnDefinition]) -> Result<schema::Schema> {
	let mut schema_builder = schema::Schema::builder();

	// RowID and SubrowID are the only stored fields, search results can be looked up in real excel for the full dataset.
	schema_builder.add_u64_field("row_id", schema::STORED);
	schema_builder.add_u64_field("subrow_id", schema::STORED);

	for column in columns {
		let name = column_to_field_name(column);

		use exh::ColumnKind as CK;
		match column.kind() {
			// TODO: per-language columns. at the moment, this is just english
			CK::String => schema_builder.add_text_field(&name, schema::TEXT),

			CK::Int8 | CK::Int16 | CK::Int32 | CK::Int64 => {
				schema_builder.add_i64_field(&name, schema::INDEXED)
			}

			CK::UInt8 | CK::UInt16 | CK::UInt32 | CK::UInt64 => {
				schema_builder.add_u64_field(&name, schema::INDEXED)
			}

			CK::Float32 => schema_builder.add_f64_field(&name, schema::INDEXED),

			// TODO: not sure how to handle bools... u64 each seems really wasteful
			CK::Bool
			| CK::PackedBool0
			| CK::PackedBool1
			| CK::PackedBool2
			| CK::PackedBool3
			| CK::PackedBool4
			| CK::PackedBool5
			| CK::PackedBool6
			| CK::PackedBool7 => schema_builder.add_u64_field(&name, schema::INDEXED),
		};
	}

	Ok(schema_builder.build())
}

fn build_row_document(
	row: Row,
	columns: &[exh::ColumnDefinition],
	schema: &schema::Schema,
) -> Result<Document> {
	let mut document = Document::new();

	document.add_u64(schema.get_field("row_id").unwrap(), (*row.row_id()).into());
	document.add_u64(
		schema.get_field("subrow_id").unwrap(),
		(*row.subrow_id()).into(),
	);

	for (index, column) in columns.iter().enumerate() {
		let field = schema.get_field(&column_to_field_name(column)).unwrap();
		// TODO: this would really value .field(impl intocolumn) or similar
		let value = row.field(index)?;
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

	Ok(document)
}

fn column_to_field_name(column: &exh::ColumnDefinition) -> String {
	// For packed bool columns, offset alone is not enough to disambiguate a
	// field - add a suffix of the packed bit position.
	use exh::ColumnKind as CK;
	let suffix = match column.kind() {
		CK::PackedBool0 => "_0",
		CK::PackedBool1 => "_1",
		CK::PackedBool2 => "_2",
		CK::PackedBool3 => "_3",
		CK::PackedBool4 => "_4",
		CK::PackedBool5 => "_5",
		CK::PackedBool6 => "_6",
		CK::PackedBool7 => "_7",
		_ => "",
	};

	format!("{}{suffix}", column.offset())
}
