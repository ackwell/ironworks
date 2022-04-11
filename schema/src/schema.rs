/// Schema and metadata for a sheet within an Excel database.
#[derive(Debug)]
pub struct Sheet {
	/// Expected ordering of column definitions to be used when reading this schema.
	pub order: Order,

	/// The schema for the sheet.
	pub schema: Node,
}

/// Ordering of column definitions.
#[derive(Debug)]
pub enum Order {
	/// Ordered by index of definition within Excel header file.
	Index,
	/// Ordered by byte offset of columns within data.
	Offset,
}

/// Node within a sheet schema.
#[derive(Debug)]
pub enum Node {
	/// An array of two or more sub-schemas.
	#[allow(missing_docs)]
	Array { count: u32, schema: Box<Node> },

	// TODO: Reference fields
	/// A reference to one or more rows in other sheets.
	Reference(Vec<ReferenceTarget>),

	/// A single scalar field with no further semantics.
	Scalar,

	/// A collection of named sub-schemas.
	Struct(Vec<(String, Node)>),
}

impl Node {
	/// The size of a given node, in columns.
	pub fn size(&self) -> u32 {
		match self {
			Self::Array { count, schema } => count * schema.size(),
			Self::Reference(_) => 1,
			Self::Scalar => 1,
			Self::Struct(fields) => fields
				.iter()
				.fold(0u32, |size, (_, schema)| size + schema.size()),
		}
	}
}

// TODO: Should this all be public?
#[derive(Debug)]
pub struct ReferenceTarget {
	pub sheet: String,
	// TODO: Some sort of standardised field selector format?
	pub selector: Option<String>,
	pub condition: Option<ReferenceCondition>,
}

#[derive(Clone, Debug)]
pub struct ReferenceCondition {
	pub selector: String,
	// TODO: Technically this is an enum, but theoretically could be any value. Resolve?
	pub value: u32,
}
