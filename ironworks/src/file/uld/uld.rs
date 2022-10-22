use std::io::SeekFrom;

use binrw::{binread, BinRead, NullString, PosValue};

use crate::{error::Result, file::File, FileStream};

use super::{component::Component, node::Node, shared::ToDo};

#[binread]
#[br(little, magic = b"uldh")]
#[derive(Debug)]
pub struct UiLayout {
	#[br(count = 4)]
	#[br(try_map = String::from_utf8)]
	version: String,

	#[br(temp)]
	addon_1_offset: u32,

	#[br(temp)]
	addon_2_offset: u32,

	#[br(seek_before = SeekFrom::Start(addon_1_offset.into()))]
	addon_1: Addon,

	#[br(seek_before = SeekFrom::Start(addon_2_offset.into()))]
	addon_2: Addon,
}

impl File for UiLayout {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut stream)?)
	}
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Addon {
	#[br(temp)]
	start: PosValue<()>,

	#[br(count = 4)]
	#[br(try_map = String::from_utf8)]
	#[br(assert(magic == "atkh"))]
	magic: String,

	#[br(count = 4)]
	#[br(try_map = String::from_utf8)]
	version: String,

	#[br(temp)]
	assets_offset: u32,

	#[br(temp)]
	parts_offset: u32,

	#[br(temp)]
	components_offset: u32,

	#[br(temp)]
	timelines_offset: u32,

	#[br(temp)]
	widgets_offset: u32,

	overwrite_data_offset: u32,
	timeline_num: u32,

	#[br(if(assets_offset > 0))]
	#[br(seek_before = SeekFrom::Start(start.pos + u64::from(assets_offset)))]
	assets: Option<Section<Asset>>,

	#[br(if(parts_offset > 0))]
	#[br(seek_before = SeekFrom::Start(start.pos + u64::from(parts_offset)))]
	parts: Option<Section<Parts>>,

	#[br(if(components_offset > 0))]
	#[br(seek_before = SeekFrom::Start(start.pos + u64::from(components_offset)))]
	components: Option<Section<Component>>,

	#[br(if(timelines_offset > 0))]
	#[br(seek_before = SeekFrom::Start(start.pos + u64::from(timelines_offset)))]
	timelines: Option<Section<Timeline>>,

	#[br(if(widgets_offset > 0))]
	#[br(seek_before = SeekFrom::Start(start.pos + u64::from(widgets_offset)))]
	widgets: Option<Section<Widget>>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Section<T: BinRead<Args = ([u8; 4], [u8; 4])>> {
	magic: [u8; 4],
	version: [u8; 4],

	#[br(pad_after = 4)]
	#[br(temp)]
	count: u32,

	#[br(args {
		count: count.try_into().unwrap(),
		inner: (magic, version)
	})]
	values: Vec<T>,
}

#[binread]
#[br(little)]
#[br(import(magic: [u8; 4], _version: [u8; 4]))]
#[br(pre_assert(
	&magic == b"ashd",
	"incorrect magic, expected \"ashd\", got \"{}\"",
	std::str::from_utf8(&magic).unwrap()
))]
#[derive(Debug)]
struct Asset {
	id: u32,

	// TODO: is it safe to assume that it's a nullstring or will a 44-char path be un-nulled
	#[br(pad_size_to = 44)]
	filename: NullString,

	db_id: u32,

	unk: i32,
}

#[binread]
#[br(little)]
#[br(import(magic: [u8; 4], _version: [u8; 4]))]
#[br(pre_assert(
	&magic == b"tphd",
	"incorrect magic, expected \"tphd\", got \"{}\"",
	std::str::from_utf8(&magic).unwrap()
))]
#[derive(Debug)]
struct Parts {
	id: u32,
	part_num: u32,
	offset: u32,
	#[br(count = part_num)]
	parts: Vec<Part>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Part {
	texture_id: u32,
	u: u16,
	v: u16,
	w: u16,
	h: u16,
}

#[binread]
#[br(little)]
#[br(import(magic: [u8; 4], _version: [u8; 4]))]
#[br(pre_assert(
	&magic == b"tlhd",
	"incorrect magic, expected \"tlhd\", got \"{}\"",
	std::str::from_utf8(&magic).unwrap()
))]
#[derive(Debug)]
struct Timeline {
	id: u32,
	offset: u32,
	frame_count: [u16; 2],

	#[br(count = frame_count[0])]
	frames_1: Vec<Frame>,
	#[br(count = frame_count[1])]
	frames_2: Vec<Frame>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Frame {
	start: u32,
	end: u32,
	offset: u32,
	key_group_count: u32,
	#[br(count = key_group_count)]
	key_groups: Vec<KeyGroup>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct KeyGroup {
	// these are enums
	usage: u16,
	kind: u16,
	offset: u16,
	count: u16, // 8

	#[br(pad_size_to = offset - 8)]
	#[br(args(kind))]
	data: KeyGroupData,
}

#[binread]
#[br(little, import(kind: u16))]
#[derive(Debug)]
enum KeyGroupData {
	Todo(#[br(args("key group", kind.into()))] ToDo),
}

#[binread]
#[br(little)]
#[br(import(magic: [u8; 4], _version: [u8; 4]))]
#[br(pre_assert(
	&magic == b"wdhd",
	"incorrect magic, expected \"wdhd\", got \"{}\"",
	std::str::from_utf8(&magic).unwrap()
))]
#[derive(Debug)]
struct Widget {
	id: u32,
	align_type: i32, // enum
	x: i16,
	y: i16,
	node_num: u16,
	offset: u16,

	#[br(count = node_num)]
	nodes: Vec<Node>,
}
