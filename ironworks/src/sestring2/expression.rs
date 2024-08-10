use super::{cursor::SliceCursor, error::Error};

#[derive(Debug)]
pub enum Expression {
	U32(u32),
	// unknown? - will need non_exhaustive
}

impl Expression {
	// todo: probably need a lifetime on this?
	pub fn read(cursor: &mut SliceCursor) -> Result<Self, Error> {
		let kind = cursor.next()?;

		let expression = match kind {
			value @ 0x01..=0xCF => Self::U32(u32::from(value - 1)),
			kind @ 0xF0..=0xFE => Self::U32(read_packed_u32(cursor, kind)?),
			other => todo!("unhandled expression kind {other:?}"),
		};

		Ok(expression)
	}
}

fn read_packed_u32(cursor: &mut SliceCursor, kind: u8) -> Result<u32, Error> {
	let flags = (kind + 1) & 0b1111;
	let mut bytes = [0; 4];
	for i in (0..=3).rev() {
		if (flags & (1 << i)) == 0 {
			continue;
		}
		bytes[i] = cursor.next()?;
	}
	Ok(u32::from_le_bytes(bytes))
}
