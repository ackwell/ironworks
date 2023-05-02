use crate::{
	error::Result,
	sestring::{context::Context, expression::Expression, value::ArgumentExt},
};

use super::payload::Payload;

pub struct TitleFirst;
impl Payload for TitleFirst {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let input = arguments.resolve::<String>(context)?;
		Ok(title(&input))
	}
}

pub struct TitleAll;
impl Payload for TitleAll {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let input = arguments.resolve::<String>(context)?;
		let output = input.split_inclusive(' ').map(title).collect::<String>();
		Ok(output)
	}
}

fn title(string: &str) -> String {
	let mut chars = string.chars();
	match chars.next() {
		Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
		None => string.into(),
	}
}

pub struct LowerFirst;
impl Payload for LowerFirst {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let input = arguments.resolve::<String>(context)?;
		let mut words = input.split_inclusive(' ');
		let output = match words.next() {
			Some(word) => word.to_lowercase() + &words.collect::<String>(),
			None => input,
		};
		Ok(output)
	}
}

pub struct LowerAll;
impl Payload for LowerAll {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let input = arguments.resolve::<String>(context)?;
		Ok(input.to_lowercase())
	}
}

pub struct Split;
impl Payload for Split {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let (string, pattern, index) = arguments.resolve::<(String, String, u32)>(context)?;
		let output = string
			.split(&pattern)
			.nth(index.try_into().unwrap())
			.unwrap_or("");
		Ok(output.into())
	}
}

pub struct Pronounciation;
impl Payload for Pronounciation {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let (string, pronounciation) = arguments.resolve::<(String, String)>(context)?;
		Ok(format!("{string} ({pronounciation})"))
	}
}

#[cfg(test)]
mod test {
	use std::io::Cursor;

	use binrw::BinRead;

	use crate::sestring::SeString;

	use super::*;

	// TODO: this is disgusting
	fn str(bytes: &[u8]) -> Expression {
		Expression::String(SeString::read_le(&mut Cursor::new(bytes)).unwrap())
	}

	#[test]
	fn title_first() {
		assert_eq!(
			TitleFirst
				.resolve(&[str(b"eeby jeeby")], &mut Context::default())
				.unwrap(),
			"Eeby jeeby"
		);
	}

	#[test]
	fn title_all() {
		assert_eq!(
			TitleAll
				.resolve(&[str(b"eeby jeeby")], &mut Context::default())
				.unwrap(),
			"Eeby Jeeby"
		);
	}

	#[test]
	fn lower_first() {
		assert_eq!(
			LowerFirst
				.resolve(&[str(b"EEBY JEEBY")], &mut Context::default())
				.unwrap(),
			"eeby JEEBY"
		);
	}

	#[test]
	fn lower_all() {
		assert_eq!(
			LowerAll
				.resolve(&[str(b"EEBY JEEBY")], &mut Context::default())
				.unwrap(),
			"eeby jeeby"
		);
	}

	#[test]
	fn split() {
		assert_eq!(
			Split
				.resolve(
					&[str(b"zero one two"), str(b" "), Expression::U32(1)],
					&mut Context::default()
				)
				.unwrap(),
			"one"
		)
	}
}
