use std::collections::HashMap;

// enum_dispatch?

// TODO: think about the types in the enum a bit. Should array have named fields? Should the struct values have named fields?
#[derive(Debug)]
pub enum Node {
	Array(u32, Box<Node>),
	Scalar,
	Struct(HashMap<String, (u32, Node)>),
}

impl Node {
	pub fn size(&self) -> u32 {
		match self {
			Self::Array(count, node) => count * node.size(),
			Self::Scalar => 1,
			// TODO: Think about this a bit. Currently calcing the size of every child, but that doesn't take offsets into account. Perhaps would be better-represented as (largest offset + size of node at largest offset)
			Self::Struct(nodes) => nodes
				.values()
				.fold(0u32, |size, (_, node)| size + node.size()),
		}
	}
}
