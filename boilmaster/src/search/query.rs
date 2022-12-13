use tantivy::query::{Occur, Query};

pub struct Clause {
	pub nodes: Vec<(Occur, Node)>,
}

pub enum Node {
	Query(Box<dyn Query>),
	Clause(Clause),
	Relation(Relation),
}

pub struct Relation {
	pub target: String,
	pub condition: Option<Box<dyn Query>>,
	pub clause: Clause,
}
