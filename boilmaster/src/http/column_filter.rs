use std::{collections::HashMap, ops::Range};

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
	// TODO: &mut self or self->self??????
	fn merge(&mut self, other: Self) {
		match (self, other) {
			(Self::Struct(struct_1), Self::Struct(struct_2)) => merge_struct(struct_1, struct_2),

			(fallback_1, fallback_2) => todo!("unhandled merge {fallback_1:?} <-> {fallback_2:?}"),
		}
	}
}

fn merge_struct(target: &mut StructFilterTest, source: StructFilterTest) {
	for (key, value) in source {
		use std::collections::hash_map::Entry;

		match target.entry(key) {
			Entry::Vacant(entry) => {
				entry.insert(value);
			}

			Entry::Occupied(mut entry) => {
				match (entry.get_mut(), value) {
					(Some(target), Some(source)) => target.merge(source),
					_ => todo!("any none propagates"),
				};
			}
		};
	}

	// let a = target.into_iter().chain(source.into_iter()).fold(
	// 	HashMap::new(),
	// 	|mut map, (key, value)| {
	// 		map.entry(key)
	// 			.and_modify(|ev| match (value, ev) {
	// 				(Some(a), Some(b)) => todo!("call merge"),
	// 				_ => todo!("any none propagates a none"),
	// 			})
	// 			.or_insert(value);
	// 		map
	// 	},
	// );

	// a

	// TODO: is mutation okay? i mean we own it, so...
	// target.extend(other);
	// target
}

// TODO: tests can use string reading instead of manual construction, probably
#[cfg(test)]
mod test {

	use super::*;

	// a,b -> {a, b}
	#[test]
	fn simple_merge() {
		let mut a = FilterTest::Struct(HashMap::from([("a".into(), None)]));
		let b = FilterTest::Struct(HashMap::from([("b".into(), None)]));
		let out = a.merge(b);
		let out = a;

		let expected = FilterTest::Struct(HashMap::from([("a".into(), None), ("b".into(), None)]));
		assert!(out == expected, "{out:?} == {expected:?}");
	}

	// a,a.c -> {a}
	#[test]
	fn struct_widen() {
		todo!()
	}

	// a.b,a.c -> {a: {b, c}}
	#[test]
	fn nested_merge() {
		let b = FilterTest::Struct(HashMap::from([("b".into(), None)]));
		let mut ab = FilterTest::Struct(HashMap::from([("a".into(), Some(b))]));
		let c = FilterTest::Struct(HashMap::from([("b".into(), None)]));
		let ac = FilterTest::Struct(HashMap::from([("a".into(), Some(c))]));
		let out = ab.merge(ac);
		let out = ab;

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
