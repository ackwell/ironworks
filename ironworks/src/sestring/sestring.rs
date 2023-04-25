use std::{
	fmt,
	io::{self, Read, Seek, SeekFrom},
	mem,
};

use binrw::{binread, BinRead, BinResult, ReadOptions};

const PAYLOAD_START: u8 = 0x02;
const PAYLOAD_END: u8 = 0x03;

/// Rich text format used in game data.
#[derive(Debug)]
pub struct SeString(Vec<Payload>);

impl fmt::Display for SeString {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		for payload in &self.0 {
			payload.fmt(formatter)?;
		}
		Ok(())
	}
}

impl BinRead for SeString {
	type Args = ();

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: &ReadOptions,
		_args: Self::Args,
	) -> BinResult<Self> {
		let mut state = SeStringReadState::default();

		loop {
			match u8::read_options(reader, options, ()) {
				// EOF or NULL signify the end of a SeString.
				Err(error) if error.is_eof() => break,
				Ok(0) => break,

				// PAYLOAD_START signifies the start of non-text payload (there's a surprise!).
				Ok(PAYLOAD_START) => {
					// Push the current state as a payload.
					state.push_buffer()?;

					// Read the new marked payload.
					let payload = Payload::read_options(reader, options, ())?;
					state.payloads.push(payload);

					// Ensure that the payload end marker exists.
					let marker = u8::read_options(reader, options, ())?;
					if marker != PAYLOAD_END {
						return Err(binrw::Error::AssertFail {
							pos: reader.stream_position()?,
							message: "payload missing end marker".into(),
						});
					}
				}

				// All other values are treated as part of the current text payload.
				maybe_byte => state.buffer.push(maybe_byte?),
			}
		}

		state.push_buffer()?;

		Ok(Self(state.payloads))
	}
}

#[derive(Default)]
struct SeStringReadState {
	payloads: Vec<Payload>,
	buffer: Vec<u8>,
}

impl SeStringReadState {
	fn push_buffer(&mut self) -> BinResult<()> {
		if !self.buffer.is_empty() {
			let bytes = mem::take(&mut self.buffer);
			let string = String::from_utf8(bytes)
				.map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

			self.payloads.push(Payload::Text(string));
		}

		Ok(())
	}
}

#[derive(Debug)]
enum Payload {
	Text(String),

	NewLine,
	SoftHyphen,
	NonBreakingSpace,
	ColorId(u32),
	EdgeColorId(u32),

	Unknown(UnknownPayload),
}

impl fmt::Display for Payload {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Text(string) => string.fmt(formatter),
			Self::NewLine => formatter.write_str("\n"),
			Self::SoftHyphen => formatter.write_str("\u{00AD}"),
			Self::NonBreakingSpace => formatter.write_str("\u{0020}"),
			Self::Unknown(_) | Self::ColorId(_) | Self::EdgeColorId(_) => Ok(()),
		}
	}
}

impl BinRead for Payload {
	type Args = ();

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: &ReadOptions,
		_args: Self::Args,
	) -> BinResult<Self> {
		let kind = u8::read_options(reader, options, ())?;
		let length = Expression::read_u32(reader, options)?;

		let position = reader.stream_position()?;

		let payload = match kind {
			0x10 => Self::NewLine,
			0x16 => Self::SoftHyphen,
			0x1D => Self::NonBreakingSpace,
			0x48 => Self::ColorId(Expression::read_u32(reader, options)?),
			0x49 => Self::EdgeColorId(Expression::read_u32(reader, options)?),
			kind => Self::Unknown(UnknownPayload::read_options(
				reader,
				options,
				(kind, length),
			)?),
		};

		reader.seek(SeekFrom::Start(position + u64::from(length)))?;

		Ok(payload)
	}
}

#[binread]
#[derive(Debug)]
#[br(import(kind: u8, length: u32))]
struct UnknownPayload {
	#[br(calc = kind)]
	kind: u8,
	#[br(count(length))]
	data: Vec<u8>,
}

#[derive(Debug)]
enum Expression {
	U32(u32),
}

impl Expression {
	// Utility for the commonly used read-expression-and-expect-it-to-be-a-number case.
	fn read_u32<R: Read + Seek>(reader: &mut R, options: &ReadOptions) -> BinResult<u32> {
		let expression = Self::read_options(reader, options, ())?;
		match expression.as_u32() {
			Some(value) => Ok(value),
			None => Err(binrw::Error::AssertFail {
				pos: reader.stream_position()?,
				message: format!("unexpected expression kind {expression:?}, expected U32"),
			}),
		}
	}

	fn as_u32(&self) -> Option<u32> {
		match self {
			Self::U32(value) => Some(*value),
		}
	}
}

impl BinRead for Expression {
	type Args = ();

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: &ReadOptions,
		_args: Self::Args,
	) -> BinResult<Self> {
		let kind = u8::read_options(reader, options, ())?;

		let expression = match kind {
			0x01..=0xCF => Self::U32(u32::from(kind - 1)),

			0xF0..=0xFD => Self::U32(read_packed_u32(kind, reader, options)?),

			other => Err(binrw::Error::AssertFail {
				pos: reader.stream_position()?,
				message: format!("unknown expression kind {other}"),
			})?,
		};

		Ok(expression)
	}
}

fn read_packed_u32<R: Read + Seek>(
	kind: u8,
	reader: &mut R,
	options: &ReadOptions,
) -> BinResult<u32> {
	let flags = (kind + 1) & 0b1111;
	let mut bytes = [0; 4];
	for (i, byte) in bytes.iter_mut().enumerate() {
		if (flags & (1 << i)) == 0 {
			continue;
		}
		*byte = u8::read_options(reader, options, ())?;
	}
	Ok(u32::from_be_bytes(bytes))
}
