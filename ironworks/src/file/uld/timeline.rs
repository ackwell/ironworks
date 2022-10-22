use binrw::binread;

use super::shared::{ByteString, ToDo};

#[binread]
#[br(little)]
#[br(import(magic: ByteString<4>, _version: ByteString<4>))]
#[br(pre_assert(
	&magic == b"tlhd",
	"incorrect magic, expected b\"tlhd\", got {:?}",
	magic
))]
#[derive(Debug)]
pub struct Timeline {
	id: u32,

	#[br(temp)]
	size: u32,

	#[br(temp)]
	frame_counts: [u16; 2],

	#[br(count = frame_counts[0])]
	frames_1: Vec<Frame>,

	#[br(count = frame_counts[1])]
	frames_2: Vec<Frame>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Frame {
	start: u32,
	end: u32,

	#[br(temp)]
	size: u32,

	#[br(temp)]
	count: u32,

	#[br(count = count)]
	key_groups: Vec<KeyGroup>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct KeyGroup {
	usage: KeyGroupUsage,

	#[br(temp)]
	kind: u16,

	#[br(temp)]
	size: u16,

	#[br(temp)]
	count: u16,

	#[br(pad_size_to = size - 8)]
	#[br(args {
    count: count.into(),
    inner: (kind,)
  })]
	key_frames: Vec<KeyFrame>,
}

#[binread]
#[br(little, repr = u16)]
#[derive(Debug)]
enum KeyGroupUsage {
	Position = 0,
	Rotation = 1,
	Scale = 2,
	Alpha = 3,
	NodeColor = 4,
	TextColor = 5,
	EdgeColor = 6,
	Number = 7,
}

#[binread]
#[br(little, import(kind: u16))]
#[derive(Debug)]
struct KeyFrame {
	time: u32,
	size: u16,
	interpolation: u8,
	unknown_1: u8,
	acceleration: f32,
	deceleration: f32,

	#[br(args(kind))]
	data: KeyFrameData,
}

#[binread]
#[br(little, import(kind: u16))]
#[derive(Debug)]
enum KeyFrameData {
	#[br(pre_assert(kind == 0))]
	F32_1(f32),

	#[br(pre_assert(kind == 1))]
	F32_2([f32; 2]),

	#[br(pre_assert(kind == 2))]
	F32_3([f32; 3]),

	#[br(pre_assert(kind == 3))]
	I8_1(i8),

	#[br(pre_assert(kind == 4))]
	I8_2([i8; 2]),

	#[br(pre_assert(kind == 5))]
	I8_3([i8; 3]),

	#[br(pre_assert(kind == 6))]
	U8_1(u8),

	#[br(pre_assert(kind == 7))]
	U8_2([u8; 2]),

	#[br(pre_assert(kind == 8))]
	U8_3([u8; 3]),

	#[br(pre_assert(kind == 9))]
	I16_1(i16),

	#[br(pre_assert(kind == 10))]
	I16_2([i16; 2]),

	#[br(pre_assert(kind == 11))]
	I16_3([i16; 3]),

	#[br(pre_assert(kind == 12))]
	U16_1(u16),

	#[br(pre_assert(kind == 13))]
	U16_2([u16; 2]),

	#[br(pre_assert(kind == 14))]
	U16_3([u16; 3]),

	#[br(pre_assert(kind == 15))]
	I32_1(i32),

	#[br(pre_assert(kind == 16))]
	I32_2([i32; 2]),

	#[br(pre_assert(kind == 17))]
	I32_3([i32; 3]),

	#[br(pre_assert(kind == 18))]
	U32_1(u32),

	#[br(pre_assert(kind == 19))]
	U32_2([u32; 2]),

	#[br(pre_assert(kind == 20))]
	U32_3([u32; 3]),

	#[br(pre_assert(kind == 21))]
	Bool1(#[br(map = |value: u8| value != 0)] bool),

	#[br(pre_assert(kind == 22))]
	Bool2(#[br(map = |values:[u8; 2]| [values[0] != 0, values[1] != 0])] [bool; 2]),

	#[br(pre_assert(kind == 23))]
	Bool3(#[br(map = |values:[u8; 3]| [values[0] != 0, values[1] != 0, values[2] != 0])] [bool; 3]),

	#[br(pre_assert(kind == 24))]
	Color {
		multiply_red: i16,
		multiply_green: i16,
		multiply_blue: i16,
		add_red: i16,
		add_green: i16,
		add_blue: i16,
	},

	#[br(pre_assert(kind == 25))]
	Label {
		id: u16,
		command: u8,
		jump: u8,
	},

	Unknown(#[br(args("key group", kind.into()))] ToDo),
}
