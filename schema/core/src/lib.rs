use std::collections::HashMap;

// enum_dispatch?

// TODO: think about the types in the enum a bit. Should array have named fields? Should the struct values have named fields?
#[derive(Debug)]
pub enum Node {
	Array(u32, Box<Node>),
	Scalar,
	Struct(HashMap<String, (u32, Node)>),
}
