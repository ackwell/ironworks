use binrw::binread;

use super::{
	character::NewLine,
	control_flow::IfSelf,
	payload::{Fallback, Payload},
};

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
