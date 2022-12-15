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
	/// Column offset this leaf targets.
	// TODO: this struct targets post-normalised data, so i'm acting under the assumption that the normalisation process will turn non-targeted queries into a group of queries targeting specific fields.
	pub offset: u32,
	pub operation: Operation,
}

#[derive(Debug)]
pub enum Operation {
	Relation(Relation),

	Equal(Value),
	// TODO: all the other relevant leaf operations. will need both further math operations, as well as ranges and string ops (given i'm using this instead of generic string param)
}

#[derive(Debug)]
pub struct Relation {
	/// Target sheet of this relation
	pub target: String,
	/// Additional conditions on the parent sheet required for this relation to be applicable for a row.
	pub condition: Option<Box<Node>>,
	/// Query to be executed on the target sheet's index.
	pub query: Box<Node>,
}

// TODO: this can probably be used on both sides of normalisation
#[derive(Debug)]
pub enum Value {
	/// Represents any positive integer
	U64(u64),
	// TODO: other value types - we'll need something for negative ints (i64?), floats (f64?), and strings (String?) at minimum
}
