#[derive(Debug, Clone)]
pub enum Node<F, T> {
	Group(Group<F, T>),
	Leaf(Leaf<F, T>),
}

#[derive(Debug, Clone)]
pub struct Group<F, T> {
	pub clauses: Vec<(Occur, Node<F, T>)>,
}

#[derive(Debug, Clone)]
pub enum Occur {
	Must,
	Should,
	MustNot,
}

#[derive(Debug, Clone)]
pub struct Leaf<F, T> {
	/// Column offset this leaf targets.
	pub field: F,
	pub operation: Operation<F, T>,
}

#[derive(Debug, Clone)]
pub enum Operation<F, T> {
	Relation(Relation<F, T>),

	Match(String),

	Equal(Value),
	// TODO: all the other relevant leaf operations. will need both further math operations, as well as ranges and string ops (given i'm using this instead of generic string param)
}

#[derive(Debug, Clone)]
pub struct Relation<F, T> {
	pub target: T,
	/// Query to be executed on the target sheet's index.
	pub query: Box<Node<F, T>>,
}

#[derive(Debug, Clone)]
pub enum Value {
	/// A positive integer.
	U64(u64),
	/// A negative integer.
	I64(i64),
	/// A floating point number.
	F64(f64),
	/// A string.
	String(String),
}
