use tantivy::{
	query::{BooleanQuery, PhraseQuery, Query, TermQuery, TermSetQuery},
	schema::{Field, IndexRecordOption, Schema, Type},
	Index, TantivyError, Term,
};

use crate::search::{
	error::{FieldTypeError, MismatchError, SearchError},
	query::post::{Group, Leaf, Node, Operation, Relation, Value},
	version::Executor,
};

use super::schema::column_field_name;

pub struct QueryResolver<'a> {
	pub index: &'a Index,
	pub schema: &'a Schema,
	pub executor: &'a Executor,
}

impl QueryResolver<'_> {
	pub fn resolve(&self, node: &Node) -> Result<Box<dyn Query>, SearchError> {
		match node {
			Node::Group(group) => self.resolve_clause(group),
			Node::Leaf(leaf) => self.resolve_leaf(leaf),
		}
	}

	fn resolve_clause(&self, group: &Group) -> Result<Box<dyn Query>, SearchError> {
		let subqueries = group
			.clauses
			.iter()
			.map(|(occur, node)| {
				use crate::search::query::post::Occur as BOccur;
				use tantivy::query::Occur as TOccur;
				let tantivy_occur = match occur {
					BOccur::Must => TOccur::Must,
					BOccur::Should => TOccur::Should,
					BOccur::MustNot => TOccur::MustNot,
				};

				Ok((tantivy_occur, self.resolve(node)?))
			})
			.collect::<Result<Vec<_>, SearchError>>()?;

		Ok(Box::new(BooleanQuery::new(subqueries)))
	}

	fn resolve_leaf(&self, leaf: &Leaf) -> Result<Box<dyn Query>, SearchError> {
		let field_name = column_field_name(&leaf.field, Language::None);
		let field = self.schema.get_field(&field_name).ok_or_else(|| {
			SearchError::SchemaMismatch(MismatchError {
				// TODO: this will be pretty cryptic to end-users, try to resolve to the schema column name?
				field: format!("field {field_name}"),
				reason: "field does not exist in search index".into(),
			})
		})?;

		match &leaf.operation {
			Operation::Relation(relation) => self.resolve_relation(relation, field),
			Operation::Match(string) => self.resolve_match(string, field),
			Operation::Equal(value) => {
				if matches!(value, Value::String(_)) {
					// TODO: term/phrase queries inherently don't handle exact equality. I might be able to handle strings as a ^regex$? will need to test.
					todo!("exact string equal handling")
				}
				// TODO: requirements for floats are pretty tight - should I translate float equality into a range around the epsilon or something, or leave that up to consumers to do?
				let term = self.value_to_term(value, field)?;
				Ok(Box::new(TermQuery::new(term, IndexRecordOption::Basic)))
			}
		}
	}

	fn resolve_relation(
		&self,
		relation: &Relation,
		field: Field,
	) -> Result<Box<dyn Query>, SearchError> {
		// Run the inner query on the target index.
		let results = self
			.executor
			.search(&relation.target.sheet, &relation.query)?;

		// Map the results to terms for the query we're building.
		// TODO: I'm ignoring the subrow here - is that sane? AFAIK subrow relations act as a pivot table, many:many - I don't _think_ it references the subrow anywhere?
		// TODO: I have access to a score from the inside here. I should propagate that, somehow.
		let terms = results
			.map(|result| self.value_to_term(&Value::U64(result.row_id.into()), field))
			.collect::<Result<Vec<_>, _>>()?;

		if relation.target.condition.is_some() {
			todo!("handle relationship conditions")
		}

		Ok(Box::new(TermSetQuery::new(terms)))
	}

	fn resolve_match(&self, string: &str, field: Field) -> Result<Box<dyn Query>, SearchError> {
		// Get the analyser for this field from the tokeniser manager.
		let analyzer = self
			.index
			.tokenizer_for_field(field)
			.map_err(|error| match error {
				// Schema errors designate that the field isn't a text field, which is a consumer issue.
				// TODO: the wording on this error leaves a lot to be desired in this particular case.
				TantivyError::SchemaError(message) => SearchError::FieldType(FieldTypeError {
					field: format!("TODO: field {}", self.schema.get_field_name(field)),
					expected: "string field".into(),
					got: message,
				}),

				other => SearchError::Failure(other.into()),
			})?;

		// Use the analyser to break the match string up into terms
		let mut terms = vec![];
		analyzer.token_stream(string).process(&mut |token| {
			let term = Term::from_field_text(field, &token.text);
			terms.push((token.position, term))
		});

		// If there's only one term, use a term query, otherwise a phrase query
		match terms.len() {
			0 => {
				todo!("i'm not sure what a 0-term phrase query would mean. input: {string}")
			}
			1 => Ok(Box::new(TermQuery::new(
				terms.swap_remove(0).1,
				IndexRecordOption::Basic,
			))),
			_ => Ok(Box::new(PhraseQuery::new_with_offset(terms))),
		}
	}

	fn value_to_term(&self, value: &Value, field: Field) -> Result<Term, SearchError> {
		let field_entry = self.schema.get_field_entry(field);
		let field_type = field_entry.field_type().value_type();

		(|| -> Option<_> {
			Some(match field_type {
				Type::Str => Term::from_field_text(field, self.value_to_str(value)?),
				Type::U64 => Term::from_field_u64(field, self.value_to_u64(value)?),
				Type::I64 => Term::from_field_i64(field, self.value_to_i64(value)?),
				Type::F64 => Term::from_field_f64(field, self.value_to_f64(value)?),
				other => todo!("{other:#?}"),
			})
		})()
		.ok_or_else(|| {
			SearchError::FieldType(FieldTypeError {
				// TODO: this will be pretty cryptic to end-users, try to resolve to the schema column name?
				field: format!("field {}", self.schema.get_field_name(field)),
				expected: field_type.name().to_string(),
				got: format!("{value:?}"),
			})
		})
	}

	fn value_to_str<'a>(&self, value: &'a Value) -> Option<&'a str> {
		// Only string values can be reasonably treated as actual strings.
		match value {
			Value::String(value) => Some(value),
			_ => None,
		}
	}

	fn value_to_u64(&self, value: &Value) -> Option<u64> {
		match value {
			Value::U64(inner) => Some(*inner),
			Value::I64(inner) => (*inner).try_into().ok(),
			Value::F64(inner) => {
				let rounded = inner.round();
				if rounded != *inner {
					return None;
				}
				Some(rounded as u64)
			}
			Value::String(_) => None,
		}
	}

	fn value_to_i64(&self, value: &Value) -> Option<i64> {
		match value {
			Value::U64(inner) => (*inner).try_into().ok(),
			Value::I64(inner) => Some(*inner),
			Value::F64(inner) => {
				let rounded = inner.round();
				if rounded != *inner {
					return None;
				}
				Some(rounded as i64)
			}
			Value::String(_) => None,
		}
	}

	fn value_to_f64(&self, value: &Value) -> Option<f64> {
		match value {
			Value::U64(inner) => Some(*inner as f64),
			Value::I64(inner) => Some(*inner as f64),
			Value::F64(inner) => Some(*inner),
			Value::String(_) => None,
		}
	}
}
