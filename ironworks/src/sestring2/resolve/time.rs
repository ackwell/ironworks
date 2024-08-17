use crate::sestring2::error::Result;

use super::{argument::Arguments, context::Context, resolve::Resolve, value::Value};

// 2013/08/27 08:00:00 GMT, release of FFXIV:ARR
const FFXIV_EPOCH: u32 = 1377590400;

pub fn set_time<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &mut Context,
) -> Result<String> {
	let timestamp = match args.evaluate::<Value>(resolver, context)? {
		Value::Unknown => FFXIV_EPOCH,
		other => other.into(),
	};

	context.set_time(timestamp);

	Ok("".into())
}
