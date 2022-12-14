pub enum Node {
	Clause(Clause),
	Leaf(Leaf),
}

// TODO: this might be worth collapsing into the parent node struct?
pub struct Clause {
	pub nodes: Vec<(Occur, Node)>,
}

pub enum Occur {
	Must,
	Should,
	MustNot,
}

pub struct Leaf {
	pub offset: u32,
	pub operation: Operation,
}

pub enum Operation {
	Relation(Relation),

	Equal(Value),
}

pub struct Relation {
	pub target: String,
	pub condition: Option<Box<Node>>,
	pub query: Box<Node>,
}

pub enum Value {
	UInt(u64),
}
