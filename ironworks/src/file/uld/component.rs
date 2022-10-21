use binrw::binread;

use super::{node::Node, shared::ToDo};

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Component {
	id: u32,
	// these u8 seem fine? check. they should be boolean eod
	ignore_input: u8,
	arrow_drag: u8,
	arrow_drop: u8,

	kind: u8,
	node_num: u32,
	size: u16,
	offset: u16,

	#[br(pad_size_to = offset - 16)]
	#[br(args(kind))]
	data: ComponentData,

	#[br(count = node_num)]
	nodes: Vec<Node>,
}

// TODO: do i want to put the data in seperate structs?
#[binread]
#[br(little, import(kind: u8))]
#[derive(Debug)]
enum ComponentData {
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
	Todo(#[br(args("component", kind.into()))] ToDo),
}
