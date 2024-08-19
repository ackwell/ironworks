use crate::sestring2::error::Result;

use super::{argument::Arguments, format::State};

// Untested; No usages in excel as of 2024-08-20.
pub fn hex<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let value = arguments.exhaustive::<u32>(state)?;
	state.writer.write(&format!("{value:X}"));
	Ok(())
}

pub fn kilo<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let (value, separator) = arguments.exhaustive::<(u32, String)>(state)?;

	let formatted = match value {
		..=999 => value.to_string(),
		other => format_float(other, 1000, &separator),
	};

	state.writer.write(&formatted);

	Ok(())
}

// Untested; No usages in excel as of 2024-08-20. The existence of this strongly
// suggests that u32 is not sufficient to represent all possible values. I don't
// want to think about what that means.
pub fn byte<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let mut value = arguments.exhaustive::<u32>(state)?;
	let mut suffix = "";
	for next_suffix in ["K", "M", "G", "T"] {
		if value < 1024 {
			break;
		}
		value /= 1024;
		suffix = next_suffix;
	}
	state.writer.write(&format!("{value}{suffix}"));
	Ok(())
}

pub fn sec<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let value = arguments.exhaustive::<u32>(state)?;
	state.writer.write(&format!("{value:02}"));
	Ok(())
}

pub fn digit<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let (value, length) = arguments.exhaustive::<(u32, u32)>(state)?;
	state.writer.write(&format!(
		"{value:0length$}",
		length = usize::try_from(length).unwrap()
	));
	Ok(())
}

pub fn float<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	// Fourth argument is infrequently used, and seems to always be a 0 when set. Assumed to be a typo.
	let (value, radix, separator, _unused) =
		arguments.exhaustive::<(u32, u32, String, Option<u32>)>(state)?;

	state.writer.write(&format_float(value, radix, &separator));

	Ok(())
}

fn format_float(value: u32, radix: u32, separator: &str) -> String {
	let left = (value as f32 / radix as f32).floor();
	let right = value % radix;
	let length = usize::try_from(radix.ilog10()).unwrap();
	format!("{left}{separator}{right:0length$}")
}

pub fn ordinal<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let value = arguments.exhaustive::<u32>(state)?;
	// TODO: This apparently only has effect beyond outputting the number in
	// english? Not sure how I'd communicate the string's language to this area of
	// the formattter, so omitting the suffix for now.
	state.writer.write(&format!("{value}"));
	Ok(())
}

#[cfg(test)]
mod test {
	use crate::sestring2::{
		expression::Expression,
		format::test::{resolve, str},
	};

	use super::*;

	#[test]
	fn hex() {
		assert_eq!(resolve(super::hex, [Ok(Expression::U32(16911))]), "420F");
	}

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
	fn kilo_large() {
		assert_eq!(
			resolve(kilo, [Ok(Expression::U32(42069)), Ok(str(b","))]),
			"42,069"
		);
	}

	#[test]
	fn byte_small() {
		assert_eq!(resolve(byte, [Ok(Expression::U32(420))]), "420");
	}

	#[test]
	fn byte_large() {
		assert_eq!(
			resolve(byte, [Ok(Expression::U32(420 * 1024 * 1024))]),
			"420M"
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
	fn sec_large() {
		assert_eq!(resolve(sec, [Ok(Expression::U32(55))]), "55")
	}

	#[test]
	fn digit_unknown() {
		assert_eq!(
			resolve(digit, [Ok(Expression::StackColor), Ok(Expression::U32(5))]),
			"00000"
		)
	}

	#[test]
	fn digit_small() {
		assert_eq!(
			resolve(digit, [Ok(Expression::U32(420)), Ok(Expression::U32(5))]),
			"00420"
		)
	}

	#[test]
	fn digit_large() {
		assert_eq!(
			resolve(digit, [Ok(Expression::U32(42069)), Ok(Expression::U32(5))]),
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
					Ok(Expression::U32(5)),
					Ok(Expression::U32(100)),
					Ok(str(b"."))
				],
			),
			"0.05"
		)
	}
}
