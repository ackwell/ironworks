use ironworks::{excel, file::exh};

use super::query;

// Re-export the query types, with assigned generics.
pub type Node = query::Node<LeafField, RelationTarget>;
pub type Group = query::Group<LeafField, RelationTarget>;
pub type Leaf = query::Leaf<LeafField, RelationTarget>;
pub type Operation = query::Operation<LeafField, RelationTarget>;
pub type Relation = query::Relation<LeafField, RelationTarget>;

pub use query::{Occur, Value};

// Types specific to post-normalised queries
pub type LeafField = (exh::ColumnDefinition, excel::Language);

#[derive(Debug)]
pub struct RelationTarget {
	pub sheet: String,
	pub condition: Option<Box<Node>>,
}
