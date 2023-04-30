use crate::{
	error::Result,
	sestring::{
		context::Context,
		expression::Expression,
		value::{ArgumentExt, Value},
	},
};

use super::payload::Payload;

// 2013/08/27 08:00:00 GMT, release of FFXIV:ARR
const FFXIV_EPOCH: u32 = 1377590400;

pub struct SetTime;
impl Payload for SetTime {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let mut timestamp = arguments.resolve::<u32>(context)?;

		// If unknown, default to the ffxiv epoch.
		if timestamp == Value::UNKNOWN {
			timestamp = FFXIV_EPOCH;
		}

		context.set_time(timestamp);

		Ok("".into())
	}
}

#[cfg(test)]
mod test {
	use std::io::Cursor;

	use binrw::BinRead;

	use crate::sestring::SeString;

	use super::*;

	// TODO: this is disgusting - i really need to make a builder pathway so tests aren't absolute dogshit
	fn str(bytes: &[u8]) -> SeString {
		SeString::read_le(&mut Cursor::new(bytes)).unwrap()
	}

	fn render_date_time() -> &'static [u8] {
		&[
			0x02, 0x20, 0x02, 0xDF, 0x03, b' ', // Year
			0x02, 0x20, 0x02, 0xDE, 0x03, b' ', // Month
			0x02, 0x20, 0x02, 0xDD, 0x03, b' ', // Weekday
			0x02, 0x20, 0x02, 0xDC, 0x03, b' ', // Day
			0x02, 0x20, 0x02, 0xDB, 0x03, b' ', // Hour
			0x02, 0x20, 0x02, 0xDA, 0x03, b' ', // Minute
			0x02, 0x20, 0x02, 0xD9, 0x03, // Second
		]
	}

	#[test]
	fn set_time_unknown() {
		assert_eq!(
			str(&[&[0x02, 0x07, 0x03, 0xE8, 0x02, 0x03], render_date_time()].concat())
				.resolve(&mut Context::default())
				.unwrap(),
			"2013 8 3 27 8 0 0"
		);
	}

	#[test]
	fn set_time_explicit() {
		assert_eq!(
			str(&[
				// 2023/04/20 06:09:42
				&[0x02, 0x07, 0x06, 0xFE, 0x64, 0x40, 0xD7, 0x26, 0x03],
				render_date_time()
			]
			.concat())
			.resolve(&mut Context::default())
			.unwrap(),
			"2023 4 5 20 6 9 42"
		);
	}
}
