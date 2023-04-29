use crate::{
	error::Result,
	sestring::{context::Context, expression::Expression, value::ArgumentExt},
};

use super::payload::Payload;

pub struct Character<const C: char>;
impl<const C: char> Payload for Character<C> {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		arguments.resolve::<()>(context)?;
		Ok(C.into())
	}
}

pub const NEW_LINE: Character<'\n'> = Character::<'\n'>;
pub const SOFT_HYPHEN: Character<'\u{00AD}'> = Character::<'\u{00AD}'>;
pub const NON_BREAKING_SPACE: Character<'\u{0020}'> = Character::<'\u{0020}'>;
pub const DASH: Character<'\u{2013}'> = Character::<'\u{2013}'>;
