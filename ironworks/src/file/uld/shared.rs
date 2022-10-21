use std::io::{Read, Seek};

use binrw::{BinRead, BinResult, ReadOptions};

#[derive(Debug)]
pub struct ToDo {
	kind: &'static str,
	value: i64,
}

impl BinRead for ToDo {
	type Args = (&'static str, i64);

	fn read_options<R: Read + Seek>(
		_reader: &mut R,
		_options: &ReadOptions,
		(kind, value): Self::Args,
	) -> BinResult<Self> {
		Ok(Self { kind, value })
	}
}
