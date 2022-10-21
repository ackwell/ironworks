use binrw::binread;
use modular_bitfield::{bitfield, specifiers::*};

use super::shared::ToDo;

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Node {
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
	attributes: NodeAttributes,
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
	data: NodeData,
}

#[bitfield]
#[binread]
#[br(map = Self::from_bytes)]
#[derive(Debug)]
struct NodeAttributes {
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
enum NodeData {
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

	Todo(#[br(args("node", kind.into()))] ToDo),
}
