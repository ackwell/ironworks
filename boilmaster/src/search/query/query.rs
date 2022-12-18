#[derive(Debug)]
pub enum Node<F, T> {
	Group(Group<F, T>),
	Leaf(Leaf<F, T>),
}

// TODO: this might be worth collapsing into the parent node struct?
#[derive(Debug)]
pub struct Group<F, T> {
	pub clauses: Vec<(Occur, Node<F, T>)>,
}

#[derive(Debug)]
pub enum Occur {
	Must,
	Should,
	MustNot,
}

#[derive(Debug)]
pub struct Leaf<F, T> {
	/// Column offset this leaf targets.
	// TODO: this struct targets post-normalised data, so i'm acting under the assumption that the normalisation process will turn non-targeted queries into a group of queries targeting specific fields.
	pub field: F,
	pub operation: Operation<F, T>,
}

#[derive(Debug)]
pub enum Operation<F, T> {
	Relation(Relation<F, T>),

	Equal(Value),
	// TODO: all the other relevant leaf operations. will need both further math operations, as well as ranges and string ops (given i'm using this instead of generic string param)
}

#[derive(Debug)]
pub struct Relation<F, T> {
	pub target: T,
	/// Query to be executed on the target sheet's index.
	pub query: Box<Node<F, T>>,
}

// TODO: this can probably be used on both sides of normalisation
#[derive(Debug)]
pub enum Value {
	/// Represents any positive integer
	U64(u64),
	// TODO: other value types - we'll need something for negative ints (i64?), floats (f64?), and strings (String?) at minimum
}
