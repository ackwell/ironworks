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

		match kind {
			value @ 0x01..=0xCF => Ok(Self::U32(u32::from(value - 1))),
			other => todo!("unhandled expression kind {other:?}"),
		}
	}
}
