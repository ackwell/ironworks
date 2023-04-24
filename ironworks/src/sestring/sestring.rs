use std::{
	fmt,
	io::{Read, Seek, SeekFrom},
};

use binrw::{binread, BinRead, BinResult, ReadOptions};

/// Rich text format used in game data.
#[derive(Debug)]
pub struct SeString(Vec<Item>);

impl fmt::Display for SeString {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		for item in &self.0 {
			item.fmt(formatter)?;
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
		let mut items = vec![];
		loop {
			// TODO: I'm not a fan of this read-rewind, but it seems the sanest way to keep the understanding of null (mostly) isolated to the top level where it's actively relevant.
			let maybe_null = u8::read_options(reader, options, ())?;
			if maybe_null == 0 {
				break;
			}
			reader.seek(SeekFrom::Current(-1))?;
			items.push(Item::read_options(reader, options, ())?);
		}
		Ok(Self(items))
	}
}

const PAYLOAD_START: u8 = 0x02;
const PAYLOAD_END: u8 = 0x03;

#[binread]
#[derive(Debug)]
enum Item {
	// All payloads are identified by a leading sigil. This variant must preceed
	// Text to ensure payloads are checked first.
	Payload(PayloadContainer),

	Text(#[br(parse_with = parse_text_item)] String),
}

impl fmt::Display for Item {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Payload(_) => Ok(()),
			Self::Text(string) => string.fmt(formatter),
		}
	}
}

fn parse_text_item<R: Read + Seek>(
	reader: &mut R,
	options: &ReadOptions,
	_args: (),
) -> BinResult<String> {
	// Collect the bytes of the string.
	let mut bytes = vec![];
	loop {
		// Get the next byte. EOF or NULL signify the end of the stream, and a START
		// signifies a payload. All other values should be treated as part of the string.
		match u8::read_options(reader, options, ()) {
			Ok(0) | Ok(PAYLOAD_START) => {
				reader.seek(SeekFrom::Current(-1))?;
				break;
			}

			other => bytes.push(other?),
		};
	}

	let position = reader.stream_position()?;

	if bytes.is_empty() {
		return Err(binrw::Error::AssertFail {
			pos: position,
			message: "zero-length string".into(),
		});
	}

	// Translate the bytes into a utf8 string, failing out early on invalid data.
	String::from_utf8(bytes).map_err(|error| binrw::Error::Custom {
		pos: position,
		err: Box::new(error),
	})
}

#[binread]
#[derive(Debug)]
struct PayloadContainer {
	// Using a temp field rather tham magic to allow reuse of the const definition.
	#[br(temp, assert(marker_start == PAYLOAD_START))]
	marker_start: u8,

	_kind: u8,

	#[br(temp)]
	length: PackedU32,

	// temp
	#[br(count(length.0))]
	_data: Vec<u8>,

	#[br(temp, assert(marker_end == PAYLOAD_END))]
	marker_end: u8,
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
