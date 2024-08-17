use crate::sestring2::error::Result;

use super::{argument::Arguments, context::Context, resolve::Resolve};

pub fn new_line<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &mut Context,
) -> Result<String> {
	args.evaluate::<()>(resolver, context)?;
	Ok("\n".into())
}

pub fn soft_hyphen<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &mut Context,
) -> Result<String> {
	args.evaluate::<()>(resolver, context)?;
	Ok("\u{00AD}".into())
}

pub fn non_breaking_space<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &mut Context,
) -> Result<String> {
	args.evaluate::<()>(resolver, context)?;
	Ok("\u{0020}".into())
}

pub fn hyphen<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &mut Context,
) -> Result<String> {
	args.evaluate::<()>(resolver, context)?;
	Ok("\u{2013}".into())
}
