// TODO: consider making internals on these private with getters?

/// Schema and metadata for a sheet within an Excel database.
#[derive(Debug)]
pub struct Sheet {
	/// Canonical name of the sheet.
	pub name: String,

	/// Expected ordering of column definitions to be used when reading this schema.
	pub order: Order,

	/// The schema for the sheet.
	pub node: Node,
}

/// Ordering of column definitions.
#[derive(Debug, PartialEq)]
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
	Array { count: u32, node: Box<Node> },

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
			Self::Array { count, node } => count * node.size(),
			Self::Reference(_) => 1,
			Self::Scalar => 1,
			Self::Struct(fields) => fields
				.iter()
				.fold(0u32, |size, (_, schema)| size + schema.size()),
		}
	}
}

// TODO: Should this all be public?
/// Metadata for a reference to a row in another sheet.
#[derive(Debug)]
pub struct ReferenceTarget {
	/// The sheet this reference points to
	pub sheet: String,
	// TODO: Some sort of standardised field selector format?
	/// Selector pointing to the column in the target sheet that the value of this
	/// reference matches. If `None`, the row ID is used.
	pub selector: Option<String>,
	/// Condition that must match for this target to be considered valid. If `None`,
	/// this target is always valid.
	pub condition: Option<ReferenceCondition>,
}

/// Selector/value pair used to limit the validity of a `ReferenceTarget`.
#[derive(Clone, Debug)]
pub struct ReferenceCondition {
	/// Selector pointing to the column in this sheet that will be matched against.
	pub selector: String,
	// TODO: Technically this is an enum, but theoretically could be any value. Resolve?
	/// Value that will be matched against.
	pub value: u32,
}
