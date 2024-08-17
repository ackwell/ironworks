use crate::sestring2::error::Result;

use super::{argument::Arguments, context::Context, resolve::Resolve};

// TODO: excel-specific logic for these on the context

pub fn sheet<'a>(
	_resolver: &mut impl Resolve,
	_args: impl Arguments<'a>,
	_context: &Context,
) -> Result<String> {
	Ok("".into())
}

pub fn ja_noun<'a>(
	_resolver: &mut impl Resolve,
	_args: impl Arguments<'a>,
	_context: &Context,
) -> Result<String> {
	Ok("".into())
}

pub fn en_noun<'a>(
	_resolver: &mut impl Resolve,
	_args: impl Arguments<'a>,
	_context: &Context,
) -> Result<String> {
	Ok("".into())
}

pub fn de_noun<'a>(
	_resolver: &mut impl Resolve,
	_args: impl Arguments<'a>,
	_context: &Context,
) -> Result<String> {
	Ok("".into())
}

pub fn fr_noun<'a>(
	_resolver: &mut impl Resolve,
	_args: impl Arguments<'a>,
	_context: &Context,
) -> Result<String> {
	Ok("".into())
}

pub fn ch_noun<'a>(
	_resolver: &mut impl Resolve,
	_args: impl Arguments<'a>,
	_context: &Context,
) -> Result<String> {
	Ok("".into())
}
