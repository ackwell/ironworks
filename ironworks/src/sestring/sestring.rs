use std::{
	fmt,
	io::{self, Cursor, Read, Seek},
	mem,
};

use binrw::{binread, BinRead, BinResult, ReadOptions};

use crate::utility::TakeSeekableExt;

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

// TODO: group these properly
// TODO: these, bar text which isn't really a payload, are more like function calls, and while i've just given them arbitrary expression values, most of them take a specific array of integer and string params. would be good to encode that in some way - and given this structure, i'm honestly leaning more and more towards seperating text out of payloads.
#[derive(Debug)]
enum Payload {
	Text(String),

	// seems to be hour, day? seen (21,6), (18, 6), (4, 0), (11, 0) - all which seem to be within range to be timezone based
	// that said, second arg is seemingly optional (ref. addon:14326) - that's a MJI string about a daily fee, so a lack of day value probably implies every day?
	SetResetTime(Expression, Option<Expression>),
	SetTime(Expression),

	// TODO: these should have dedicated structs
	// expr, true, false
	If(Expression, Expression, Expression),
	// expr, branches
	// TODO: Vec<> isn't... _quite_ right, because cases, while consecutive, seem to be 1-indexed.
	Switch(Expression, Vec<Expression>),

	PlayerName(Expression),

	// acts like If but hardcoded to check if the param1 is the current player
	IfSelf(Expression, Expression, Expression),

	Icon(u32),

	// Seem to be inlined colors - an int here is an ARGB value (i think), and UnknownEC seems to be a reset?
	Color(Expression),
	EdgeColor(Expression),

	Bold(Bound),
	Italic(Bound),
	Edge(Bound),   // This is a guess
	Shadow(Bound), // As is this

	NewLine,
	SoftHyphen,
	PageSeparator, // unknown
	NonBreakingSpace,
	Icon2(u32),
	Dash,

	Number(Expression),

	// value, separator
	Kilo(Expression, Expression),

	// TODO: second? this is also used for minutes formatting, and behaves more like a two-digit zero-pad. rename? digit also seems to be a zero pad with variable count though.
	Second(Expression),

	// TODO: first param is an actual param, but what about the second? i'm guessing the second is some digit count or radix (got a 10 on 1635) or something. third param is seperator. check if there's any inline numbers for first value, if there is, should probably add a fmt for it.
	// not sure what the 4th param is. seems to be a numeric value, visible on addon:13401. check if this is every something other than 0 because it might be a typo?
	Float(Expression, Expression, Expression, Option<Expression>),

	String(Expression),
	// TODO: what is this? it seems to wrap an arbitrary string payload. StC calls it "clickable", but addon@de:110/0 uses it and that's a popup confirmation dialog, and it sure isn't clickable. i tried. winter reckons head is title case on first word, headall is title case on all contents
	Head(Expression),

	// (string value, text to split on, split to use?)
	Split(Expression, Expression, Expression),

	// Also what is this
	HeadAll(Expression),

	// grup, id
	AutoTranslate(u32, u32),

	// guessing this'll be like head and lowercase the enclosed string. should probably rename to (upper|lower)(first)?
	Lower(Expression),

	Sheet(SheetPayload),
	NounJa(NounPayload),
	NounEn(NounPayload),
	NounDe(NounPayload),
	NounFr(NounPayload),
	NounZh(NounPayload),

	// yeah this basically confirms that lower<->head
	LowerHead(Expression),

	ColorId(Expression),
	EdgeColorId(Expression),

	// Seemingly used in JP to give the (kanji, hiragana pronounciation) - rendered as "kanji (hira)", but acts a bit like furi
	Pronounciation(Expression, Expression),

	// Zero padded value, seems to be (int, int) where (value, digit count)
	Digit(Expression, Expression),
	Ordinal(Expression),

	// apparently what causes the jingles when the gold saucer banners show up?
	Sound(Expression, Expression),
	// seems to be referring to locations in the game - i have to assume it'll be an lgb id or similar
	LevelPos(Expression),

	Unknown(UnknownPayload),
}

