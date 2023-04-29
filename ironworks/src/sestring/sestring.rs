use std::{
	io::{self, Read, Seek},
	mem,
};

use binrw::{until_eof, BinRead, BinResult, ReadOptions};

use crate::{error::Result, utility::TakeSeekableExt};

use super::{context::Context, expression::Expression, payload::Kind};

const PAYLOAD_START: u8 = 0x02;
const PAYLOAD_END: u8 = 0x03;

// TEMPORARY
impl std::fmt::Display for SeString {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let result = self
			.resolve(&mut Context::default())
			.map_err(|_| std::fmt::Error)?;
		result.fmt(formatter)
	}
}

#[derive(Debug)]
pub struct SeString(Vec<Segment>);

impl SeString {
	// TODO: Consider if I can internally use Cows to avoid string building until as late as possible
	// TODO: should this be publicly accessible outside the module? i'm tempted to say yes, but think on it.
	pub(super) fn resolve(&self, context: &mut Context) -> Result<String> {
		let Self(segments) = self;

		// Happy path - single segment can be treated as a pass-through.
		if let [first] = &segments[..] {
			return first.resolve(context);
		}

		// More than one segment, collect resolved segments into a string.
		let string = segments
			.iter()
			.map(|segment| segment.resolve(context))
			.collect::<Result<String>>()?;

		Ok(string)
	}
}

#[derive(Debug)]
enum Segment {
	Text(String),
	// TODO: consider if this should have a payload container struct rather than struct variant
	Payload {
		kind: Kind,
		arguments: Vec<Expression>,
	},
}

impl Segment {
	fn resolve(&self, context: &mut Context) -> Result<String> {
		let value = match self {
			Self::Text(string) => string.clone(),
			Self::Payload { kind, arguments } => {
				// TODO: check the context for a provided impl first?
				let payload = kind.default_payload();
				payload.resolve(arguments, context)?
			}
		};

		Ok(value)
	}
}

impl BinRead for SeString {
	type Args = ();

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: &ReadOptions,
		_args: Self::Args,
	) -> BinResult<Self> {
		let mut state = ReadState::default();

		loop {
			match u8::read_options(reader, options, ()) {
				// EOF or NULL signify the end of a SeString.
				Err(error) if error.is_eof() => break,
				Ok(0) => break,

				// PAYLOAD_START signifies the start of non-text payload.
				Ok(PAYLOAD_START) => {
					// Push the current text buffer as a segment.
					state.push_buffer()?;

					// Read and store the payload segment.
					state.segments.push(read_payload_segment(reader, options)?);

					// Ensure that we've reached a payload end marker.
					let marker = u8::read_options(reader, options, ())?;
					if marker != PAYLOAD_END {
						return Err(binrw::Error::AssertFail {
							pos: reader.stream_position()?,
							message: "payload missing end marker".into(),
						});
					}
				}

				maybe_byte => state.buffer.push(maybe_byte?),
			}
		}

		state.push_buffer()?;

		Ok(Self(state.segments))
	}
}

fn read_payload_segment<R: Read + Seek>(
	reader: &mut R,
	options: &ReadOptions,
) -> BinResult<Segment> {
	let kind = Kind::read_options(reader, options, ())?;
	let length = Expression::read_u32(reader, options)?;

	let mut buffer = reader.take_seekable(length.into())?;
	let arguments: Vec<Expression> = until_eof(&mut buffer, options, ())?;

	Ok(Segment::Payload { kind, arguments })
}

#[derive(Default)]
struct ReadState {
	segments: Vec<Segment>,
	buffer: Vec<u8>,
}

impl ReadState {
	fn push_buffer(&mut self) -> BinResult<()> {
		if self.buffer.is_empty() {
			return Ok(());
		}

		let bytes = mem::take(&mut self.buffer);
		let string = String::from_utf8(bytes)
			.map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

		self.segments.push(Segment::Text(string));

		Ok(())
	}
}
