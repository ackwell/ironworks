use ironworks::file::exh;
use tantivy::schema;

pub const ROW_ID: &str = "row_id";
pub const SUBROW_ID: &str = "subrow_id";

pub fn build_sheet_schema(columns: &[exh::ColumnDefinition]) -> schema::Schema {
	let mut schema_builder = schema::Schema::builder();

	// RowID and SubrowID are the only stored fields, search results can be looked up in real excel for the full dataset.
	schema_builder.add_u64_field(ROW_ID, schema::STORED);
	schema_builder.add_u64_field(SUBROW_ID, schema::STORED);

	for column in columns {
		let name = column_field_name(column);

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

	schema_builder.build()
}

pub fn column_field_name(column: &exh::ColumnDefinition) -> String {
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
