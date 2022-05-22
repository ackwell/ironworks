// TODO: remove
#![allow(dead_code)]

use binrw::binread;

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Material {
	// Container header
	version: u32,
	file_size: u16,
	#[br(temp)]
	data_set_size: u16,
	#[br(temp)]
	string_table_size: u16,
	shader_package_name_offset: u16,
	#[br(temp)]
	texture_count: u8,
	#[br(temp)]
	uv_set_count: u8,
	#[br(temp)]
	color_set_count: u8,
	#[br(temp)]
	additional_data_size: u8,

	#[br(count = texture_count)]
	pub texture_offsets: Vec<TextureOffset>,

	#[br(count = uv_set_count)]
	uv_color_sets: Vec<UvColorSet>,

	// TODO: i32? really?
	#[br(count = color_set_count)]
	color_set_offsets: Vec<i32>,

	// TODO: can this be eagerly resolved?
	#[br(
    count = string_table_size,
	  // TODO: unknown, seems to be a struct of some kind
    pad_after = additional_data_size,
  )]
	pub string_data: Vec<u8>,

	// TODO: Check this info, stems from TT
	#[br(if(data_set_size > 0))]
	color_set_info: Option<[u16; 256]>,
	#[br(if(data_set_size > 512))]
	color_set_dye_info: Option<[u16; 16]>,

	// Material header
	#[br(temp)]
	shader_value_list_size: u16,
	#[br(temp)]
	shader_key_count: u16,
	#[br(temp)]
	constant_count: u16,
	#[br(temp)]
	sampler_count: u16,
	unknown1: u16,
	unknown2: u16,

	#[br(count = shader_key_count)]
	shader_keys: Vec<ShaderKey>,

	#[br(count = constant_count)]
	constants: Vec<Constant>,

	#[br(count = sampler_count)]
	pub samplers: Vec<Sampler>,

	#[br(count = shader_value_list_size / 4)]
	shader_values: Vec<f32>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct TextureOffset {
	pub offset: u16,
	// TODO: Unknown if actually flags.
	flags: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct UvColorSet {
	name_offset: u16,
	index: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct ShaderKey {
	category: u32,
	value: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Constant {
	constant_id: u32,
	value_offset: u16,
	value_size: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Sampler {
	pub id: u32,
	// TODO: bitfield
	pub state: u32,
	#[br(pad_after = 3)]
	pub texture_index: u8,
	// padding: [u8; 3].
}
