pub enum Node {
	Clause(Clause),
	Leaf(Leaf),
}

pub struct Clause {
	nodes: Vec<(Occur, Node)>,
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
	pub clause: Clause,
}
