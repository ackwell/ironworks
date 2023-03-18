use ironworks::{excel, file::exh};
use ironworks_schema as schema;

use crate::{
	search::{MismatchError, SearchError},
	utility::{anyhow::Anyhow, field},
};

use super::{post, pre};

#[derive(Clone)]
struct Context<'a> {
	languages: &'a [excel::Language],

	schema: &'a schema::Node,
	columns: &'a [exh::ColumnDefinition],
	language: excel::Language,
}

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
		ambient_language: excel::Language,
	) -> Result<post::Node, SearchError> {
		// Fetch the schema and columns for the requested sheet.
		let sheet_schema = self.schema.sheet(sheet_name).map_err(|error| match error {
			// A missing schema can be considered analogous to a missing field _in_ a
			// schema, and is such a mismatch between the query and the schema.
			schema::Error::NotFound(inner) => SearchError::QuerySchemaMismatch(MismatchError {
				field: inner.to_string(),
				reason: "not found".into(),
			}),
			other => SearchError::Failure(other.into()),
		})?;

		let sheet_data = self.excel.sheet(sheet_name).map_err(|error| match error {
			ironworks::Error::NotFound(ironworks::ErrorValue::Sheet(sheet)) => {
				SearchError::SchemaGameMismatch(MismatchError {
					field: sheet,
					reason: "not found".into(),
				})
			}
			other => SearchError::Failure(other.into()),
		})?;

		let languages = sheet_data.languages().anyhow()?;
		let columns = sheet_data.columns().anyhow()?;

		// Check if the ambient language is valid for this sheet, trying to fall
		// back to `None` if it is not, to mimic read behavior.
		let language = [ambient_language, excel::Language::None]
			.into_iter()
			.find(|language| languages.contains(language))
			.ok_or_else(|| {
				SearchError::QueryGameMismatch(MismatchError {
					field: format!("sheet {sheet_name}"),
					reason: format!("unsupported language {ambient_language:?}"),
				})
			})?;

		// Start walking the node tree
		self.normalize_node(
			query,
			Context {
				languages: &languages,
				schema: &sheet_schema.node,
				columns: &columns,
				language,
			},
		)
	}

	fn normalize_node(
		&self,
		node: &pre::Node,
		context: Context,
	) -> Result<post::Node, SearchError> {
		match node {
			pre::Node::Group(group) => self.normalize_group(group, context),
			pre::Node::Leaf(leaf) => self.normalize_leaf(leaf, context),
		}
	}

	fn normalize_group(
		&self,
		group: &pre::Group,
		context: Context,
	) -> Result<post::Node, SearchError> {
		Ok(post::Node::Group(post::Group {
			clauses: group
				.clauses
				.iter()
				.map(|(occur, node)| {
					Ok((occur.clone(), self.normalize_node(node, context.clone())?))
				})
				.collect::<Result<Vec<_>, SearchError>>()?,
		}))
	}

	fn normalize_leaf(
		&self,
		leaf: &pre::Leaf,
		context: Context,
	) -> Result<post::Node, SearchError> {
		match &leaf.field {
			Some(specifier) => self.normalize_leaf_bound(specifier, &leaf.operation, context),
			None => self.normalize_leaf_unbound(&leaf.operation, context),
		}
	}

	fn normalize_leaf_bound(
		&self,
		specifier: &pre::FieldSpecifier,
		operation: &pre::Operation,
		context: Context,
	) -> Result<post::Node, SearchError> {
		match (specifier, context.schema) {
			// A struct specifier into a struct schema narrows the field space
			(
				pre::FieldSpecifier::Struct(field_name, requested_language),
				schema::Node::Struct(fields),
			) => {
				// Get the requested field from the struct, mismatch if no such field exists.
				// Mismatch here implies the query and schema do not match.
				let field = fields
					.iter()
					// TODO: this is _really_ wasteful. see TODO in the utility file w/r/t sanitizing schema preemptively
					.find(|field| &field::sanitize_name(&field.name) == field_name)
					.ok_or_else(|| {
						SearchError::QuerySchemaMismatch(MismatchError {
							field: field_name.into(),
							reason: "field does not exist".into(),
						})
					})?;

				// Get the requested language, falling back to the contextual language.
				// We do _not_ fall back to `Language::None` here - an explicit request
				// for an invalid language should fail. As-is, the contextual language
				// is already coerced to `Language::None` at the sheet boundary `.normalize`
				// call, so this will already fall back to `None` unless an erroneous
				// language is requested explicitly.
				let language = requested_language.unwrap_or(context.language);
				if !context.languages.contains(&language) {
					return Err(SearchError::QueryGameMismatch(MismatchError {
						field: field_name.into(),
						reason: format!("{language:?} is not supported by this sheet"),
					}));
				}

				// Narrow the column array to the columns relevant to the field, mismatch if those columns do not exist.
				// Mismatch here implies the game data and schema do not match.
				let start = usize::try_from(field.offset).unwrap();
				let end = start + usize::try_from(field.node.size()).unwrap();
				let narrowed_columns = context.columns.get(start..end).ok_or_else(|| {
					SearchError::SchemaGameMismatch(MismatchError {
						field: field_name.into(),
						reason: "game data does not contain enough columns".into(),
					})
				})?;

				self.normalize_operation(
					operation,
					Context {
						schema: &field.node,
						columns: narrowed_columns,
						language,
						..context
					},
				)
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
		_operation: &pre::Operation,
		_context: Context,
	) -> Result<post::Node, SearchError> {
		// TODO: notes; an unbound leaf only makes semantic sense on a structural schema node; were it pointing to a scalar node, it would be equivalent semantically to a bound leaf on that node. following from that; an unbound leaf should "fan out" to all of the current structural node's children as an or-group, in doing so effectively "consuming" the current node at the leaf point, which maintains consistency with bound leaf handling.

		todo!("normalize leaf unbound")
	}

	fn normalize_operation(
		&self,
		operation: &pre::Operation,
		context: Context,
	) -> Result<post::Node, SearchError> {
		match operation {
			// TODO: should this panic if it _isn't_ a 1:1 relation:reference pair?
			//       no, it shouldn't - it could also be a struct... wait, can it?
			//       yeah, the callsite might have drilled into a struct, but this relation forms the basis of the next target, i think
			// so tldr;
			// for relations, if the schema is a reference, resolve the reference. if it's a struct, call down. if it's anything else, throw?
			pre::Operation::Relation(relation) => {
				let node = match context.schema {
					schema::Node::Struct(_fields) => todo!(
						"i think this is passing the entire schema node down to the subquery?"
					),

					schema::Node::Reference(targets) => {
						let field = match context.columns {
							[column] => column,
							other => {
								return Err(SearchError::SchemaGameMismatch(MismatchError {
									field: "TODO: query path".into(),
									reason: format!(
										"cross-sheet references must have a single source (found {})",
										other.len()
									),
								}))
							}
						};

						let target_queries = targets
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
								let query = self.normalize(
									&relation.query,
									&target.sheet,
									context.language,
								)?;

								let operation = post::Operation::Relation(post::Relation {
									target: post::RelationTarget {
										sheet: target.sheet.clone(),
										condition: None, // todo
									},
									query: Box::new(query),
								});

								let node = post::Node::Leaf(post::Leaf {
									field: (field.clone(), context.language),
									operation,
								});

								Ok(node)
							})
							// Filter out query mismatches to prune those branches - other errors will be raised.
							.filter(|result| {
								!matches!(result, Err(SearchError::QuerySchemaMismatch(_)))
							})
							.collect::<Result<Vec<_>, _>>()?;

						// TODO: target_queries.len() == 0 here means none of the relations matched, which should be raised as a query mismatch

						// There might be multiple viable relation paths, group them together.
						create_or_group(target_queries.into_iter())
					}

					other => todo!("i think this is a schema mismatch {other:?}"),
				};

				Ok(node)
			}

			pre::Operation::Match(string) => {
				let scalar_columns = collect_scalars(context.schema, context.columns, vec![])
					.ok_or_else(|| {
						SearchError::SchemaGameMismatch(MismatchError {
							// TODO: i'll need to wire down the current query path for this field to be meaningful
							field: "query".into(),
							reason: "insufficient game data to satisfy schema".into(),
						})
					})?;

				// NOTE: The collect is not actually needless - .filter precludes ExactSizeIterator
				#[allow(clippy::needless_collect)]
				let string_columns = scalar_columns
					.into_iter()
					.filter(|column| column.kind() == exh::ColumnKind::String)
					.collect::<Vec<_>>();

				let group = create_or_group(string_columns.into_iter().map(|column| {
					post::Node::Leaf(post::Leaf {
						field: (column, context.language),
						operation: post::Operation::Match(string.clone()),
					})
				}));

				Ok(group)
			}

			// TODO: this should collect all scalars i think?
			// TODO: this pattern will be pretty repetetive, make a utility that does this or something
			pre::Operation::Equal(value) => {
				let scalar_columns = collect_scalars(context.schema, context.columns, vec![])
					.ok_or_else(|| {
						SearchError::SchemaGameMismatch(MismatchError {
							// TODO: i'll need to wire down the current query path for this field to be meaningful
							field: "query".into(),
							reason: "insufficient game data to satisfy schema".into(),
						})
					})?;

				let group = create_or_group(scalar_columns.into_iter().map(|column| {
					post::Node::Leaf(post::Leaf {
						field: (column, context.language),
						operation: post::Operation::Equal(value.clone()),
					})
				}));

				Ok(group)
			}
		}
	}
}

