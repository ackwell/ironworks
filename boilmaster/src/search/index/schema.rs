use ironworks::{excel, file::exh};
use tantivy::schema;

use crate::data::LanguageString;

pub const ROW_ID: &str = "row_id";
pub const SUBROW_ID: &str = "subrow_id";

pub fn build_sheet_schema(
	columns: &[exh::ColumnDefinition],
	languages: &[excel::Language],
) -> schema::Schema {
	let mut schema_builder = schema::SchemaBuilder::new();

	// RowID and SubrowID are the only stored fields, search results can be looked up in real excel for the full dataset.
	schema_builder.add_u64_field(ROW_ID, schema::STORED);
	schema_builder.add_u64_field(SUBROW_ID, schema::STORED);

	for column in columns {
		for language in languages {
			add_column_field(&mut schema_builder, column, *language)
		}
	}

	schema_builder.build()
}

fn add_column_field(
	builder: &mut schema::SchemaBuilder,
	column: &exh::ColumnDefinition,
	language: excel::Language,
) {
	let name = column_field_name(column, language);

	use exh::ColumnKind as CK;
	match column.kind() {
		// TODO: per-language columns. at the moment, this is just english
		CK::String => builder.add_text_field(&name, schema::TEXT),

		CK::Int8 | CK::Int16 | CK::Int32 | CK::Int64 => {
			builder.add_i64_field(&name, schema::INDEXED)
		}

		CK::UInt8 | CK::UInt16 | CK::UInt32 | CK::UInt64 => {
			builder.add_u64_field(&name, schema::INDEXED)
		}

		CK::Float32 => builder.add_f64_field(&name, schema::INDEXED),

		// TODO: not sure how to handle bools... u64 each seems really wasteful
		CK::Bool
		| CK::PackedBool0
		| CK::PackedBool1
		| CK::PackedBool2
		| CK::PackedBool3
		| CK::PackedBool4
		| CK::PackedBool5
		| CK::PackedBool6
		| CK::PackedBool7 => builder.add_u64_field(&name, schema::INDEXED),
	};
}

pub fn column_field_name(column: &exh::ColumnDefinition, language: excel::Language) -> String {
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

	let language_string = LanguageString::from(language);
	let offset = column.offset();
	format!("{language_string}_{offset}{suffix}")
}
