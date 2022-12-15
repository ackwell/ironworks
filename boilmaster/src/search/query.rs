#[derive(Debug)]
pub enum Node {
	Clause(Clause),
	Leaf(Leaf),
}

// TODO: this might be worth collapsing into the parent node struct?
#[derive(Debug)]
pub struct Clause {
	pub nodes: Vec<(Occur, Node)>,
}

#[derive(Debug)]
pub enum Occur {
	Must,
	Should,
	MustNot,
}

#[derive(Debug)]
pub struct Leaf {
	pub offset: u32,
	pub operation: Operation,
}

#[derive(Debug)]
pub enum Operation {
	Relation(Relation),

	Equal(Value),
}

#[derive(Debug)]
pub struct Relation {
	pub target: String,
	pub condition: Option<Box<Node>>,
	pub query: Box<Node>,
}

#[derive(Debug)]
pub enum Value {
	U64(u64),
}
