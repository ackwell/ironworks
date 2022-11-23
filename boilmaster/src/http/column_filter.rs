use std::collections::HashMap;

use nom::{
	bytes::complete::{tag, take_while1},
	multi::separated_list1,
	IResult,
};
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct ColumnFilter {}

impl<'de> Deserialize<'de> for ColumnFilter {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let raw = String::deserialize(deserializer)?;

		let processed = paths(&raw);
		tracing::info!("WIP {processed:#?}");

		Ok(Self {})
	}
}

fn paths(input: &str) -> IResult<&str, Vec<Path>> {
	separated_list1(tag(","), path)(input)
}

type Path<'a> = Vec<Field<'a>>;

fn path(input: &str) -> IResult<&str, Path> {
	separated_list1(tag("."), field)(input)
}

// TODO: lmao
type Field<'a> = &'a str;

fn field(input: &str) -> IResult<&str, Field> {
	// TODO: ascii safe to use here? i'd hope?
	take_while1(|c: char| c.is_ascii_alphanumeric())(input)
}

// ----- thoughts -----
type StructFilterTest = HashMap<String, Option<FilterTest>>;

#[derive(Debug, PartialEq)]
enum FilterTest {
	Struct(HashMap<String, Option<FilterTest>>),

	// can probably flesh out with more info as required
	// due to multiple slices, probably easiest to halt merges at arrays and only start merging again on index access
	Array,
	// do i want seperate syntax for references?
	// Reference
}

impl FilterTest {
	fn merge(self, other: Self) -> Self {
		match (self, other) {
			(Self::Struct(struct_1), Self::Struct(struct_2)) => {
				Self::Struct(merge_struct(struct_1, struct_2))
			}

			(fallback_1, fallback_2) => todo!("unhandled merge {fallback_1:?} <-> {fallback_2:?}"),
		}
	}
}

fn merge_struct(mut target: StructFilterTest, source: StructFilterTest) -> StructFilterTest {
	for (key, value) in source {
		let merged = match target.remove(&key) {
			// The target didn't contain this key yet, use the incoming value
			None => value,
			// We already had this key, perform a merge
			Some(ev) => match (ev, value) {
				// If both sides already had filters for this key, merge recursively
				(Some(target), Some(source)) => Some(target.merge(source)),
				// If either side had None, which acts as an "All" value, propagate the None.
				_ => None,
			},
		};

		target.insert(key, merged);
	}

	target
}

// TODO: tests can use string reading instead of manual construction, probably
#[cfg(test)]
mod test {
	use super::*;

	// a,b -> {a, b}
	#[test]
	fn simple_merge() {
		let a = FilterTest::Struct(HashMap::from([("a".into(), None)]));
		let b = FilterTest::Struct(HashMap::from([("b".into(), None)]));
		let out = a.merge(b);

		let expected = FilterTest::Struct(HashMap::from([("a".into(), None), ("b".into(), None)]));
		assert!(out == expected, "{out:?} == {expected:?}");
	}

	// a,a.b -> {a}
	#[test]
	fn struct_widen() {
		let a = FilterTest::Struct(HashMap::from([("a".into(), None)]));
		let b = FilterTest::Struct(HashMap::from([("b".into(), None)]));
		let ab = FilterTest::Struct(HashMap::from([("a".into(), Some(b))]));
		let out = a.merge(ab);

		let expected = FilterTest::Struct(HashMap::from([("a".into(), None)]));
		assert!(out == expected, "{out:?} == {expected:?}");
	}

	// a.b,a.c -> {a: {b, c}}
	#[test]
	fn nested_merge() {
		let b = FilterTest::Struct(HashMap::from([("b".into(), None)]));
		let ab = FilterTest::Struct(HashMap::from([("a".into(), Some(b))]));
		let c = FilterTest::Struct(HashMap::from([("c".into(), None)]));
		let ac = FilterTest::Struct(HashMap::from([("a".into(), Some(c))]));
		let out = ab.merge(ac);

		let expected = FilterTest::Struct(HashMap::from([(
			"a".into(),
			Some(FilterTest::Struct(HashMap::from([
				("b".into(), None),
				("c".into(), None),
			]))),
		)]));
		assert!(out == expected, "{out:?} == {expected:?}");
	}
}

// "a.b" should be thinged bottom-up so we can create struct, read down, and add the result as a
