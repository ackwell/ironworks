use binrw::binread;

use super::{node::Node, shared::ByteString};

#[binread]
#[br(little)]
#[br(import(magic: ByteString<4>, _version: ByteString<4>))]
#[br(pre_assert(
	&magic == b"wdhd",
	"incorrect magic, expected b\"wdhd\", got {:?}",
	magic
))]
#[derive(Debug)]
pub struct Widget {
	id: u32,
	// TODO: Alignment is documented as an i32 enum, but there's a bunch of widgets cropping up with the 9th bit set, which pushes the value way out of documented range. I'd deem it unlikely that 9-255 all have values, so I'm making an assumtion here that alignment is actually a u8, and the remaining bytes are unknown fields or padding.
	alignment: Alignment,
	unknown_1: [u8; 3],
	x: i16,
	y: i16,
	#[br(temp)]
	count: u16,
	#[br(temp)]
	size: u16,

	#[br(pad_size_to = size - 16)]
	#[br(count = count)]
	nodes: Vec<Node>,
}

// TODO: this is duplicated with node's alignment. consolidate, depending on resolution of todo above?
#[binread]
#[br(little, repr = u8)]
#[derive(Debug)]
enum Alignment {
	TopLeft = 0,
	TopMiddle = 1,
	TopRight = 2,
	MiddleLeft = 3,
	Center = 4,
	MiddleRight = 5,
	BottomLeft = 6,
	BottomMiddle = 7,
	BottomRight = 8,
}
