use std::collections::HashMap;

// enum_dispatch?

pub enum Node {
	Array(ArrayNode),
	Scalar(ScalarNode),
	Struct(StructNode),
}

pub struct ArrayNode {
	node: Box<Node>,
	count: u32,
}

pub struct ScalarNode {}

pub struct StructNode {
	nodes: HashMap<String, Node>,
}
