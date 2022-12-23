use std::{collections::HashMap, fmt};

use nom::{
	branch::alt,
	bytes::complete::{tag, take_while1},
	combinator::{map, opt},
	multi::separated_list1,
	sequence::{delimited, preceded, tuple},
	Finish, IResult,
};
use serde::{de, Deserialize, Deserializer};

use crate::utility::warnings::{SoftDeserialize, Warnings};

// TODO: should this be in a top level filter module? will depend if there's other types of filters i guess. also semantics...
//       might make sense as read::Filter to go alongside i.e. search::Filter

type StructFilter = HashMap<String, Option<FieldFilter>>;
type ArrayFilter = Option<Box<FieldFilter>>;

#[derive(Debug, PartialEq)]
pub enum FieldFilter {
	Struct(StructFilter),

	// due to multiple slices, probably easiest to halt merges at arrays and only start merging again on index access
	Array(ArrayFilter),
	// do i want seperate syntax for references?
	// Reference
}

impl fmt::Display for FieldFilter {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Struct(fields) => {
				formatter.write_str("{")?;
				let fields = fields
					.iter()
					.map(|(key, value)| match value {
						Some(filter) => format!("{key}: {filter}"),
						None => key.to_string(),
					})
					.reduce(|left, right| format!("{left}, {right}"))
					.unwrap_or_default();
				formatter.write_str(&fields)?;
				formatter.write_str("}")?;
			}

			Self::Array(inner) => {
				formatter.write_str("[]")?;
				if let Some(filter) = inner {
					formatter.write_fmt(format_args!(".{filter}"))?;
				}
			}
		}
		Ok(())
	}
}

impl<'de> SoftDeserialize<'de> for Option<FieldFilter> {
	fn deserialize<D>(deserializer: D) -> Result<Warnings<Self>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let raw = String::deserialize(deserializer)?;

		let (remaining, filter) = group(&raw)
			.finish()
			.map_err(|error| de::Error::custom(format!("field filter parse error: {error}")))?;

		if !remaining.is_empty() {
			return Err(de::Error::custom(format!(
				"field filter parse error: trailing characters found {remaining:?}"
			)));
		}

		Ok(filter)
	}
}

impl FieldFilter {
	fn merge(self, source: Self) -> Warnings<Option<Self>> {
		match (self, source) {
			(Self::Struct(target_struct), Self::Struct(source_struct)) => {
				merge_struct(target_struct, source_struct)
					.map(|struct_filter| Some(Self::Struct(struct_filter)))
			}

			(Self::Array(target_array), Self::Array(source_array)) => {
				merge_array(target_array, source_array)
					.map(|array_filter| Some(Self::Array(array_filter)))
			}

			// TODO: this will need a "path", i think?
			(fallback_1, fallback_2) => Warnings::new(None).with_warning(format!(
				"filters `{fallback_1}` and `{fallback_2}` cannot be merged, and have been ignored"
			)),
		}
	}
}

fn merge_struct(target: StructFilter, source: StructFilter) -> Warnings<StructFilter> {
	// Fold entries in the source into the target, collecting warnings along the way.
	source.into_iter().fold(
		Warnings::new(target),
		|target, (source_key, source_maybe_filter)| {
			target.and_then(|target_map| {
				merge_struct_field(target_map, source_key, source_maybe_filter)
			})
		},
	)
}

fn merge_struct_field(
	mut target: StructFilter,
	key: String,
	source_value: Option<FieldFilter>,
) -> Warnings<StructFilter> {
	// This uses remove->insert rather than the entry API, as merging requires an owned value, not a mutable reference
	// TODO: is it worth trying to refactor all this to actually use mutable references? I'm not convinced...
	let new_child = match target.remove(&key) {
		// Target didn't contain the key at all, so source value can go straight in.
		None => Warnings::new(source_value),

		// There's a collision, we might need to merge.
		Some(target_maybe_filter) => merge_optional_filters(target_maybe_filter, source_value),
	};

	new_child.map(|child| {
		target.insert(key, child);
		target
	})
}

fn merge_array(left: ArrayFilter, right: ArrayFilter) -> Warnings<ArrayFilter> {
	merge_optional_filters(left.map(|x| *x), right.map(|x| *x)).map(|output| output.map(Box::new))
}

fn group(input: &str) -> IResult<&str, Warnings<Option<FieldFilter>>> {
	map(
		separated_list1(tag(","), filter),
		// .reduce only returns None when there was 0 inputs, which is impossible due to _list1
		|filters| filters.into_iter().reduce(merge_warning_filters).unwrap(),
	)(input)
}

fn merge_warning_filters(
	left: Warnings<Option<FieldFilter>>,
	right: Warnings<Option<FieldFilter>>,
) -> Warnings<Option<FieldFilter>> {
	// Step into the left and right warnings to prep merging them.
	left.and_then(|maybe_filter_left| {
		right.and_then(|maybe_filter_right| {
			merge_optional_filters(maybe_filter_left, maybe_filter_right)
		})
	})
}

fn merge_optional_filters(
	left: Option<FieldFilter>,
	right: Option<FieldFilter>,
) -> Warnings<Option<FieldFilter>> {
	match (left, right) {
		// If both sides have an active filter, merge them and lift any warnings.
		(Some(filter_left), Some(filter_right)) => filter_left.merge(filter_right),
		// Otherwise, a None filter in a group should clear the group.
		(other_left, other_right) => Warnings::new(None).with_warning(format!(
			"filter `{}` ignored as another branch selected all values",
			other_left.or(other_right).unwrap()
		)),
	}
}

