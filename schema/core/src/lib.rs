use std::collections::HashMap;

// enum_dispatch?

#[derive(Debug)]
pub enum Node {
	Array(ArrayNode),
	Scalar(ScalarNode),
	Struct(StructNode),
}

#[derive(Debug)]
pub struct ArrayNode {
	node: Box<Node>,
	count: u32,
}

#[derive(Debug)]
pub struct ScalarNode {}

impl ScalarNode {
	pub fn new() -> Self {
		Self {}
	}
}

#[derive(Debug)]
pub struct StructNode {
	nodes: HashMap<String, (u32, Node)>,
}

impl StructNode {
	pub fn new(nodes: HashMap<String, (u32, Node)>) -> Self {
		Self { nodes }
	}
}
