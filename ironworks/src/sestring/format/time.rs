use time::{OffsetDateTime, ext::NumericalDuration};

use crate::sestring::error::{Error, Result};

use super::{argument::Arguments, format::State, value::Value};

// 2013/08/27 08:00:00 GMT, release of FFXIV:ARR
pub const FFXIV_EPOCH: u32 = 1377590400;

pub fn set_time<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let timestamp = match arguments.exhaustive::<Value>(state)? {
		Value::Unknown => FFXIV_EPOCH,
		other => other.into(),
	};

	state.time = timestamp;

	Ok(())
}

pub fn set_reset_time<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let (target_hour, target_weekday) = arguments.exhaustive::<(u32, Option<u32>)>(state)?;

	let time = state.time;
	let mut datetime =
		OffsetDateTime::from_unix_timestamp(time.into()).map_err(|_error| Error::InvalidMacro)?;

	let mut day_offset = 0;

	// Get the offset required to reach the target weekday. If the target is in
	// the past this week, move to next week.
	if let Some(target_weekday) = target_weekday {
		let current_weekday = datetime.weekday().number_days_from_sunday();
		day_offset += i64::from(target_weekday) - i64::from(current_weekday);
		if day_offset < 0 {
			day_offset += 7;
		}
	}

	// If we've not moved forward on the day offset yet, and the target hour has
	// passed, move to the next day.
	if target_hour < datetime.hour().into() && day_offset <= 0 {
		day_offset += 1;
	}

	datetime += day_offset.days();
	datetime = datetime
		.replace_hour(target_hour.try_into().unwrap())
		.map_err(|_error| Error::InvalidMacro)?;

	state.time = datetime.unix_timestamp().try_into().unwrap();

	Ok(())
}

#[cfg(test)]
mod test {
	use crate::sestring::{
		format::{format::format_sestring, test::with_state},
		sestring::SeString,
	};

	fn render_date_time(prelude: &[u8]) -> String {
		let content = [
			prelude,
			&[
				0x02, 0x20, 0x02, 0xDF, 0x03, b' ', // Year
				0x02, 0x20, 0x02, 0xDE, 0x03, b' ', // Month
				0x02, 0x20, 0x02, 0xDD, 0x03, b' ', // Weekday
				0x02, 0x20, 0x02, 0xDC, 0x03, b' ', // Day
				0x02, 0x20, 0x02, 0xDB, 0x03, b' ', // Hour
				0x02, 0x20, 0x02, 0xDA, 0x03, b' ', // Minute
				0x02, 0x20, 0x02, 0xD9, 0x03, b' ', // Second
				0x02, 0x20, 0x02, 0xD8, 0x03, // Millisecond
			],
		]
		.concat();

		let sestring = SeString::new(content.as_slice());

		with_state(|state| {
			format_sestring(sestring, state).expect("format should not fail");
		})
	}

	#[test]
	fn set_time_unknown() {
		// Using EC (StackColor) as Unknown
		assert_eq!(
			render_date_time(&[0x02, 0x07, 0x02, 0xEC, 0x03],),
			"2013 8 3 27 8 0 0 0"
		)
	}

	#[test]
	fn set_time_explicit() {
		assert_eq!(
			// 2023/04/20 06:09:42
			render_date_time(&[0x02, 0x07, 0x06, 0xFE, 0x64, 0x40, 0xD7, 0x26, 0x03]),
			"2023 4 5 20 6 9 42 0"
		);
	}

	#[test]
	fn set_reset_time_forward() {
		assert_eq!(
			// SetResetTime(12, 6)
			render_date_time(&[0x02, 0x06, 0x03, 0x0D, 0x07, 0x03]),
			"2013 8 7 31 12 0 0 0"
		);
	}

	#[test]
	fn set_reset_time_same_day() {
		assert_eq!(
			// SetResetTime(6, 2)
			render_date_time(&[0x02, 0x06, 0x03, 0x07, 0x03, 0x03]),
			"2013 8 4 28 6 0 0 0"
		);
	}

	#[test]
	fn set_reset_time_backward() {
		assert_eq!(
			// SetResetTime(12, 0)
			render_date_time(&[0x02, 0x06, 0x03, 0x0D, 0x01, 0x03]),
			"2013 9 1 1 12 0 0 0"
		);
	}

	#[test]
	fn set_reset_time_time_forward() {
		assert_eq!(
			// SetResetTime(12)
			render_date_time(&[0x02, 0x06, 0x02, 0x0D, 0x03]),
			"2013 8 3 27 12 0 0 0"
		);
	}

	#[test]
	fn set_reset_time_time_backward() {
		assert_eq!(
			// SetResetTime(6)
			render_date_time(&[0x02, 0x06, 0x02, 0x07, 0x03]),
			"2013 8 4 28 6 0 0 0"
		);
	}
}
