use binrw::binread;
use modular_bitfield::{bitfield, specifiers::*};

use super::shared::{U8Bool, Unknown};

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Node {
	id: u32,
	// why are these i32? i'm guessing -1 to signify none?
	parent_id: i32,
	next_sibling_id: i32,
	previous_sibling_id: i32,
	child_id: i32, // probably first_child_id?
	#[br(temp)]
	kind: u32,
	size: u16,
	tab_index: i16,
	unknown_1: [i32; 4],
	x: i16,
	y: i16,
	width: u16,
	height: u16,
	rotation: f32,
	scale_x: f32,
	scale_y: f32,
	origin_x: i16,
	origin_y: i16,
	priority: u16,
	attributes: NodeAttributes,
	unknown_2: u8,
	// todo: structs for these maybe?
	multiply_r: i16,
	multiply_g: i16,
	multiply_b: i16,
	add_r: i16,
	add_g: i16,
	add_b: i16,
	alpha: u8,
	clip_count: i8,
	timeline_id: u16,

	#[br(pad_size_to = size - 88)]
	#[br(args(kind))]
	data: NodeData,
}

#[bitfield]
#[binread]
#[br(map = Self::from_bytes)]
#[derive(Debug)]
struct NodeAttributes {
	visible: bool,
	enabled: bool,
	clip: bool,
	fill: bool,
	anchor_top: bool,
	anchor_bottom: bool,
	anchor_left: bool,
	anchor_right: bool,
}

#[binread]
#[br(little, import(kind: u32))]
#[derive(Debug)]
enum NodeData {
	#[br(pre_assert(kind == 1))]
	None,

	#[br(pre_assert(kind == 2))]
	Image {
		parts_id: u32,
		part_id: u32,
		flip_horizontal: U8Bool,
		flip_vertical: U8Bool,
		wrap_mode: u8,
		blend_mode: u8, // unknown?
	},

	#[br(pre_assert(kind == 3))]
	Text {
		text_id: u32,
		text_color: u32,
		text_align: TextAlignment,
		font_type: TextFont,
		font_size: u8,
		edge_color: u32, // "glow in stc parlance?"
		attributes: TextAttributes,
		sheet: TextSheet,
		character_spacing: i8,
		line_spacing: u8,
		// seems first bit is a flag and then 31 bits of padding?
		flags2: u32, // unknown?
	},

	#[br(pre_assert(kind == 4))]
	NineGrid {
		parts_id: u32,
		part_id: u32,
		parts_kind: NineGridPartsKind,
		render_kind: NineGridRenderKind,
		offset_top: i16,
		offset_bottom: i16,
		offset_left: i16,
		offset_right: i16,
		blend_mode: u8, // unknown?
		padding: u8,    // unknown?
	},

	#[br(pre_assert(kind == 5))]
	Counter {
		parts_id: u32,
		part_id: u8,
		number_width: u8,
		comma_width: u8,
		space_width: u8,
		text_align: u16,
		unknown_1: u16,
	},

	#[br(pre_assert(kind == 8))]
	Collision {
		kind: CollisionKind,
		unknown_1: u16,
		x: i32,
		y: i32,
		radius: u32,
	},

	#[br(pre_assert(kind >= 1000))]
	Component(#[br(args(kind))] Component),

	Unknown(#[br(args("node", kind.into()))] Unknown),
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
enum TextFont {
	Axis = 0,
	MeidingerMedium = 1,
	Meidinger = 2,
	TrumpGothic = 3,
	Jupiter = 4,
	JupiterLarge = 5,
}

#[binread]
#[br(repr = u16)]
#[derive(Debug)]
enum TextAlignment {
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

#[bitfield]
#[binread]
#[br(map = Self::from_bytes)]
#[derive(Debug)]
struct TextAttributes {
	bold: bool,
	italic: bool,
	edge: bool,
	glare: bool,
	multi_line: bool,
	ellipsis: bool,
	paragraph: bool,
	emboss: bool,
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
enum TextSheet {
	Addon = 0,
	Lobby = 1,
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
enum NineGridPartsKind {
	Divide = 0,
	Compose = 1,
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
enum NineGridRenderKind {
	Scale = 0,
	Tile = 1,
}

#[binread]
#[br(little, repr = u16)]
#[derive(Debug)]
enum CollisionKind {
	Hit = 0,
	Focus = 1,
	Move = 2,
}

#[binread]
#[br(little, import(kind: u32))]
#[derive(Debug)]
struct Component {
	#[br(calc = kind)]
	component_id: u32,

	index: u8,
	up: u8,
	down: u8,
	left: u8,
	right: u8,
	cursor: u8,
	attributes: ComponentAttributes,
	unknown_1: u8,
	offset_x: i16,
	offset_y: i16,
	// ... oh you're fucking kidding me. remaining data here depends on the component's kind as referenced by component_id. hellscape.
}

#[bitfield]
#[binread]
#[br(map = Self::from_bytes)]
#[derive(Debug)]
struct ComponentAttributes {
	repeat_up: bool,
	repeat_down: bool,
	repeat_left: bool,
	repeat_right: bool,
	unknown: B4,
}
