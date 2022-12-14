use std::path::PathBuf;

use anyhow::Result;
use ironworks::excel::Sheet;
use tantivy::{
	collector::TopDocs,
	directory::MmapDirectory,
	query::{BooleanQuery, Query, TermQuery},
	schema::{Field, FieldType, IndexRecordOption, Schema},
	ReloadPolicy, Term,
};

use super::{
	ingest::Ingester,
	query::{Clause, Leaf, Node, Operation, Value},
};

#[derive(Debug)]
pub struct IndexResult {
	pub score: f32,
	pub row_id: u32,
	pub subrow_id: u16,
}

pub struct Index {
	// Do i actually need a reference to the index at all?
	index: tantivy::Index,
	reader: tantivy::IndexReader,
}

impl Index {
	// TODO: creating a new index requires a schema, which in turn requires columns, which requires an .exh. For now, I'm keeping the creation bundled to avoid that read - consider how it might be done to split new/ingest
	pub async fn ingest(
		ingester: &Ingester,
		path: PathBuf,
		sheet: Sheet<'static, String>,
	) -> Result<Self> {
		tokio::fs::create_dir_all(&path).await?;
		let directory = MmapDirectory::open(path)?;

		let index = match tantivy::Index::exists(&directory)? {
			true => tantivy::Index::open(directory)?,
			// TODO: this should do... something. retry? i don't know. if any step of ingestion fails. A failed ingest is pretty bad.
			// TODO: i don't think an index existing actually means ingestion was successful - i should probably split the index creation out of ingest_sheet, and then put ingestion as a seperate step in this function as part of a document count check
			false => ingester
				.ingest_sheet(sheet, directory)
				.await
				.expect("TODO: error handling for ingestion failures"),
		};

		let reader = index
			.reader_builder()
			// TODO: this is set to manual 'cus technically an index is never updated in this setup. is that sane? i dunno?
			.reload_policy(ReloadPolicy::Manual)
			.try_into()?;

		Ok(Self { index, reader })
	}

	// TODO: probably need some form of typedef for the id pair - where does that live? should it be a struct?
	pub fn search(
		&self,
		// query_string: &str,
		query_node: &Node,
	) -> Result<impl Iterator<Item = IndexResult>> {
		let searcher = self.reader.searcher();

		let schema = searcher.schema();

		let query_resolver = QueryResolver { schema };
		let query = query_resolver.resolve(query_node);

		// // so immediate complication to deal with; need to specify the fields to search if the user (lmao as if) doesn't specify any. we... techncially want to search _every_thing. or, at least every string thing? idfk. all strings makes sense i guess?
		// // TODO: this should probably be precomputed
		// let string_fields = schema
		// 	.fields()
		// 	.filter(|(_field, entry)| matches!(entry.field_type(), FieldType::Str(_)))
		// 	.map(|(field, _entry)| field)
		// 	.collect::<Vec<_>>();

		// // No string fields means a query string will never match (it'll throw an error, even).
		// // TODO: this condition does not hold when further filters are added
		// if string_fields.is_empty() {
		// 	return Ok(Either::Right(std::iter::empty()));
		// }

		// TODO: these string constants should be in a shared location.
		let row_id_field = schema
			.get_field("row_id")
			.expect("row_id field is specified on all indices");
		let subrow_id_field = schema
			.get_field("subrow_id")
			.expect("subrow_id field is specified on all indices");

		// let query_parser = QueryParser::for_index(&self.index, string_fields);
		// let query = query_parser.parse_query(query_string)?;

		// TODO: in tantivy 0.19 i can throw a const scorer on this to make it worth nothing or something? i imagine the actual strings are the important bits
		// let query = BooleanQuery::new(b.collect());

		// TODO: this results in each individuial index having a limit, as opposed to the whole query itself - think about how to approach this.
		let top_docs = searcher.search(&query, &TopDocs::with_limit(100))?;
		let todo_result = top_docs.into_iter().map(move |(score, doc_address)| {
			let doc = searcher.doc(doc_address).expect(
				"TODO: error handling. is there any reasonable expectation this will fail?",
			);
			// TODO: error handling; this is frankly disgusting
			let row_id = doc
				.get_first(row_id_field)
				.unwrap()
				.as_u64()
				.unwrap()
				.try_into()
				.unwrap();
			let subrow_id = doc
				.get_first(subrow_id_field)
				.unwrap()
				.as_u64()
				.unwrap()
				.try_into()
				.unwrap();

			IndexResult {
				score,
				row_id,
				subrow_id,
			}
		});

		// Ok(Either::Left(todo_result))
		Ok(todo_result)
	}
}

struct QueryResolver<'a> {
	schema: &'a Schema,
}

impl QueryResolver<'_> {
	fn resolve(&self, node: &Node) -> Box<dyn Query> {
		match node {
			Node::Clause(clause) => self.resolve_clause(clause),
			Node::Leaf(leaf) => self.resolve_leaf(leaf),
		}
	}

	fn resolve_clause(&self, clause: &Clause) -> Box<dyn Query> {
		let subqueries = clause.nodes.iter().map(|(occur, node)| {
			use super::query::Occur as BOccur;
			use tantivy::query::Occur as TOccur;
			let tantivy_occur = match occur {
				BOccur::Must => TOccur::Must,
				BOccur::Should => TOccur::Should,
				BOccur::MustNot => TOccur::MustNot,
			};

			(tantivy_occur, self.resolve(node))
		});

		Box::new(BooleanQuery::new(subqueries.collect()))
	}

	fn resolve_leaf(&self, leaf: &Leaf) -> Box<dyn Query> {
		// TODO: this should use a schema-provided name fetcher or something, this is not stable
		let field = self
			.schema
			.get_field(&leaf.offset.to_string())
			.expect("this should probably be a warning of some kind");

		match &leaf.operation {
			Operation::Relation(_) => todo!(),
			Operation::Equal(value) => {
				let term = self.value_to_term(value, field);
				Box::new(TermQuery::new(term, IndexRecordOption::Basic))
			}
		}
	}

	fn value_to_term(&self, value: &Value, field: Field) -> Term {
		let field_entry = self.schema.get_field_entry(field);
		let field_type = field_entry.field_type();

		match field_type {
			FieldType::U64(_) => Term::from_field_u64(field, self.value_to_u64(value)),
			FieldType::I64(_) => Term::from_field_i64(field, self.value_to_i64(value)),
			other => todo!("{other:#?}"),
		}
	}

	fn value_to_u64(&self, value: &Value) -> u64 {
		match value {
			Value::UInt(inner) => *inner,
		}
	}

	fn value_to_i64(&self, value: &Value) -> i64 {
		match value {
			Value::UInt(inner) => (*inner).try_into().expect("TODO: this should also be a warning. i need a general purpose warning thing for invalid field types or something"),
		}
	}
}