fn create_or_group(mut nodes: impl ExactSizeIterator<Item = post::Node>) -> post::Node {
	match nodes.len() {
		0 => todo!("what do i do for the zero case? it's possibly a schema mismatch but i'm not sure. consider callsites"),
		1 => nodes.next().unwrap(),
		_ => post::Node::Group(post::Group {
			clauses: nodes.map(|node| (post::Occur::Should, node)).collect(),
		})
	}
}

// The whole premise of this is that we want to _exclude_ references. If that premise does not hold, then the `columns` slice itself is basically exactly what we want.
// TODO: On discussing, people(singular) seemed okay with a field being simultaneously scalar and a reference. I can't say I'm convinced, but it might be fine.
fn collect_scalars(
	schema: &schema::Node,
	columns: &[exh::ColumnDefinition],
	mut output: Vec<exh::ColumnDefinition>,
) -> Option<Vec<exh::ColumnDefinition>> {
	match schema {
		schema::Node::Array { count, node } => {
			// TODO: this is pretty silly, can technically derive the range from 1 call down.
			let size = usize::try_from(node.size()).unwrap();
			let count = usize::try_from(*count).unwrap();
			(0..count).try_fold(output, |output, index| {
				let start = index * size;
				let end = start + size;
				let slice = columns.get(start..end)?;
				collect_scalars(node, slice, output)
			})
		}

		schema::Node::Reference(_references) => {
			// ignore refs?
			Some(output)
		}

		schema::Node::Scalar => {
			output.push(columns.get(0)?.clone());
			Some(output)
		}

		schema::Node::Struct(fields) => fields.iter().try_fold(output, |output, field| {
			let start = usize::try_from(field.offset).unwrap();
			let end = start + usize::try_from(field.node.size()).unwrap();
			let slice = columns.get(start..end)?;
			collect_scalars(&field.node, slice, output)
		}),
	}
}
