use std::io::SeekFrom;

use binrw::{binread, BinRead, PosValue};

use crate::{error::Result, file::File, FileStream};

use super::{
	asset::Asset, component::Component, node::Node, parts::Parts, shared::ByteString,
	timeline::Timeline,
};

#[binread]
#[br(little, magic = b"uldh")]
#[derive(Debug)]
pub struct UiLayout {
	version: ByteString<4>,

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

	#[br(assert(&magic == b"atkh"))]
	magic: ByteString<4>,

	version: ByteString<4>,

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
struct Section<T: BinRead<Args = (ByteString<4>, ByteString<4>)>> {
	magic: ByteString<4>,
	version: ByteString<4>,

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
#[br(import(magic: ByteString<4>, _version: ByteString<4>))]
#[br(pre_assert(
	&magic == b"wdhd",
	"incorrect magic, expected b\"wdhd\", got {:?}",
	magic
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
