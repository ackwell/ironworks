use ironworks::{excel, file::exh};
use ironworks_schema as schema;

use crate::search::SearchError;

use super::{post, pre};

pub struct Normalizer<'a> {
	excel: &'a excel::Excel<'a>,
	schema: &'a dyn schema::Schema,
}

impl<'a> Normalizer<'a> {
	pub fn new(excel: &'a excel::Excel, schema: &'a dyn schema::Schema) -> Self {
		Self { excel, schema }
	}

	pub fn normalize(&self, query: pre::Node, sheet_name: &str) -> Result<post::Node, SearchError> {
		let sheet_schema = self
			.schema
			.sheet(sheet_name)
			.expect("TODO: what does this mean?");

		let sheet_data = self
			.excel
			.sheet(sheet_name)
			.expect("TODO: What does this mean?");

		let columns = sheet_data
			.columns()
			.expect("TODO: what the fuck does this mean?");

		let out = self.normalize_node(query, &sheet_schema.node, &columns);

		Ok(out)
	}

	fn normalize_node(
		&self,
		node: pre::Node,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> post::Node {
		match node {
			pre::Node::Group(group) => self.normalize_group(group, schema, columns),
			pre::Node::Leaf(leaf) => self.normalize_leaf(leaf, schema, columns),
		}
	}

	fn normalize_group(
		&self,
		group: pre::Group,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> post::Node {
		post::Node::Group(post::Group {
			clauses: group
				.clauses
				.into_iter()
				.map(|(occur, node)| (occur, self.normalize_node(node, schema, columns)))
				.collect(),
		})
	}

	fn normalize_leaf(
		&self,
		leaf: pre::Leaf,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> post::Node {
		// // let fsda = self.schema.sheet("fdsaf").unwrap().node;
		// let fas = schema;

		// // THOUGHTS
		// // if the field is Some(value), then resolve that value to a column definition
		// // if the field is None, then it becomes a group of every field possible, BUT
		// //   we can probably "localise" that to a subset of columns in nested-but-not-relational structs
		// let a = match leaf.field {
		// 	Some(field_name) => todo!("specified {field_name:?}"),
		// 	None => todo!("no specified field"),
		// };

		match leaf.field {
			Some(specifier) => {
				self.normalize_leaf_bound(specifier, leaf.operation, schema, columns)
			}
			None => self.normalize_leaf_unbound(leaf.operation, schema, columns),
		}
	}

	fn normalize_leaf_bound(
		&self,
		specifier: pre::FieldSpecifier,
		operation: pre::Operation,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> post::Node {
		match (specifier, schema) {
			// A struct specifier into a struct schema narrows the field space
			(pre::FieldSpecifier::Struct(field_name), schema::Node::Struct(fields)) => {
				let field = fields
					.iter()
					.find(|field| field.name == field_name)
					.expect("TODO: this should return schema mismatch");

				// TODO: this will probably need to use field.offset and field.node.size to narrow the exh array
				let start = usize::try_from(field.offset).unwrap();
				let end = start + usize::try_from(field.node.size()).unwrap();
				let narrowed_columns = columns
					.get(start..end)
					.expect("TODO: WHAT DOES THIS MEAAAAAAAAAAAAN?");

				self.normalize_operation(operation, &field.node, narrowed_columns)
			}

			// TODO: reference
			// a (struct, reference) pair means... what
			// references are equivalent in data to a scalar, i.e. it's a leaf of an individual schema (though points to another)
			// i'm tempted to say that this should never occur. normalising the relation operation should handle references at that point, which would leave the inner leaf bound to already be pointing at something else. leaf bounds are inherently a structural detail, and scalars (and references) are not structural. think on that a bit more

			// TODO: array

			//
			(sp, sc) => todo!("{sp:?} {sc:?}"),
		}
	}

	fn normalize_leaf_unbound(
		&self,
		operation: pre::Operation,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> post::Node {
		// TODO: if operation is in charge of "collecting" all the appropriate remaining fields to apply to, then perhaps unbound just passes directly to operation, given it's an unbounded selector?
		todo!("normalize leaf unbound")
	}

	fn normalize_operation(
		&self,
		operation: pre::Operation,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> post::Node {
		match operation {
			// TODO: should this panic if it _isn't_ a 1:1 relation:reference pair?
			//       no, it shouldn't - it could also be a struct... wait, can it?
			//       yeah, the callsite might have drilled into a struct, but this relation forms the basis of the next target, i think
			// so tldr;
			// for relations, if the schema is a reference, resolve the reference. if it's a struct, call down. if it's anything else, throw?
			pre::Operation::Relation(relation) => todo!(),

			// TODO: this should collect all scalars i think?
			// TODO: this pattern will be pretty repetetive, make a utility that does this or something
			pre::Operation::Equal(value) => {
				let mut scalar_columns = collect_scalars(schema, columns, vec![]);
				match scalar_columns.len() {
					0 => todo!("guessing this should be like, a schema mismatch? maybe? TODO: work out what this means"),

					1 => post::Node::Leaf(post::Leaf {
						field: scalar_columns.swap_remove(0),
						operation: post::Operation::Equal(value)
					}),

					_ => {
						let clauses = scalar_columns.into_iter().map(|column| {(
							post::Occur::Should,
							post::Node::Leaf(post::Leaf {
								field: column,
								operation: post::Operation::Equal(value.clone()),
							}),
						)}).collect::<Vec<_>>();

						post::Node::Group(post::Group { clauses })
					}
				}
			}
		}
	}
}

// The whole premise of this is that we want to _exclude_ references. If that premise does not hold, then the `columns` slice itself is basically exactly what we want.
fn collect_scalars(
	schema: &schema::Node,
	columns: &[exh::ColumnDefinition],
	mut output: Vec<exh::ColumnDefinition>,
) -> Vec<exh::ColumnDefinition> {
	match schema {
		schema::Node::Array { count, node } => {
			// TODO: this is pretty silly, can technically derive the range from 1 call down.
			let size = usize::try_from(node.size()).unwrap();
			let count = usize::try_from(*count).unwrap();
			(0..count).fold(output, |output, index| {
				let start = index * size;
				let end = start + size;
				let slice = columns
					.get(start..end)
					.expect("TODO: what's the failure mode here?");
				collect_scalars(node, slice, output)
			})
		}

		schema::Node::Reference(_references) => {
			// ignore refs?
			output
		}

		schema::Node::Scalar => {
			output.push(
				columns
					.get(0)
					.expect("TODO: what's the failure mode here?")
					.clone(),
			);
			output
		}

		schema::Node::Struct(fields) => fields.iter().fold(output, |output, field| {
			let start = usize::try_from(field.offset).unwrap();
			let end = start + usize::try_from(field.node.size()).unwrap();
			let slice = columns
				.get(start..end)
				.expect("TODO: what's the failure mode here?");
			collect_scalars(&field.node, slice, output)
		}),
	}
}
