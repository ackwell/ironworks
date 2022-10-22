use binrw::binread;

use super::{
	node::Node,
	shared::{ByteString, U8Bool, Unknown},
};

#[binread]
#[br(little)]
#[br(import(magic: ByteString<4>, _version: ByteString<4>))]
#[br(pre_assert(
	&magic == b"cohd",
	"incorrect magic, expected b\"cohd\", got {:?}",
	magic
))]
#[derive(Debug)]
pub struct Component {
	id: u32,
	ignore_input: U8Bool,
	arrow_drag: U8Bool,
	arrow_drop: U8Bool,
	kind: u8,
	count: u32,
	// Total size of the component including child nodes.
	size: u16,
	// Size of the component excluding nodes.
	data_size: u16,

	#[br(pad_size_to = data_size - 16)]
	#[br(args(kind))]
	data: ComponentData,

	#[br(pad_size_to = size - data_size)]
	#[br(count = count)]
	nodes: Vec<Node>,
}

// TODO: How should I represent nodes? I don't think I can reasonably eagerly resolve them. Leave it as a consumer concern?
// TODO: 0 implies no linked node - should i use NonZero variants and translate 0 into an option to better represent the lack of node?
type NodeReference = u32;

// TODO: do i want to put the data in seperate structs?
#[binread]
#[br(little, import(kind: u8))]
#[derive(Debug)]
enum ComponentData {
	#[br(pre_assert(kind == 0))]
	Custom,

	#[br(pre_assert(kind == 1))]
	Button {
		content_node: NodeReference,
		background_image_node: NodeReference,
	},

	#[br(pre_assert(kind == 2))]
	Window {
		unknown_nodes: [NodeReference; 8],
	},

	#[br(pre_assert(kind == 3))]
	CheckButton {
		// TODO: this shares structure with button - reuse?
		content_node: NodeReference,
		background_image_node: NodeReference,
		unknown_nodes: [NodeReference; 1],
	},

	#[br(pre_assert(kind == 4))]
	RadioButton {
		unknown_nodes: [NodeReference; 4],
	},

	#[br(pre_assert(kind == 5))]
	GaugeBar {
		unknown_nodes: [NodeReference; 6],
		vertical_margin: u16,
		horizontal_margin: u16,
		#[br(pad_after = 3)]
		is_vertical: U8Bool,
	},

	#[br(pre_assert(kind == 6))]
	Slider {
		unknown_nodes: [NodeReference; 4],
		is_vertical: U8Bool,
		left_offset: u8,
		#[br(pad_after = 1)]
		right_offset: u8,
	},

	#[br(pre_assert(kind == 7))]
	TextInput {
		unknown_nodes: [NodeReference; 16],
		color: u32,
		ime_color: i32,
		unknown_1: u32,
	},

	#[br(pre_assert(kind == 8))]
	NumericInput {
		unknown_nodes: [NodeReference; 5],
		color: u32,
	},

	#[br(pre_assert(kind == 9))]
	List {
		unknown_nodes: [NodeReference; 5],
		wrap: u8,
		#[br(pad_after = 2)]
		orientation: u8,
	},

	#[br(pre_assert(kind == 10))]
	DropDown {
		unknown_nodes: [NodeReference; 2],
	},

	#[br(pre_assert(kind == 11))]
	Tab {
		unknown_nodes: [NodeReference; 4],
	},

	#[br(pre_assert(kind == 12))]
	TreeList {
		// This is shared with list?
		unknown_nodes: [NodeReference; 5],
		wrap: u8,
		#[br(pad_after = 2)]
		orientation: u8,
	},

	#[br(pre_assert(kind == 13))]
	ScrollBar {
		nodes: [u32; 4],
		margin: u16,
		#[br(pad_after = 1)]
		is_vertical: U8Bool,
	},

	#[br(pre_assert(kind == 14))]
	ListItem {
		unknown_nodes: [NodeReference; 4],
	},

	#[br(pre_assert(kind == 15))]
	Icon {
		unknown_nodes: [NodeReference; 8],
	},

	#[br(pre_assert(kind == 16))]
	IconText {
		unknown_nodes: [NodeReference; 2],
	},

	#[br(pre_assert(kind == 17))]
	DragDrop {
		unknown_nodes: [NodeReference; 1],
	},

	#[br(pre_assert(kind == 18))]
	GuildLeveCard {
		unknown_nodes: [NodeReference; 3],
	},

	#[br(pre_assert(kind == 19))]
	TextNineGrid {
		unknown_nodes: [NodeReference; 2],
	},

	#[br(pre_assert(kind == 20))]
	JournalCanvas {
		unknown_nodes: [NodeReference; 32],
		margin: u16,
		unknown_1: u16,
		#[br(pad_after = 2)]
		unknown_2: u16,
	},

	#[br(pre_assert(kind == 21))]
	MultiPurpose {
		unknown_nodes: [NodeReference; 3],
	},

	#[br(pre_assert(kind == 22))]
	Map {
		unknown_nodes: [NodeReference; 10],
	},

	#[br(pre_assert(kind == 23))]
	Preview {
		unknown_nodes: [NodeReference; 2],
	},

	#[br(pre_assert(kind == 24))]
	HoldButton {
		unknown_nodes: [NodeReference; 4],
	},

	#[br(pre_assert(kind == 25))]
	CharacterCard {
		unknown_nodes: [NodeReference; 3],
	},

	Unknown(#[br(args("component", kind.into()))] Unknown),
}
