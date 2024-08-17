use crate::sestring2::error::Result;

use super::{argument::Arguments, context::Context, resolve::Resolve};

pub fn identity<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	args.evaluate::<String>(resolver, context)
}

pub fn kilo<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	let (value, separator) = args.evaluate::<(u32, String)>(resolver, context)?;

	if value < 1000 {
		return Ok(value.to_string());
	}

	let left = (value as f32 / 1000.0).floor();
	let right = value % 1000;
	Ok(format!("{left}{separator}{right:03}"))
}

pub fn sec<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	let value = args.evaluate::<u32>(resolver, context)?;
	Ok(format!("{value:02}"))
}

pub fn digit<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	let (value, length) = args.evaluate::<(u32, u32)>(resolver, context)?;
	Ok(format!(
		"{value:0length$}",
		length = usize::try_from(length).unwrap()
	))
}

pub fn float<'a>(
	resolver: &mut impl Resolve,
	args: impl Arguments<'a>,
	context: &Context,
) -> Result<String> {
	let (value, radix, separator, _unused) =
		args.evaluate::<(u32, u32, String, Option<u32>)>(resolver, context)?;

	let left = (value as f32 / radix as f32).floor();
	let right = value % radix;
	Ok(format!("{left}{separator}{right}"))
}

#[cfg(test)]
mod test {
	use crate::sestring2::{
		resolve::shared::test::{resolve, str},
		Expression,
	};

	use super::*;

	#[test]
	fn kilo_unknown() {
		assert_eq!(
			resolve(kilo, [Ok(Expression::StackColor), Ok(str(b","))]),
			"0"
		);
	}

	#[test]
	fn kilo_small() {
		assert_eq!(
			resolve(kilo, [Ok(Expression::U32(69)), Ok(str(b","))]),
			"69"
		);
	}

	#[test]
	fn thousands_large() {
		assert_eq!(
			resolve(kilo, [Ok(Expression::U32(42069)), Ok(str(b","))]),
			"42,069"
		);
	}

	#[test]
	fn sec_unknown() {
		assert_eq!(resolve(sec, [Ok(Expression::StackColor)]), "00")
	}

	#[test]
	fn sec_small() {
		assert_eq!(resolve(sec, [Ok(Expression::U32(5))]), "05")
	}

	#[test]
	fn two_digit_large() {
		assert_eq!(resolve(sec, [Ok(Expression::U32(55))]), "55")
	}

	#[test]
	fn digit_unknown() {
		assert_eq!(
			resolve(digit, [Ok(Expression::StackColor), Ok(Expression::U32(5))],),
			"00000"
		)
	}

	#[test]
	fn digit_small() {
		assert_eq!(
			resolve(digit, [Ok(Expression::U32(420)), Ok(Expression::U32(5))],),
			"00420"
		)
	}

	#[test]
	fn digit_large() {
		assert_eq!(
			resolve(digit, [Ok(Expression::U32(42069)), Ok(Expression::U32(5))],),
			"42069"
		)
	}

	#[test]
	fn float_unknown() {
		assert_eq!(
			resolve(
				float,
				[
					Ok(Expression::StackColor),
					Ok(Expression::U32(10)),
					Ok(str(b"."))
				],
			),
			"0.0"
		)
	}

	#[test]
	fn float_zero() {
		assert_eq!(
			resolve(
				float,
				[
					Ok(Expression::U32(0)),
					Ok(Expression::U32(10)),
					Ok(str(b"."))
				],
			),
			"0.0"
		)
	}

	#[test]
	fn float_small() {
		assert_eq!(
			resolve(
				float,
				[
					Ok(Expression::U32(5)),
					Ok(Expression::U32(10)),
					Ok(str(b"."))
				],
			),
			"0.5"
		)
	}

	#[test]
	fn float_large() {
		assert_eq!(
			resolve(
				float,
				[
					Ok(Expression::U32(55)),
					Ok(Expression::U32(10)),
					Ok(str(b"."))
				],
			),
			"5.5"
		)
	}

	#[test]
	fn float_radix() {
		assert_eq!(
			resolve(
				float,
				[
					Ok(Expression::U32(55)),
					Ok(Expression::U32(100)),
					Ok(str(b"."))
				],
			),
			"0.55"
		)
	}
}
