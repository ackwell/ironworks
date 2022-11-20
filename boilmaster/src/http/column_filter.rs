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

		let processed = columns(&raw);
		tracing::info!("WIP {processed:#?}");

		Ok(Self {})
	}
}

fn columns(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
	separated_list1(tag(","), column)(input)
}

fn column(input: &str) -> IResult<&str, Vec<&str>> {
	separated_list1(tag("."), field)(input)
}

fn field(input: &str) -> IResult<&str, &str> {
	// TODO: ascii safe to use here? i'd hope?
	take_while1(|c: char| c.is_ascii_alphanumeric())(input)
}
