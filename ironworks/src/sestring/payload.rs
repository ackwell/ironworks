use binrw::binread;

use crate::{error::Result, sestring::value::ArgumentExt, Error, ErrorValue};

use super::{context::Context, expression::Expression, value::Value};

pub trait Payload {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String>;
}

#[rustfmt::skip]
#[non_exhaustive]
#[binread]
#[derive(Debug)]
pub enum Kind {
	#[br(magic = 0x06_u8)] SetResetTime,
	#[br(magic = 0x07_u8)] SetTime,
	#[br(magic = 0x08_u8)] If,
	#[br(magic = 0x09_u8)] Switch,
	#[br(magic = 0x0A_u8)] PlayerName,
	#[br(magic = 0x0F_u8)] IfSelf,
	#[br(magic = 0x10_u8)] NewLine,
	#[br(magic = 0x12_u8)] Icon,
	#[br(magic = 0x13_u8)] Color,
	#[br(magic = 0x14_u8)] EdgeColor,
	#[br(magic = 0x16_u8)] SoftHyphen,
	#[br(magic = 0x17_u8)] PageSeparator,
	#[br(magic = 0x19_u8)] Bold,
	#[br(magic = 0x1A_u8)] Italic,
	#[br(magic = 0x1B_u8)] Edge,
	#[br(magic = 0x1C_u8)] Shadow,
	#[br(magic = 0x1D_u8)] NonBreakingSpace,
	#[br(magic = 0x1E_u8)] Icon2,
	#[br(magic = 0x1F_u8)] Dash,
	#[br(magic = 0x20_u8)] Number,
	#[br(magic = 0x22_u8)] Kilo,
	#[br(magic = 0x24_u8)] Second,
	#[br(magic = 0x26_u8)] Float,
	#[br(magic = 0x28_u8)] Sheet,
	#[br(magic = 0x29_u8)] String,
	#[br(magic = 0x2B_u8)] Head,
	#[br(magic = 0x2C_u8)] Split,
	#[br(magic = 0x2D_u8)] HeadAll,
	#[br(magic = 0x2E_u8)] AutoTranslate,
	#[br(magic = 0x2F_u8)] Lower,
	#[br(magic = 0x30_u8)] NounJa,
	#[br(magic = 0x31_u8)] NounEn,
	#[br(magic = 0x32_u8)] NounDe,
	#[br(magic = 0x33_u8)] NounFr,
	#[br(magic = 0x34_u8)] NounZh,
	#[br(magic = 0x40_u8)] LowerHead,
	#[br(magic = 0x48_u8)] ColorId,
	#[br(magic = 0x49_u8)] EdgeColorId,
	#[br(magic = 0x4A_u8)] Pronounciation,
	#[br(magic = 0x50_u8)] Digit,
	#[br(magic = 0x51_u8)] Ordinal,
	#[br(magic = 0x60_u8)] Sound,
	#[br(magic = 0x61_u8)] LevelPos,

	Unknown(u8),
}

impl Kind {
	pub fn default_payload(&self) -> &dyn Payload {
		match self {
			Self::NewLine => &NewLine,
			Self::IfSelf => &IfSelf,
			_ => &Fallback,
		}
	}
}

struct Fallback;

impl Payload for Fallback {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		// Given this is a fallback and therefore we do not know the semantics of
		// the arguments, err to collecting all valid string arguments and returning as-is.
		let string = arguments
			.iter()
			.filter_map(|argument| match argument.resolve(context) {
				Ok(Value::String(string)) => Some(Ok(string)),
				Ok(Value::U32(_)) => None,
				Err(error) => Some(Err(error)),
			})
			.collect::<Result<String>>()?;

		Ok(string)
	}
}

struct NewLine;

impl Payload for NewLine {
	fn resolve(&self, arguments: &[Expression], _context: &mut Context) -> Result<String> {
		if !arguments.is_empty() {
			return Err(Error::Invalid(
				// Should i have a sestring error value? maybe once i add a feature i guess?
				ErrorValue::Other("SeString".into()),
				format!("NewLine expected 0 arguments, got {}", arguments.len()),
			));
		}

		Ok("\n".into())
	}
}

struct IfSelf;

impl Payload for IfSelf {
	fn resolve(&self, arguments: &[Expression], context: &mut Context) -> Result<String> {
		let (_player_id, branch_true, _branch_false) =
			arguments.resolve::<(u32, String, String)>(context)?;

		// TODO: this is just assuming that every player id is the player - i'll need to decide how to handle this conceptually - maybe a faux `IfSelf::PLAYER_ID`? but that'd mean assuming which param is the player ID, which isn't safe

		Ok(branch_true)
	}
}
