use std::io::SeekFrom;

use binrw::{binread, BinRead, NullString, PosValue};
use modular_bitfield::prelude::*;

use crate::{error::Result, FileStream};

use super::file::File;

// TODO: restructure this stuff to put headers at actual appropriate locations and so forth

#[binread]
#[br(little, magic = b"uldh")]
#[derive(Debug)]
pub struct UiLayout {
	#[br(count = 4)]
	#[br(try_map = String::from_utf8)]
	version: String,
	component_offset: u32,
	widget_offset: u32,

	// TODO: this should seek based on component_offset
	#[br(temp)]
	component_header_start: PosValue<()>,
	component_header: ComponentHeader,

	// assets
	#[br(seek_before = SeekFrom::Start(component_header_start.pos + u64::from(component_header.asset_list_offset)))]
	asset_list: PartHeader,
	#[br(count = asset_list.data_num)]
	asset_data: Vec<AssetData>,

	// "parts"? - looks like it's a list of metadata, rects of textures to use
	#[br(seek_before = SeekFrom::Start(component_header_start.pos + u64::from(component_header.parts_list_offset)))]
	part_list: PartHeader,
	#[br(count = part_list.data_num)]
	parts: Vec<PartsData>,

	// components
	#[br(seek_before = SeekFrom::Start(component_header_start.pos + u64::from(component_header.component_list_offset)))]
	component_list: PartHeader,
	#[br(count = component_list.data_num)]
	components: Vec<ComponentData>,

	// timelines
	#[br(seek_before = SeekFrom::Start(component_header_start.pos + u64::from(component_header.timeline_list_offset)))]
	timeline_list: PartHeader,
	#[br(count = timeline_list.data_num)]
	timelines: Vec<TimelineData>,

	// widgets? combinewidget?
	widget_header_start: PosValue<()>,
	#[br(seek_before = SeekFrom::Start(widget_offset.into()))]
	widget_header: ComponentHeader,

	#[br(seek_before = SeekFrom::Start(widget_header_start.pos + u64::from(widget_header.widget_offset)))]
	widgets_list: PartHeader,
	#[br(count = widgets_list.data_num)]
	widgets: Vec<WidgetData>,
}

impl File for UiLayout {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut stream)?)
	}
}

