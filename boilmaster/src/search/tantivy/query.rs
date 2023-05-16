use std::sync::Arc;

use tantivy::{
	fastfield::{AliveBitSet, Column},
	query::{EnableScoring, Explanation, Query, RegexQuery, Scorer, Weight},
	schema::Field,
	DocId, DocSet, Score, SegmentReader, TantivyError,
};

use crate::search::{error::Result, Error};

#[derive(Debug)]
pub struct MatchQuery {
	query: RegexQuery,
	field: Field,
	target: u64,
}

impl MatchQuery {
	pub fn new(match_string: &str, field_string: Field, field_length: Field) -> Result<Self> {
		// String columns are ingested untokenised, so we can run "matches" using a regex partial match.
		// TODO: consider allowing ^$ (impl by removing leading/trailing .*) and * (repl. with .*)
		// TODO: for the above, consider how the above is scored - should they be trimmed? we can't expand the * to the matched length
		// TODO: What behavior should an empty string perform?
		let pattern = format!("(?i).*{}.*", regex_syntax::escape(match_string));
		let query =
			RegexQuery::from_pattern(&pattern, field_string).map_err(|error| match error {
				TantivyError::InvalidArgument(_) => {
					Error::MalformedQuery(format!("invalid match string \"{match_string}\""))
				}
				other => Error::Failure(other.into()),
			})?;

		let target = u64::try_from(match_string.len()).unwrap();

		Ok(Self {
			query,
			field: field_length,
			target,
		})
	}
}

impl Clone for MatchQuery {
	fn clone(&self) -> Self {
		Self {
			query: self.query.clone(),
			field: self.field,
			target: self.target,
		}
	}
}

impl Query for MatchQuery {
	fn weight(&self, enable_scoring: EnableScoring<'_>) -> tantivy::Result<Box<dyn Weight>> {
		// The weight/scorer implementations solely adjust result scores, so they
		// can be completely skipped if scoring is disabled.
		let weight = self.query.weight(enable_scoring)?;
		if !enable_scoring.is_scoring_enabled() {
			return Ok(weight);
		}

		Ok(Box::new(MatchWeight {
			weight,
			field: self.field,
			target: self.target,
		}))
	}
}

struct MatchWeight {
	weight: Box<dyn Weight>,
	field: Field,
	target: u64,
}

impl Weight for MatchWeight {
	fn scorer(&self, reader: &SegmentReader, boost: Score) -> tantivy::Result<Box<dyn Scorer>> {
		let length_reader = reader.fast_fields().u64(self.field).unwrap();

		Ok(Box::new(MatchScorer {
			scorer: self.weight.scorer(reader, boost)?,
			length_reader,
			target: self.target,
		}))
	}

	fn explain(&self, _reader: &SegmentReader, _doc: DocId) -> tantivy::Result<Explanation> {
		todo!()
	}
}

struct MatchScorer<S> {
	scorer: S,
	length_reader: Arc<dyn Column<u64>>,
	target: u64,
}

impl<S> Scorer for MatchScorer<S>
where
	S: Scorer,
{
	fn score(&mut self) -> Score {
		let score = self.scorer.score();

		let length = self.length_reader.get_val(self.doc());
		let boost = self.target as f32 / length as f32;

		score * boost
	}
}

impl<S> DocSet for MatchScorer<S>
where
	S: Scorer,
{
	fn advance(&mut self) -> DocId {
		self.scorer.advance()
	}

	fn seek(&mut self, target: DocId) -> DocId {
		self.scorer.seek(target)
	}

	fn fill_buffer(&mut self, buffer: &mut [DocId]) -> usize {
		self.scorer.fill_buffer(buffer)
	}

	fn doc(&self) -> DocId {
		self.scorer.doc()
	}

	fn size_hint(&self) -> u32 {
		self.scorer.size_hint()
	}

	fn count(&mut self, alive_bitset: &AliveBitSet) -> u32 {
		self.scorer.count(alive_bitset)
	}

	fn count_including_deleted(&mut self) -> u32 {
		self.scorer.count_including_deleted()
	}
}
