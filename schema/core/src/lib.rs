use std::collections::HashMap;

// enum_dispatch?

// TODO: think about the types in the enum a bit. Should array have named fields? Should the struct values have named fields?
#[derive(Debug)]
pub enum Node {
	Array(u32, Box<Node>),
	Reference(Vec<ReferenceTarget>),
	Scalar,
	Struct(HashMap<String, (u32, Node)>),
}

impl Node {
	pub fn size(&self) -> u32 {
		match self {
			Self::Array(count, node) => count * node.size(),
			Self::Reference(_) => 1,
			Self::Scalar => 1,
			// TODO: Think about this a bit. Currently calcing the size of every child, but that doesn't take offsets into account. Perhaps would be better-represented as (largest offset + size of node at largest offset)
			Self::Struct(nodes) => nodes
				.values()
				.fold(0u32, |size, (_, node)| size + node.size()),
		}
	}
}

// TODO: not convinced all these fields should be public
#[derive(Debug)]
pub struct ReferenceTarget {
	pub sheet: String,
	// TODO: Some sort of standardised field selector format/struct/something?
	pub selector: Option<String>,
	pub condition: Option<ReferenceCondition>,
}

#[derive(Clone, Debug)]
pub struct ReferenceCondition {
	pub selector: String,
	// TODO: technically this is an enum, but theoretically could be any value. Resolve?
	pub value: u32,
}
