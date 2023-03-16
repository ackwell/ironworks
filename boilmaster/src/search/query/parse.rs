use std::str::FromStr;

use ironworks::excel;
use nom::{
	branch::alt,
	bytes::complete::{tag, take_till, take_while1},
	character::complete::{char, digit1, multispace1},
	combinator::{map, map_res, not, opt, success, value as nom_value},
	error::convert_error,
	multi::separated_list1,
	number::complete::double,
	sequence::{delimited, preceded, terminated, tuple},
	Finish,
};
use serde::{de, Deserialize};

use crate::{data, search::SearchError};

use super::pre;

// TODO: confirm
const LANGUAGE_SIGIL: &str = "@";

type IResult<'a, I, O> = nom::IResult<I, O, nom::error::VerboseError<&'a str>>;

impl FromStr for pre::Node {
	type Err = SearchError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		// Root level of a query is an implicit group
		let (remaining, group) = group(input)
			.finish()
			.map_err(|error| SearchError::MalformedQuery(convert_error(input, error)))?;

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
	alt((
		map(delimited(char('('), group, char(')')), pre::Node::Group),
		map(leaf, pre::Node::Leaf),
	))(input)
}

fn group(input: &str) -> IResult<&str, pre::Group> {
	map(
		separated_list1(multispace1, tuple((occur, node))),
		|clauses| pre::Group { clauses },
	)(input)
}

fn occur(input: &str) -> IResult<&str, pre::Occur> {
	alt((
		nom_value(pre::Occur::Must, char('+')),
		nom_value(pre::Occur::MustNot, char('-')),
		success(pre::Occur::Should),
	))(input)
}

fn leaf(input: &str) -> IResult<&str, pre::Leaf> {
	map(
		tuple((opt(field_specifier), operation)),
		|(field, operation)| pre::Leaf { field, operation },
	)(input)
}

fn field_specifier(input: &str) -> IResult<&str, pre::FieldSpecifier> {
	terminated(
		alt((field_specifier_struct, field_specifier_array)),
		opt(char(':')),
	)(input)
}

fn field_specifier_struct(input: &str) -> IResult<&str, pre::FieldSpecifier> {
	map(tuple((alphanumeric, opt(language))), |(name, language)| {
		pre::FieldSpecifier::Struct(name.into(), language.map(excel::Language::from))
	})(input)
}

fn language(input: &str) -> IResult<&str, data::LanguageString> {
	map_res(preceded(tag(LANGUAGE_SIGIL), alphanumeric), str::parse)(input)
}

fn alphanumeric(input: &str) -> IResult<&str, &str> {
	take_while1(|c: char| c.is_ascii_alphanumeric())(input)
}

fn field_specifier_array(input: &str) -> IResult<&str, pre::FieldSpecifier> {
	map(tag("[]"), |_| pre::FieldSpecifier::Array)(input)
}

fn operation(input: &str) -> IResult<&str, pre::Operation> {
	alt((
		map(relation, pre::Operation::Relation),
		map(preceded(char('='), value), pre::Operation::Equal),
		// An un-adorned string acts as a match query. This needs to be last to ensure other sigils take priority.
		map(string, pre::Operation::Match),
	))(input)
}

fn relation(input: &str) -> IResult<&str, pre::Relation> {
	map(preceded(char('.'), node), |node| pre::Relation {
		target: (),
		query: Box::new(node),
	})(input)
}

fn value(input: &str) -> IResult<&str, pre::Value> {
	alt((
		// Try to parse the number as a potentially-signed integer. If it's followed by `.`, it'll fall through to the float check.
		terminated(
			alt((
				map(map_res(digit1, str::parse), pre::Value::U64),
				map(map_res(take_while1(is_signed), str::parse), pre::Value::I64),
			)),
			not(char('.')),
		),
		map(double, pre::Value::F64),
		map(string, pre::Value::String),
	))(input)
}

fn string(input: &str) -> IResult<&str, String> {
	map(
		delimited(
			char('"'),
			take_till(|character| character == '"'),
			char('"'),
		),
		|str: &str| str.to_string(),
	)(input)
}

fn is_signed(char: char) -> bool {
	char.is_ascii_digit() || char == '-'
}