// TODO: decide if display makes sense to impl, make proper contextal render impl
impl fmt::Display for Payload {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Text(string) => string.fmt(formatter),
			// TODO: this is omitting potentially relevant data by skipping the false branch - look into exposing a means to "format" sestring with actual values and so on.
			Self::If(_expr, branch_true, _branch_false) => branch_true.fmt(formatter),
			Self::IfSelf(_expr, branch_true, _branch_false) => branch_true.fmt(formatter),
			Self::Switch(_expr, branches) => match branches.first() {
				Some(branch) => branch.fmt(formatter),
				None => Ok(()),
			},
			Self::NewLine | Self::PageSeparator => formatter.write_str("\n"),
			Self::SoftHyphen => formatter.write_str("\u{00AD}"),
			Self::NonBreakingSpace => formatter.write_str("\u{0020}"),
			Self::Dash => formatter.write_str("\u{2013}"),
			Self::String(expression) => expression.fmt(formatter),
			Self::Head(expression) => expression.fmt(formatter),
			// TODO: sanity check this then wildcard it.
			Self::SetResetTime(..)
			| Self::SetTime(..)
			| Self::PlayerName(..)
			| Self::Bold(..)
			| Self::Italic(..)
			| Self::Edge(..)
			| Self::Shadow(..)
			| Self::Icon(..)
			| Self::Color(..)
			| Self::EdgeColor(..)
			| Self::Icon2(..)
			| Self::Number(..)
			| Self::Kilo(..)
			| Self::Second(..)
			| Self::Float(..)
			| Self::Sheet(..)
			| Self::Split(..)
			| Self::HeadAll(..)
			| Self::AutoTranslate(..)
			| Self::Lower(..)
			| Self::NounJa(..)
			| Self::NounEn(..)
			| Self::NounDe(..)
			| Self::NounFr(..)
			| Self::NounZh(..)
			| Self::LowerHead(..)
			| Self::ColorId(..)
			| Self::EdgeColorId(..)
			| Self::Pronounciation(..)
			| Self::Digit(..)
			| Self::Ordinal(..)
			| Self::Sound(..)
			| Self::LevelPos(..)
			| Self::Unknown(..) => Ok(()),
		}
	}
}

impl BinRead for Payload {
	type Args = ();