fn filter(input: &str) -> IResult<&str, Warnings<Option<FieldFilter>>> {
	alt((
		map(alt((struct_entry, array_index)), |filter| filter.map(Some)),
		delimited(tag("("), group, tag(")")),
	))(input)
}

fn chained_filter(input: &str) -> IResult<&str, Warnings<Option<FieldFilter>>> {
	map(opt(preceded(tag("."), filter)), |filter| {
		filter.unwrap_or_else(|| Warnings::new(None))
	})(input)
}

fn struct_entry(input: &str) -> IResult<&str, Warnings<FieldFilter>> {
	map(tuple((field_name, chained_filter)), |(key, child)| {
		child.map(|filter| FieldFilter::Struct(HashMap::from([(key.into(), filter)])))
	})(input)
}

fn field_name(input: &str) -> IResult<&str, &str> {
	// TODO: ascii safe to use here? i'd hope?
	take_while1(|c: char| c.is_ascii_alphanumeric())(input)
}

fn array_index(input: &str) -> IResult<&str, Warnings<FieldFilter>> {
	map(
		tuple((tag("[]"), chained_filter)),
		// TODO: actually parse an index
		|(_, child)| child.map(|filter| FieldFilter::Array(filter.map(Box::new))),
	)(input)
}

// TODO: need to add tests for error paths - and at that, add error handling. a lot of error cases (like mismatched types on a merge) can soft fail, but i should still surface warnings that they did soft fail. need to work out how that would work
#[cfg(test)]
mod test {
	use super::*;

	fn test_parse(input: &str) -> Option<FieldFilter> {
		let (value, warnings) = test_warning_parse(input).decompose();
		assert!(warnings.is_empty(), "unexpected warnings: {:?}", warnings);
		value
	}

	fn test_warning_parse(input: &str) -> Warnings<Option<FieldFilter>> {
		let (remaining, output) = group(input).finish().expect("parse should not fail");
		assert_eq!(remaining, "");
		output
	}

	fn struct_filter(
		entries: impl IntoIterator<Item = (&'static str, Option<FieldFilter>)>,
	) -> FieldFilter {
		let map = entries
			.into_iter()
			.map(|(key, value)| (key.to_string(), value))
			.collect::<HashMap<_, _>>();
		FieldFilter::Struct(map)
	}

	fn array_filter(child: Option<FieldFilter>) -> FieldFilter {
		FieldFilter::Array(child.map(Box::new))
	}

	#[test]
	fn parse_struct_simple() {
		let out = test_parse("a");
		let expected = Some(struct_filter([("a", None)]));
		assert_eq!(out, expected);
	}

	#[test]
	fn parse_struct_nested() {
		let out = test_parse("a.b");
		let expected = Some(struct_filter([("a", Some(struct_filter([("b", None)])))]));
		assert_eq!(out, expected);
	}

	#[test]
	fn parse_array_simple() {
		let out = test_parse("[]");
		let expected = Some(FieldFilter::Array(None));
		assert_eq!(out, expected);
	}

	#[test]
	fn parse_array_nested() {
		let out = test_parse("a.[].[].b");
		let expected = Some(struct_filter([(
			"a",
			Some(array_filter(Some(array_filter(Some(struct_filter([(
				"b", None,
			)])))))),
		)]));
		assert_eq!(out, expected);
	}

	#[test]
	fn merge_fail() {
		let (value, warnings) = test_warning_parse("a,[]").decompose();
		assert_eq!(value, None);
		assert_eq!(
			warnings,
			vec![String::from(
				"filters `{a}` and `[]` cannot be merged, and have been ignored"
			)]
		);
	}

	// a.[],a.b -> {a}
	#[test]
	fn merge_nested_fail() {
		let (value, warnings) = test_warning_parse("a.[],a.b").decompose();
		let expected = Some(struct_filter([("a", None)]));
		assert_eq!(value, expected);
		assert_eq!(
			warnings,
			vec![String::from(
				"filters `[]` and `{b}` cannot be merged, and have been ignored"
			)]
		);
	}

	// a,b -> {a, b}
	#[test]
	fn merge_struct_simple() {
		let out = test_parse("a,b");
		let expected = Some(struct_filter([("a", None), ("b", None)]));
		assert_eq!(out, expected);
	}

	// a,a.b -> {a}
	#[test]
	fn merge_struct_widen() {
		let (value, warnings) = test_warning_parse("a,a.b").decompose();
		let expected = Some(struct_filter([("a", None)]));
		assert_eq!(value, expected);
		assert_eq!(
			warnings,
			vec![String::from(
				"filter `{b}` ignored as another branch selected all values"
			)]
		);
	}

	// a.b,a.c -> {a: {b, c}}
	#[test]
	fn merge_struct_nested() {
		let out = test_parse("a.b,a.c");
		let expected = Some(struct_filter([(
			"a",
			Some(struct_filter([("b", None), ("c", None)])),
		)]));
		assert_eq!(out, expected);
	}

	// a.(b,c),a.d -> {a: {b, c, d}}
	#[test]
	fn merge_nested_group() {
		let out = test_parse("a.(b,c),a.d");
		let expected = Some(struct_filter([(
			"a",
			Some(struct_filter([("b", None), ("c", None), ("d", None)])),
		)]));
		assert_eq!(out, expected);
	}

	// [].a,[].b -> [{a, b}]
	#[test]
	fn merge_array_children() {
		let out = test_parse("[].a,[].b");
		let expected = Some(array_filter(Some(struct_filter([
			("a", None),
			("b", None),
		]))));
		assert_eq!(out, expected);
	}
}
