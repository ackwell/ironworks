use binrw::binread;
use getset::CopyGetters;

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Material {
	// Container header
	pub version: u32,
	_file_size: u16,
	#[br(temp)]
	data_set_size: u16,
	#[br(temp)]
	string_table_size: u16,
	pub shader_package_name_offset: u16,
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
	_uv_color_sets: Vec<UvColorSet>,

	#[br(count = color_set_count)]
	_color_set_offsets: Vec<u32>,

	// TODO: can this be eagerly resolved?
	#[br(
    count = string_table_size,
	  // TODO: unknown, seems to be a struct of some kind
    pad_after = additional_data_size,
  )]
	pub string_data: Vec<u8>,

	// TODO: Check this info, stems from TT
	#[br(if(data_set_size > 0))]
	_color_set_info: Option<[u16; 256]>,
	#[br(if(data_set_size > 512))]
	_color_set_dye_info: Option<[u16; 16]>,

	// Material header
	#[br(temp)]
	shader_value_list_size: u16,
	#[br(temp)]
	shader_key_count: u16,
	#[br(temp)]
	constant_count: u16,
	#[br(temp)]
	sampler_count: u16,
	_unknown1: u16,
	_unknown2: u16,

	#[br(count = shader_key_count)]
	_shader_keys: Vec<ShaderKey>,

	#[br(count = constant_count)]
	_constants: Vec<Constant>,

	#[br(count = sampler_count)]
	pub samplers: Vec<Sampler>,

	#[br(count = shader_value_list_size / 4)]
	_shader_values: Vec<f32>,
}

// todo: actually u32?
#[binread]
#[br(little)]
#[derive(Debug)]
pub struct TextureOffset {
	pub offset: u16,
	// TODO: Unknown if actually flags.
	_flags: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct UvColorSet {
	_name_offset: u16,
	_index: u16,
}

#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
pub struct ShaderKey {
	_category: u32,
	_value: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Constant {
	_constant_id: u32,
	// These seem to be byte offsets into the shader values?
	// Size is a mult. of 4, seen 4 and 12 - assume that 12 is a vec3?
	_value_offset: u16,
	_value_size: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Sampler {
	pub id: u32,
	// TODO: bitfield, unknown fields.
	pub state: u32,
	#[br(pad_after = 3)]
	pub texture_index: u8,
	// padding: [u8; 3].
}
