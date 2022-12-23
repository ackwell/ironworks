use std::str::FromStr;

use nom::{
	branch::alt,
	bytes::complete::{tag, take_while1},
	character::complete::{digit1, multispace1},
	combinator::{map, map_res, opt, success, value as nom_value},
	multi::separated_list1,
	sequence::{delimited, preceded, tuple},
	Finish, IResult,
};
use serde::{de, Deserialize};

use crate::search::SearchError;

use super::pre;

impl FromStr for pre::Node {
	type Err = SearchError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// Root level of a query is an implicit group
		let (remaining, group) = group(input)
			.finish()
			.map_err(|error| SearchError::MalformedQuery(error.to_string()))?;

		if !remaining.is_empty() {
			return Err(SearchError::MalformedQuery(format!(
				"unexpected trailing characters {remaining:?}"
			)));
		}

		Ok(pre::Node::Group(group))
	}
}

impl<'de> Deserialize<'de> for pre::Node {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let raw = String::deserialize(deserializer)?;
		raw.parse().map_err(de::Error::custom)
	}
}

fn node(input: &str) -> IResult<&str, pre::Node> {
	tracing::debug!("node {input:?}");
	alt((
		map(delimited(tag("("), group, tag(")")), pre::Node::Group),
		map(leaf, pre::Node::Leaf),
	))(input)
}

fn group(input: &str) -> IResult<&str, pre::Group> {
	tracing::debug!("group {input:?}");
	map(
		separated_list1(multispace1, tuple((occur, node))),
		|clauses| pre::Group { clauses },
	)(input)
}

fn occur(input: &str) -> IResult<&str, pre::Occur> {
	tracing::debug!("occur {input:?}");
	alt((
		nom_value(pre::Occur::Must, tag("+")),
		nom_value(pre::Occur::MustNot, tag("-")),
		success(pre::Occur::Should),
	))(input)
}

fn leaf(input: &str) -> IResult<&str, pre::Leaf> {
	tracing::debug!("leaf {input:?}");
	map(
		tuple((opt(field_specifier), operation)),
		|(field, operation)| pre::Leaf { field, operation },
	)(input)
}

fn field_specifier(input: &str) -> IResult<&str, pre::FieldSpecifier> {
	tracing::debug!("field_specifier {input:?}");
	alt((field_specifier_struct, field_specifier_array))(input)
}

fn field_specifier_struct(input: &str) -> IResult<&str, pre::FieldSpecifier> {
	tracing::debug!("field_specifier_struct {input:?}");
	map(
		take_while1(|c: char| c.is_ascii_alphanumeric()),
		|name: &str| pre::FieldSpecifier::Struct(name.into()),
	)(input)
}

fn field_specifier_array(input: &str) -> IResult<&str, pre::FieldSpecifier> {
	tracing::debug!("field_specifier_array {input:?}");
	map(tag("[]"), |_| pre::FieldSpecifier::Array)(input)
}

fn operation(input: &str) -> IResult<&str, pre::Operation> {
	tracing::debug!("operation {input:?}");
	alt((
		map(relation, pre::Operation::Relation),
		map(preceded(tag("="), value), pre::Operation::Equal),
	))(input)
}

fn relation(input: &str) -> IResult<&str, pre::Relation> {
	tracing::debug!("relation {input:?}");
	map(preceded(tag("."), node), |node| pre::Relation {
		target: (),
		query: Box::new(node),
	})(input)
}

fn value(input: &str) -> IResult<&str, pre::Value> {
	tracing::debug!("value {input:?}");
	map(map_res(digit1, str::parse), |value: u64| {
		pre::Value::U64(value)
	})(input)
}
