use std::{
	fmt,
	io::{self, Read, Seek},
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
		let mut state = SeStringReader::default();

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
struct SeStringReader {
	payloads: Vec<Payload>,
	buffer: Vec<u8>,
}

impl SeStringReader {
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

	Unknown(UnknownPayload),
}

impl fmt::Display for Payload {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Text(string) => string.fmt(formatter),
			Self::NewLine => formatter.write_str("\n"),
			Self::Unknown(_) => Ok(()),
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
		let length = PackedU32::read_options(reader, options, ())?.0;

		let payload = match kind {
			0x10 => Self::NewLine,
			kind => Self::Unknown(UnknownPayload::read_options(
				reader,
				options,
				(kind, length),
			)?),
		};

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

// TODO: Going by lumina, this is part of a more hollistic expression system that is used (i presume) in If payloads and such. Flesh out.
struct PackedU32(u32);

impl BinRead for PackedU32 {
	type Args = ();

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: &ReadOptions,
		_args: Self::Args,
	) -> BinResult<Self> {
		let marker = u8::read_options(reader, options, ())?;

		let value = match marker {
			0x01..=0xCF => u32::from(marker - 1),

			0xF0..=0xFD => {
				let flags = (marker + 1) & 0b1111;
				let mut bytes = [0; 4];
				for i in (0..=3).rev() {
					if (flags & (1 << i)) == 0 {
						continue;
					}
					bytes[i] = u8::read_options(reader, options, ())?;
				}
				u32::from_le_bytes(bytes)
			}

			other => Err(binrw::Error::AssertFail {
				pos: reader.stream_position()?,
				message: format!("unexpected marker packed u32 marker {other}"),
			})?,
		};

		Ok(Self(value))
	}
}
