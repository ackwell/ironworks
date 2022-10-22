use binrw::{binread, NullString};
use modular_bitfield::{bitfield, specifiers::*};

use super::shared::ByteString;

#[binread]
#[br(little)]
#[br(import(magic: ByteString<4>, version: ByteString<4>))]
#[br(pre_assert(
	&magic == b"ashd",
	"incorrect magic, expected b\"ashd\", got {:?}",
	magic
))]
#[derive(Debug)]
pub struct Asset {
	id: u32,

	// TODO: is it safe to assume that it's a nullstring or will a 44-char path be un-nulled
	#[br(pad_size_to = 44)]
	filename: NullString,

	icon_id: u32,

	// TODO: Do I want to effectively "upgrade" all assets to have themes, or make this optional?
	#[br(if(&version == b"0101"))]
	themes: AssetThemes,
}

#[bitfield]
#[binread]
#[br(map = Self::from_bytes)]
#[derive(Debug, Default)]
struct AssetThemes {
	light: bool,
	classic: bool,
	// NOTE: this bit is set on a lot of the heavily-themed assets, but there's no folder for a fourth theme. WIP unreleased theme?
	unknown: bool,
	#[skip]
	reserved: B29,
}
