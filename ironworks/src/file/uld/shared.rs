use std::{
	fmt,
	io::{Read, Seek},
};

use binrw::{binread, BinRead, BinResult, ReadOptions};

#[binread]
#[br(little)]
#[derive(Clone, Copy)]
pub struct ByteString<const N: usize> {
	bytes: [u8; N],
}

impl<const N: usize> PartialEq<[u8; N]> for ByteString<N> {
	fn eq(&self, other: &[u8; N]) -> bool {
		&self.bytes == other
	}
}

impl<const N: usize> fmt::Display for ByteString<N> {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_str(std::str::from_utf8(&self.bytes).map_err(|_| fmt::Error)?)
	}
}

impl<const N: usize> fmt::Debug for ByteString<N> {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_fmt(format_args!("b\"{}\"", self))
	}
}

#[derive(Debug)]
pub struct Unknown {
	kind: &'static str,
	value: i64,
}

impl BinRead for Unknown {
	type Args = (&'static str, i64);

	fn read_options<R: Read + Seek>(
		_reader: &mut R,
		_options: &ReadOptions,
		(kind, value): Self::Args,
	) -> BinResult<Self> {
		Ok(Self { kind, value })
	}
}