	fn read_options<R: Read + Seek>(
		outer_reader: &mut R,
		options: &ReadOptions,
		_args: Self::Args,
	) -> BinResult<Self> {
		let kind = u8::read_options(outer_reader, options, ())?;
		let length = Expression::read_u32(outer_reader, options)?;

		let reader = &mut outer_reader.take_seekable(length.into())?;

		let payload = match kind {
			0x06 => Self::SetResetTime(
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ()).ok(),
			),
			0x07 => Self::SetTime(Expression::read_options(reader, options, ())?),
			0x08 => Self::If(
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
			),
			0x09 => {
				let expresssion = Expression::read_options(reader, options, ())?;
				let mut branches = vec![];
				while reader.stream_position()? < length.into() {
					branches.push(Expression::read_options(reader, options, ())?);
				}
				Self::Switch(expresssion, branches)
			}
			0x0A => Self::PlayerName(Expression::read_options(reader, options, ())?),
			0x0F => Self::IfSelf(
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
			),
			0x10 => Self::NewLine,
			0x12 => Self::Icon(Expression::read_u32(reader, options)?),
			0x13 => Self::Color(Expression::read_options(reader, options, ())?),
			0x14 => Self::EdgeColor(Expression::read_options(reader, options, ())?),
			0x16 => Self::SoftHyphen,
			0x17 => Self::PageSeparator,
			0x19 => Self::Bold(Bound::read_options(reader, options, ())?),
			0x1A => Self::Italic(Bound::read_options(reader, options, ())?),
			0x1B => Self::Edge(Bound::read_options(reader, options, ())?),
			0x1C => Self::Shadow(Bound::read_options(reader, options, ())?),
			0x1D => Self::NonBreakingSpace,
			0x1E => Self::Icon2(Expression::read_u32(reader, options)?),
			0x1F => Self::Dash,
			0x20 => Self::Number(Expression::read_options(reader, options, ())?),
			0x22 => Self::Kilo(
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
			),
			0x24 => Self::Second(Expression::read_options(reader, options, ())?),
			0x26 => Self::Float(
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ()).ok(),
			),
			0x28 => Self::Sheet(SheetPayload::read_options(reader, options, ())?),
			0x29 => Self::String(Expression::read_options(reader, options, ())?),
			0x2B => Self::Head(Expression::read_options(reader, options, ())?),
			0x2C => Self::Split(
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
			),
			0x2D => Self::HeadAll(Expression::read_options(reader, options, ())?),
			0x2E => Self::AutoTranslate(
				Expression::read_u32(reader, options)?,
				Expression::read_u32(reader, options)?,
			),
			0x2F => Self::Lower(Expression::read_options(reader, options, ())?),
			0x30 => Self::NounJa(NounPayload::read_options(reader, options, ())?),
			0x31 => Self::NounEn(NounPayload::read_options(reader, options, ())?),
			0x32 => Self::NounDe(NounPayload::read_options(reader, options, ())?),
			0x33 => Self::NounFr(NounPayload::read_options(reader, options, ())?),
			0x34 => Self::NounZh(NounPayload::read_options(reader, options, ())?),
			0x40 => Self::LowerHead(Expression::read_options(reader, options, ())?),
			0x48 => Self::ColorId(Expression::read_options(reader, options, ())?),
			0x49 => Self::EdgeColorId(Expression::read_options(reader, options, ())?),
			0x4A => Self::Pronounciation(
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
			),
			0x50 => Self::Digit(
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
			),
			0x51 => Self::Ordinal(Expression::read_options(reader, options, ())?),
			0x60 => Self::Sound(
				Expression::read_options(reader, options, ())?,
				Expression::read_options(reader, options, ())?,
			),
			0x61 => Self::LevelPos(Expression::read_options(reader, options, ())?),
			kind => {
				// TODO: actually use unknown payload properly, this is just for error checking temporarily
				let p = UnknownPayload::read_options(reader, options, (kind, length))?;
				return Err(binrw::Error::AssertFail {
					pos: reader.stream_position()?,
					message: format!("{:#x} {:?}", p.kind, p.data),
				});
			}
		};

		// let expected = position + u64::from(length);
		let actual = reader.stream_position()?;
		if u64::from(length) != actual {
			return Err(binrw::Error::AssertFail {
				pos: actual,
				message: format!(
					"kind {kind:#X} length mismatch, expected {length}, read {}",
					actual
				),
			});
		}

		// reader.seek(SeekFrom::Start(position + u64::from(length)))?;

		Ok(payload)
	}
}

#[derive(Debug)]
enum Bound {
	Start,
	End,
}

impl BinRead for Bound {
	type Args = ();

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: &ReadOptions,
		_args: Self::Args,
	) -> BinResult<Self> {
		match Expression::read_u32(reader, options)? {
			0 => Ok(Self::End),
			1 => Ok(Self::Start),
			other => Err(binrw::Error::AssertFail {
				pos: reader.stream_position()?,
				message: format!("unexpected value \"{other}\" for bound"),
			})?,
		}
	}
}

// TODO: sheet can be assumed to be a string, latter numbers - should that be enforced? i imagine some of the row usages will be params but i don't suspect the sheet will be a param.
#[binread]
#[derive(Debug)]
struct SheetPayload {
	sheet: Expression,
	row: Expression,

	#[br(try)]
	column: Option<Expression>,

	// this is seemingly the parameter(s?) passed to the selected sestring in the target sheet field
	#[br(try)]
	parameter: Option<Expression>,
}

#[binread]
#[derive(Debug)]
struct NounPayload {
	sheet: Expression,
	attributive_row: Expression,
	row: Expression,

	// TODO: I'm not convinced by these - column being driven by the count param on addon:110/0 seems wrong.
	// winter thinks the args are sheet,sets,row,count,attributes,extra
	#[br(try)]
	column: Option<Expression>,

	#[br(try)]
	attributive_index: Option<Expression>,

