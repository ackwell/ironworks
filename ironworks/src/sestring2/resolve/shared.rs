use crate::sestring2::error::Result;

use super::{argument::Arguments, context::Context, resolve::Resolve};

pub fn noop<'a>(
	_resolver: &mut impl Resolve,
	_args: impl Arguments<'a>,
	_context: &Context,
) -> Result<String> {
	Ok("".into())
}

#[cfg(test)]
pub mod test {
	use crate::sestring2::{DefaultString, Expression, SeString};

	use super::*;

	pub fn str(content: &[u8]) -> Expression {
		Expression::SeString(SeString::from(content))
	}

	pub fn resolve<'a, F, I>(r#fn: F, input: I) -> String
	where
		F: FnOnce(&mut DefaultString, I::IntoIter, &mut Context) -> Result<String>,
		I: IntoIterator,
	{
		let mut resolver = DefaultString::new();
		let mut context = Context::new();
		let args = input.into_iter();
		r#fn(&mut resolver, args, &mut context).expect("test fn should not error")
	}
}
