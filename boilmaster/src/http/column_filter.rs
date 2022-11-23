use std::collections::HashMap;

use nom::{
	branch::alt,
	bytes::complete::{tag, take_while1},
	combinator::{map, opt},
	multi::separated_list1,
	sequence::{preceded, tuple},
	IResult,
};
use serde::{Deserialize, Deserializer};

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

impl<'de> Deserialize<'de> for FilterTest {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let raw = String::deserialize(deserializer)?;

		let processed = group(&raw);
		tracing::info!("WIP {processed:#?}");

		Ok(Self::Array)
	}
}

fn group(input: &str) -> IResult<&str, FilterTest> {
	map(
		separated_list1(tag(","), filter),
		// Unwrap is safe here, as `reduce` only returns `None` on 0-entry iterators, and `separated_list1` guarantees >=1 entries.
		|filters| filters.into_iter().reduce(|a, b| a.merge(b)).unwrap(),
	)(input)
}

fn filter(input: &str) -> IResult<&str, FilterTest> {
	alt((struct_entry,))(input)
}

fn struct_entry(input: &str) -> IResult<&str, FilterTest> {
	map(
		tuple((field_name, opt(preceded(tag("."), group)))),
		|(key, child)| FilterTest::Struct(HashMap::from([(key.into(), child)])),
	)(input)
}

fn field_name(input: &str) -> IResult<&str, &str> {
	// TODO: ascii safe to use here? i'd hope?
	take_while1(|c: char| c.is_ascii_alphanumeric())(input)
}

// TODO: tests can use string reading instead of manual construction, probably
#[cfg(test)]
mod test {
	use super::*;

	fn test_parse(input: &str) -> FilterTest {
		let (remaining, output) = group(input).expect("parse should not fail");
		assert_eq!(remaining, "");
		output
	}

	#[test]
	fn parse_simple() {
		let out = test_parse("a");

		let expected = FilterTest::Struct(HashMap::from([("a".into(), None)]));
		assert_eq!(out, expected);
	}

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