	#[br(try)]
	parameter: Option<Expression>,
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

// TODO: consider grouping some of these into enums with sub enums? maybe?
#[derive(Debug)]
enum Expression {
	// Inline values
	U32(u32),
	String(SeString),

	// PLaceholders
	UnkD8, // used in a m:s:(this) setup, so presumably a sub-second value. is put in a two-digit zero-pad, so perhaps centiseconds?
	Second, // maybe?
	Minute,
	Hour,
	Day,
	Weekday,
	Month,
	Year,

	// Expected to be placeholders
	// TODO: Look into this more
	UnknownEC,

	// Comparators
	Gte(Box<Expression>, Box<Expression>),
	Gt(Box<Expression>, Box<Expression>),
	Lte(Box<Expression>, Box<Expression>),
	Lt(Box<Expression>, Box<Expression>),
	Eq(Box<Expression>, Box<Expression>),
	Ne(Box<Expression>, Box<Expression>),

	// Parameters
	IntegerParameter(Box<Expression>),
	PlayerParameter(Box<Expression>),
	StringParameter(Box<Expression>),
	ObjectParameter(Box<Expression>),
}

impl Expression {
	// Utility for the commonly used read-expression-and-expect-it-to-be-a-number case.
	fn read_u32<R: Read + Seek>(reader: &mut R, options: &ReadOptions) -> BinResult<u32> {
		let expression = Self::read_options(reader, options, ())?;
		match expression {
			Self::U32(value) => Ok(value),
			other => Err(binrw::Error::AssertFail {
				pos: reader.stream_position()?,
				message: format!("unexpected expression kind {other:?}, expected U32"),
			}),
		}
	}
}

impl fmt::Display for Expression {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::U32(value) => value.fmt(formatter),
			Self::String(value) => value.fmt(formatter),

			_ => Ok(()),
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

		let mut read_expr =
			|| -> BinResult<_> { Ok(Box::new(Expression::read_options(reader, options, ())?)) };

		let expression = match kind {
			0x01..=0xCF => Self::U32(u32::from(kind - 1)),

			0xD8 => Self::UnkD8,
			0xD9 => Self::Second,
			0xDA => Self::Minute,
			0xDB => Self::Hour,
			0xDC => Self::Day,
			0xDD => Self::Weekday,
			0xDE => Self::Month,
			0xDF => Self::Year,

			0xE0 => Self::Gte(read_expr()?, read_expr()?),
			0xE1 => Self::Gt(read_expr()?, read_expr()?),
			0xE2 => Self::Lte(read_expr()?, read_expr()?),
			0xE3 => Self::Lt(read_expr()?, read_expr()?),
			0xE4 => Self::Eq(read_expr()?, read_expr()?),
			0xE5 => Self::Ne(read_expr()?, read_expr()?),

			0xE8 => Self::IntegerParameter(read_expr()?),
			0xE9 => Self::PlayerParameter(read_expr()?),
			0xEA => Self::StringParameter(read_expr()?),
			0xEB => Self::ObjectParameter(read_expr()?),

			// ??? seems to be used as a "reset" marker for color/edgecolor?
			0xEC => Self::UnknownEC,

			0xF0..=0xFE => Self::U32(read_packed_u32(kind, reader, options)?),

			0xFF => Self::String(read_inline_sestring(reader, options)?),

			other => Err(binrw::Error::AssertFail {
				pos: reader.stream_position()?,
				message: format!("unknown expression kind {other:#X}"),
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
	for i in (0..=3).rev() {
		if (flags & (1 << i)) == 0 {
			continue;
		}
		bytes[i] = u8::read_options(reader, options, ())?;
	}
	Ok(u32::from_le_bytes(bytes))
}

fn read_inline_sestring<R: Read + Seek>(
	reader: &mut R,
	options: &ReadOptions,
) -> BinResult<SeString> {
	let length = Expression::read_u32(reader, options)?;
	let mut buffer = Cursor::new(Vec::with_capacity(length.try_into().unwrap()));
	io::copy(&mut reader.take(length.into()), &mut buffer)?;
	buffer.set_position(0);
	let sestring = SeString::read_options(&mut buffer, options, ())?;
	Ok(sestring)
}