#[binread]
#[br(little, magic = b"atkh")]
#[derive(Debug)]
struct ComponentHeader {
	// TODO: probably should make a 4-char-string type because this is silly
	#[br(count = 4)]
	#[br(try_map = String::from_utf8)]
	version: String,
	asset_list_offset: u32,
	parts_list_offset: u32,
	component_list_offset: u32,
	timeline_list_offset: u32,
	widget_offset: u32,
	overwrite_data_offset: u32,
	timeline_num: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct PartHeader {
	#[br(count = 4)]
	#[br(try_map = String::from_utf8)]
	id: String,

	#[br(count = 4)]
	#[br(try_map = String::from_utf8)]
	version: String,

	#[br(pad_after = 4)]
	data_num: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct AssetData {
	id: u32,

	// TODO: is it safe to assume that it's a nullstring or will a 44-char path be un-nulled
	#[br(pad_size_to = 44)]
	filename: NullString,

	db_id: u32,

	unk: i32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct PartsData {
	id: u32,
	part_num: u32,
	offset: u32,
	#[br(count = part_num)]
	parts: Vec<PartData>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct PartData {
	texture_id: u32,
	u: u16,
	v: u16,
	w: u16,
	h: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct ComponentData {
	id: u32,
	// these u8 seem fine? check. they should be boolean eod
	ignore_input: u8,
	arrow_drag: u8,
	arrow_drop: u8,
	// kind: ComponentKind,
	kind: u8,
	node_num: u32,
	size: u16,
	offset: u16,
	// magic kind shit?
	#[br(pad_size_to = offset - 16)]
	#[br(args(kind))]
	data: ComponentExtendedData,

	#[br(count = node_num)]
	nodes: Vec<NodeData>,
}

// TODO: better name. componentdata should probably be component, and this component data
// TODO: do i want to put the data in seperate structs?
// todo: offset is for todo
#[binread]
#[br(little, import(kind: u8))]
#[derive(Debug)]
enum ComponentExtendedData {
	#[br(pre_assert(kind == 0))]
	Custom,

	#[br(pre_assert(kind == 1))]
	Button {
		nodes: [u32; 2],
	},

	// #[br(pre_assert(kind == 2))]
	// Window,

	// #[br(pre_assert(kind == 3))]
	// CheckButton,

	// #[br(pre_assert(kind == 4))]
	// RadioButton,

	// #[br(pre_assert(kind == 5))]
	// GaugeBar,

	// #[br(pre_assert(kind == 6))]
	// Slider,

	// #[br(pre_assert(kind == 7))]
	// TextInput,

	// #[br(pre_assert(kind == 8))]
	// NumericInput,

	// #[br(pre_assert(kind == 9))]
	// TreeList,

	// #[br(pre_assert(kind == 10))]
	// DropDown,

	// #[br(pre_assert(kind == 11))]
	// Tab,

	// #[br(pre_assert(kind == 12))]
	// TreeList,
	#[br(pre_assert(kind == 13))]
	ScrollBar {
		nodes: [u32; 4],
		margin: u16,
		vertical: u8, //bool?
		padding: u8,
	},

	// #[br(pre_assert(kind == 14))]
	// ListItem,

	// #[br(pre_assert(kind == 15))]
	// Icon,

	// #[br(pre_assert(kind == 16))]
	// IconText,

	// #[br(pre_assert(kind == 17))]
	// DragDrop,

	// #[br(pre_assert(kind == 18))]
	// GuildleveCard,

	// #[br(pre_assert(kind == 19))]
	// TextNineGrid,

	// #[br(pre_assert(kind == 20))]
	// JournalCanvas,

	// #[br(pre_assert(kind == 21))]
	// MultiPurpose,

	// #[br(pre_assert(kind == 22))]
	// Map,

	// #[br(pre_assert(kind == 23))]
	// Preview,
	Todo(#[br(args("component", kind.into()))] Todo),
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct NodeData {
	id: u32,
	// why are these i32? i'm guessing -1 to signify none?
	parent_id: i32,
	older_id: i32,
	younger_id: i32,
	child_id: i32,
	// this seems to be a data tag
	kind: i32,
	offset: u16,
	tab_index: i16,
	navigation_id: [i32; 4],
	x: i16,
	y: i16,
	w: u16,
	h: u16,
	rot: f32,
	scale_x: f32,
	scale_y: f32,
	origin_x: i16,
	origin_y: i16,
	priority: u16,
	attributes: NodeDataAttributes,
	// todo: structs for these maybe?
	mul_r: i16,
	mul_g: i16,
	mul_b: i16,
	add_r: i16,
	add_g: i16,
	add_b: i16,
	alpha: u8,
	clip_count: i8,
	timeline_id: u16,

	#[br(pad_size_to = offset - 88)]
	#[br(args(kind))]
	data: NodeExtendedData,
}

#[bitfield]
#[binread]
#[br(map = Self::from_bytes)]
#[derive(Debug)]
struct NodeDataAttributes {
	visible: bool,
	enable: bool,
	clip: bool,
	fill: bool,
	anchor_top: bool,
	anchor_bottom: bool,
	anchor_left: bool,
	anchor_right: bool,
	is_hit: bool,
	#[skip]
	reserved: B7,
}

// TODO: same shit about naming
// TODO: remove offset, it's just there for the todo
#[binread]
#[br(little, import(kind: i32))]
#[derive(Debug)]
enum NodeExtendedData {
	#[br(pre_assert(kind == 1))]
	None,

	#[br(pre_assert(kind == 2))]
	Image {
		parts_id: u32,
		part_id: u32,
		// these two are bools
		h_flip: u8,
		v_flip: u8,
		wrap_mode: u8,
		blend_mode: u8,
	},

	#[br(pre_assert(kind == 3))]
	Text {
		text_id: u32,
		text_color: u32,
		text_align: u16,
		font_type: u8, // this is an enum
		font_size: u8,
		edge_color: u32, // "glow in stc parlance?"
		// TODO: BITFIELD SHITTERY
		// bold
		// italic
		// edge
		// glare
		// multiline
		// ellipsis
		// paragraph
		// emboss
		flags: u8,
		sheet_kind: u8,
		char_spacing: i8,
		line_spacing: u8,
		// seems first bit is a flag and then 31 bits of padding?
		flags2: u32,
	},

	#[br(pre_assert(kind == 4))]
	NineGrid {
		parts_id: u32,
		part_id: u32,
		parts_type: u8,
		render_type: u8,
		offset_top: i16,
		offset_bottom: i16,
		offset_left: i16,
		offset_right: i16,
		blend_mode: u8,
		padding: u8,
	},

	#[br(pre_assert(kind == 5))]
	Counter {
		parts_id: u32,
		part_id: u8,
		num_w: u8,
		comma_w: u8,
		space_w: u8,
		text_align: u16,
		padding: u16,
	},

	#[br(pre_assert(kind == 8))]
	Collision {
		kind: u16,
		uses: u16,
		x: i32,
		y: i32,
		radius: u32,
	},

	Todo(#[br(args("node", kind.into()))] Todo),
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct TimelineData {
	id: u32,
	offset: u32,
	frame_count: [u16; 2],

	#[br(count = frame_count[0])]
	frame_data_1: Vec<FrameData>,
	#[br(count = frame_count[1])]
	frame_data_2: Vec<FrameData>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct FrameData {
	start: u32,
	end: u32,
	offset: u32,
	key_group_count: u32,
	#[br(count = key_group_count)]
	key_groups: Vec<KeyGroupData>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct KeyGroupData {
	// these are enums
	usage: u16,
	kind: u16,
	offset: u16,
	count: u16, // 8

	#[br(pad_size_to = offset - 8)]
	#[br(args(kind))]
	data: KeyGroupExtendedData,
}

#[binread]
#[br(little, import(kind: u16))]
#[derive(Debug)]
enum KeyGroupExtendedData {
	Todo(#[br(args("key group", kind.into()))] Todo),
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct WidgetData {
	id: u32,
	align_type: i32, // enum
	x: i16,
	y: i16,
	node_num: u16,
	offset: u16,

	#[br(count = node_num)]
	nodes: Vec<NodeData>,
}

#[derive(Debug)]
struct Todo {
	kind: &'static str,
	value: i64,
}
impl BinRead for Todo {
	type Args = (&'static str, i64);

	fn read_options<R: std::io::Read + std::io::Seek>(
		reader: &mut R,
		options: &binrw::ReadOptions,
		(kind, value): Self::Args,
	) -> binrw::BinResult<Self> {
		Ok(Self { kind, value })
	}
}
