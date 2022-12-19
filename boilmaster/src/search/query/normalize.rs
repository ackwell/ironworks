use ironworks::{excel, file::exh};
use ironworks_schema as schema;

use crate::search::{SchemaMismatchError, SearchError};

use super::{post, pre};

pub struct Normalizer<'a> {
	excel: &'a excel::Excel<'a>,
	schema: &'a dyn schema::Schema,
}

impl<'a> Normalizer<'a> {
	pub fn new(excel: &'a excel::Excel, schema: &'a dyn schema::Schema) -> Self {
		Self { excel, schema }
	}

	pub fn normalize(
		&self,
		query: &pre::Node,
		sheet_name: &str,
	) -> Result<post::Node, SearchError> {
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

		out
	}

	fn normalize_node(
		&self,
		node: &pre::Node,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> Result<post::Node, SearchError> {
		match node {
			pre::Node::Group(group) => self.normalize_group(group, schema, columns),
			pre::Node::Leaf(leaf) => self.normalize_leaf(leaf, schema, columns),
		}
	}

	fn normalize_group(
		&self,
		group: &pre::Group,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> Result<post::Node, SearchError> {
		Ok(post::Node::Group(post::Group {
			clauses: group
				.clauses
				.iter()
				.map(|(occur, node)| {
					Ok((occur.clone(), self.normalize_node(node, schema, columns)?))
				})
				.collect::<Result<Vec<_>, SearchError>>()?,
		}))
	}

	fn normalize_leaf(
		&self,
		leaf: &pre::Leaf,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> Result<post::Node, SearchError> {
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

		match &leaf.field {
			Some(specifier) => {
				self.normalize_leaf_bound(specifier, &leaf.operation, schema, columns)
			}
			None => self.normalize_leaf_unbound(&leaf.operation, schema, columns),
		}
	}

	fn normalize_leaf_bound(
		&self,
		specifier: &pre::FieldSpecifier,
		operation: &pre::Operation,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> Result<post::Node, SearchError> {
		match (specifier, schema) {
			// A struct specifier into a struct schema narrows the field space
			(pre::FieldSpecifier::Struct(field_name), schema::Node::Struct(fields)) => {
				// Get the requested field from the struct, mismatch if no such field exists.
				// Mismatch here implies the query and schema do not match.
				let field = fields
					.iter()
					.find(|field| &field.name == field_name)
					.ok_or_else(|| {
						SearchError::SchemaMismatch(SchemaMismatchError {
							field: field_name.into(),
							reason: "field does not exist".into(),
						})
					})?;

				// Narrow the column array to the columns relevant to the field, mismatch if those columns do not exist.
				// Mismatch here implies the game data and schema do not match.
				let start = usize::try_from(field.offset).unwrap();
				let end = start + usize::try_from(field.node.size()).unwrap();
				let narrowed_columns = columns.get(start..end).ok_or_else(|| {
					SearchError::SchemaMismatch(SchemaMismatchError {
						field: field_name.into(),
						reason: "game data does not contain enough columns".into(),
					})
				})?;

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
		operation: &pre::Operation,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> Result<post::Node, SearchError> {
		// TODO: if operation is in charge of "collecting" all the appropriate remaining fields to apply to, then perhaps unbound just passes directly to operation, given it's an unbounded selector?
		todo!("normalize leaf unbound")
	}

	fn normalize_operation(
		&self,
		operation: &pre::Operation,
		schema: &schema::Node,
		columns: &[exh::ColumnDefinition],
	) -> Result<post::Node, SearchError> {
		match operation {
			// TODO: should this panic if it _isn't_ a 1:1 relation:reference pair?
			//       no, it shouldn't - it could also be a struct... wait, can it?
			//       yeah, the callsite might have drilled into a struct, but this relation forms the basis of the next target, i think
			// so tldr;
			// for relations, if the schema is a reference, resolve the reference. if it's a struct, call down. if it's anything else, throw?
			pre::Operation::Relation(relation) => {
				//
				let node = match schema {
					schema::Node::Struct(fields) => todo!(
						"i think this is passing the entire schema node down to the subquery?"
					),

					schema::Node::Reference(targets) => {
						// uuuuh. references are scalars with glitter - so on the parent sheet (where this is executing), we want the field for this leaf as a scalar... right?
						let field = columns.get(0).expect("TODO: this is probably the same mismatch as the game data not enough coulumns thing");

						//
						let mut target_queries = targets
							.iter()
							.map(|target| {
								// this seems to be used for _one_ use case across all of stc - look into if it's worth supporting
								if target.selector.is_some() {
									todo!("todo: normalise reference target selectors")
								}

								// this should be modelled as a boolean group (+condition +innerquery)
								if target.condition.is_some() {
									todo!("TODO: normalise reference target conditions")
								}

								// TODO: this needs to handle schema mismatches and discard those branches. error time? error time.
								let query = self.normalize(&relation.query, &target.sheet)?;

								let operation = post::Operation::Relation(post::Relation {
									target: post::RelationTarget {
										sheet: target.sheet.clone(),
										condition: None, // todo
									},
									query: Box::new(query),
								});

								let node = post::Node::Leaf(post::Leaf {
									field: field.clone(),
									operation,
								});

								Ok((post::Occur::Should, node))
							})
							// Filter out schema mismatches to prune those branches - other errors will be raised.
							.filter(|result| !matches!(result, Err(SearchError::SchemaMismatch(_))))
							.collect::<Result<Vec<_>, _>>()?;

						// TODO: this is basically exactly the same as what i'm doing for ::equal - helper it?
						match target_queries.len() {
							0 => todo!("mismatch?"),

							1 => target_queries.swap_remove(0).1,

							_ => post::Node::Group(post::Group {
								clauses: target_queries,
							}),
						}
					}

					other => todo!("i think this is a schema mismatch {other:?}"),
				};

				Ok(node)
			}

			// TODO: this should collect all scalars i think?
			// TODO: this pattern will be pretty repetetive, make a utility that does this or something
			pre::Operation::Equal(value) => {
				let mut scalar_columns = collect_scalars(schema, columns, vec![]);
				match scalar_columns.len() {
					0 => todo!("guessing this should be like, a schema mismatch? maybe? TODO: work out what this means"),

					1 => Ok(post::Node::Leaf(post::Leaf {
						field: scalar_columns.swap_remove(0),
						operation: post::Operation::Equal(value.clone())
					})),

					_ => {
						let clauses = scalar_columns.into_iter().map(|column| {(
							post::Occur::Should,
							post::Node::Leaf(post::Leaf {
								field: column,
								operation: post::Operation::Equal(value.clone()),
							}),
						)}).collect::<Vec<_>>();

						Ok(post::Node::Group(post::Group { clauses }))
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
