use binrw::binread;

use super::{
	case::{LowerAll, LowerFirst, TitleAll, TitleFirst},
	character::{DASH, NEW_LINE, NON_BREAKING_SPACE, SOFT_HYPHEN},
	control_flow::{If, IfSelf, Switch},
	format::{Float, Identity, Thousands, TwoDigit, ZeroPad},
	payload::{Fallback, NoOp, Payload},
	player::PlayerName,
	time::{SetResetTime, SetTime},
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

	#[br(magic = 0x22_u8)] Thousands,

	#[br(magic = 0x24_u8)] TwoDigit,

	#[br(magic = 0x26_u8)] Float,

	#[br(magic = 0x28_u8)] Sheet,
	#[br(magic = 0x29_u8)] String,

	#[br(magic = 0x2B_u8)] TitleFirst,
	#[br(magic = 0x2C_u8)] Split,
	#[br(magic = 0x2D_u8)] TitleAll,
	#[br(magic = 0x2E_u8)] AutoTranslate,
	#[br(magic = 0x2F_u8)] LowerAll,
	#[br(magic = 0x30_u8)] NounJa,
	#[br(magic = 0x31_u8)] NounEn,
	#[br(magic = 0x32_u8)] NounDe,
	#[br(magic = 0x33_u8)] NounFr,
	#[br(magic = 0x34_u8)] NounZh,

	#[br(magic = 0x40_u8)] LowerFirst,

	#[br(magic = 0x48_u8)] ColorId,
	#[br(magic = 0x49_u8)] EdgeColorId,
	#[br(magic = 0x4A_u8)] Pronounciation,

	#[br(magic = 0x50_u8)] ZeroPad,
	#[br(magic = 0x51_u8)] Ordinal,

	#[br(magic = 0x60_u8)] Sound,
	#[br(magic = 0x61_u8)] LevelPos,

	Unknown(u8),
}

impl Kind {
	pub fn default_payload(&self) -> &dyn Payload {
		#[cfg(feature = "excel")]
		match self {
			Self::Sheet => return &super::sheet::Sheet,
			Self::AutoTranslate => return &super::sheet::AutoTranslate,
			Self::NounJa => return &super::sheet::Noun(crate::excel::Language::Japanese),
			Self::NounEn => return &super::sheet::Noun(crate::excel::Language::English),
			Self::NounDe => return &super::sheet::Noun(crate::excel::Language::German),
			Self::NounFr => return &super::sheet::Noun(crate::excel::Language::French),
			Self::NounZh => return &super::sheet::Noun(crate::excel::Language::ChineseSimplified),
			_ => {}
		};

		match self {
			Self::NewLine | Self::PageSeparator => &NEW_LINE,
			Self::SoftHyphen => &SOFT_HYPHEN,
			Self::NonBreakingSpace => &NON_BREAKING_SPACE,
			Self::Dash => &DASH,

			Self::PlayerName => &PlayerName,

			Self::String | Self::Number => &Identity,
			Self::Thousands => &Thousands,
			Self::TwoDigit => &TwoDigit,
			Self::ZeroPad => &ZeroPad,
			Self::Float => &Float,

			Self::TitleFirst => &TitleFirst,
			Self::TitleAll => &TitleAll,
			Self::LowerFirst => &LowerFirst,
			Self::LowerAll => &LowerAll,

			Self::If => &If,
			Self::IfSelf => &IfSelf,
			Self::Switch => &Switch,

			Self::SetTime => &SetTime,
			Self::SetResetTime => &SetResetTime,

			// If excel is not available, explicitly NoOp all of these - they commonly
			// contain strings that fallback logic would pick up on errnoeously.
			Self::Sheet => &NoOp,
			Self::AutoTranslate => &NoOp,
			Self::NounJa => &NoOp,
			Self::NounEn => &NoOp,
			Self::NounDe => &NoOp,
			Self::NounFr => &NoOp,
			Self::NounZh => &NoOp,

			_ => &Fallback,
	}
}
