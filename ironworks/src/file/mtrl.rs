//! Structs and utilities for parsing .mtrl files.

// TODO: remove
#![allow(missing_docs)]

use std::{borrow::Cow, io::Cursor};

use binrw::{binread, BinRead};

use crate::error::Result;

use super::file::File;

// TODO: .mtrl files are a container-esque format too, consider if should be handling that akin to .mdl

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
	texture_offsets: Vec<TextureOffset>,

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
	string_data: Vec<u8>,

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
	samplers: Vec<Sampler>,

	#[br(count = shader_value_list_size / 4)]
	shader_values: Vec<f32>,
}

impl File for Material {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut Cursor::new(data.into()))?)
	}
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct TextureOffset {
	offset: u16,
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
	value_sizze: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Sampler {
	sampler_id: u32,
	// TODO: Unknown bitfield
	flags: u32,
	#[br(pad_after = 3)]
	texture_index: u8,
	// padding: [u8; 3].
}
