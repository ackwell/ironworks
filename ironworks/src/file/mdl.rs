//! Structs and utilities for parsing .mdl files.

use std::io::{Cursor, Read, Seek};

use binrw::{binread, BinRead, BinResult, NullString, ReadOptions};
use modular_bitfield::bitfield;

use crate::error::Result;

use super::file::File;

const MAX_LODS: usize = 3;

// TODO: this is currently inlining a bunch of structures - look into if it's worth pulling it apart at all.
#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Model {
	// Model file header
	version: u32,
	stack_size: u32,
	runtime_size: u32,
	vertex_declaration_count: u16,
	material_count: u16,
	vertex_offset: [u32; MAX_LODS],
	index_offset: [u32; MAX_LODS],
	vertex_buffer_size: [u32; MAX_LODS],
	index_buffer_size: [u32; MAX_LODS],
	lod_count: u8,

	#[br(map = to_bool)]
	enable_index_buffer_streaming: bool,

	#[br(map = to_bool)]
	enable_edge_geometry: bool,

	// padding: u8

	// Loose data
	#[br(
    pad_before = 1,
    count = vertex_declaration_count,
  )]
	vertex_declarations: Vec<VertexDeclaration>,

	#[br(temp)]
	string_count: u16,
	// padding: u16,
	#[br(pad_before = 2, temp)]
	string_size: u32,
	#[br(
	  count = string_count,
    pad_size_to = string_size,
	)]
	strings: Vec<NullString>,

	// Model header
	// TODO: this has name conflicts with the file header - they seem to always be equiv, either skip one of them or break up the struct
	radius: f32,
	#[br(temp)]
	mesh_count: u16,
	#[br(temp)]
	attribute_count: u16,
	#[br(temp)]
	submesh_count: u16,
	#[br(temp)]
	material_count_2: u16,
	#[br(temp)]
	bone_count: u16,
	#[br(temp)]
	bone_table_count: u16,
	#[br(temp)]
	shape_count: u16,
	#[br(temp)]
	shape_mesh_count: u16,
	#[br(temp)]
	shape_value_count: u16,
	lod_count_2: u8,

	flags1: Flags1,

	#[br(temp)]
	element_id_count: u16,
	#[br(temp)]
	terrain_shadow_mesh_count: u8,

	flags2: Flags2,

	model_clip_out_distance: f32,
	shadow_clip_out_distance: f32,
	unknown4: u16,
	#[br(temp)]
	terrain_shadow_submesh_count: u16,
	unknown5: u8,
	bg_change_material_index: u8,
	bg_crest_change_material_index: u8,
	unknown6: u8,
	unknown7: u16,
	unknown8: u16,
	#[br(pad_after = 6)]
	unknown9: u16,

	// padding: [u8; 6],
	#[br(count = element_id_count)]
	element_ids: Vec<ElementId>,

	lods: [Lod; MAX_LODS],
	#[br(
    args(flags2.extra_lod_enabled()),
    parse_with = read_extra_lods,
  )]
	extra_lods: Option<[ExtraLod; MAX_LODS]>,

	#[br(count = mesh_count)]
	meshes: Vec<Mesh>,

	#[br(count = attribute_count)]
	attribute_name_offsets: Vec<u32>,

	#[br(count = terrain_shadow_mesh_count)]
	terrain_shadow_meshes: Vec<TerrainShadowMesh>,

	#[br(count = submesh_count)]
	submeshes: Vec<Submesh>,

	#[br(count = terrain_shadow_submesh_count)]
	terrain_shadow_submeshes: Vec<TerrainShadowSubmesh>,

	#[br(count = material_count_2)]
	material_name_offsets: Vec<u32>,

	#[br(count = bone_count)]
	bone_name_offsets: Vec<u32>,

	#[br(count = bone_table_count)]
	bone_tables: Vec<BoneTable>,

	#[br(count = shape_count)]
	shapes: Vec<Shape>,

	#[br(count = shape_mesh_count)]
	shape_meshes: Vec<ShapeMesh>,

	#[br(count = shape_value_count)]
	shape_values: Vec<ShapeValue>,

	#[br(temp)]
	submesh_bone_map_size: u32,
	#[br(count = submesh_bone_map_size / 2)]
	submesh_bone_map: Vec<u16>,

	// lmao what
	#[br(temp)]
	padding_size: u8,
	#[br(pad_before = padding_size)]
	bounding_boxes: BoundingBox,
	model_bounding_boxes: BoundingBox,
	water_bounding_boxes: BoundingBox,
	vertical_fog_bounding_boxes: BoundingBox,
	#[br(count = bone_count)]
	bone_bounding_boxes: Vec<BoundingBox>,
}

impl File for Model {
	fn read(data: Vec<u8>) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut Cursor::new(data))?)
	}
}

fn to_bool(value: u8) -> bool {
	println!("{value}, {}", value == 0);
	value != 0
}

#[derive(Debug)]
struct VertexDeclaration(Vec<VertexElement>);
impl BinRead for VertexDeclaration {
	type Args = ();

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: &ReadOptions,
		args: Self::Args,
	) -> BinResult<Self> {
		// There's always space for 17, but the element with stream == 255 and after are
		// invalid data - remove them.
		// TODO: This eagerly reads all 17 - can use parse_with and skip some reading.
		let raw = <[VertexElement; 17]>::read_options(reader, options, args)?;
		let filtered = raw
			.into_iter()
			.take_while(|element| element.stream != 255)
			.collect::<Vec<_>>();
		Ok(Self(filtered))
	}
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct VertexElement {
	// todo names
	stream: u8,
	offset: u8,
	type_: u8,
	usage: u8,
	#[br(pad_after = 3)]
	usage_index: u8,
}

#[bitfield]
#[binread]
#[derive(Debug)]
struct Flags1 {
	dust_occlusion_enabled: bool,
	show_occlusion_enabled: bool,
	rain_occlusion_enabled: bool,
	unknown1: bool,
	lighting_reflection_enabled: bool,
	waving_animation_disabled: bool,
	light_shadow_disabled: bool,
	shadow_disabled: bool,
}

#[bitfield]
#[binread]
#[derive(Debug)]
struct Flags2 {
	unknown2: bool,
	bg_uv_scroll_enabled: bool,
	enable_force_non_resident: bool,
	extra_lod_enabled: bool,
	shadow_mask_enabled: bool,
	force_lod_range_enabled: bool,
	edge_geometry_enabled: bool,
	unknown3: bool,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct ElementId {
	element_id: u32,
	// name?
	parent_bone_name: u32,
	translate: [f32; 3],
	rotate: [f32; 3],
}

// TODO: index/count pattern is super repetetive - abstract?
#[binread]
#[br(little)]
#[derive(Debug)]
struct Lod {
	mesh_index: u16,
	mesh_count: u16,
	model_lod_range: f32,
	texture_lod_range: f32,
	water_mesh_index: u16,
	water_mesh_count: u16,
	shadow_mesh_index: u16,
	shadow_mesh_count: u16,
	terrain_shadow_mesh_index: u16,
	terrain_shadow_mesh_count: u16,
	vertical_fog_mesh_index: u16,
	vertical_fog_mesh_count: u16,
	edge_geometry_size: u32,
	edge_geometry_data_offset: u32,
	polygon_count: u32,
	unknown1: u32,
	vertex_buffer_size: u32,
	index_buffer_size: u32,
	vertex_data_offset: u32,
	index_data_offset: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct ExtraLod {
	light_shaft_mesh_index: u16,
	light_shaft_mesh_count: u16,
	glass_mesh_index: u16,
	glass_mesh_count: u16,
	material_change_mesh_index: u16,
	material_change_mesh_count: u16,
	crest_change_mesh_index: u16,
	crest_change_mesh_count: u16,
	unknown1: u16,
	unknown2: u16,
	unknown3: u16,
	unknown4: u16,
	unknown5: u16,
	unknown6: u16,
	unknown7: u16,
	unknown8: u16,
	unknown9: u16,
	unknown10: u16,
	unknown11: u16,
	unknown12: u16,
}

fn read_extra_lods(
	reader: &mut (impl Read + Seek),
	options: &ReadOptions,
	(enable,): (bool,),
) -> BinResult<Option<[ExtraLod; MAX_LODS]>> {
	Ok(match enable {
		false => None,
		true => Some(<[ExtraLod; MAX_LODS]>::read_options(reader, options, ())?),
	})
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Mesh {
	vertex_count: u16,
	//padding:u16,
	#[br(pad_before = 2)]
	index_count: u32,
	material_index: u16,
	sub_mesh_index: u16,
	sub_mesh_count: u16,
	bone_table_index: u16,
	start_index: u32,
	vertex_buffer_offset: [u32; MAX_LODS],
	vertex_buffer_stride: [u8; MAX_LODS],
	vertex_stream_count: u8,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Submesh {
	index_offset: u32,
	index_count: u32,
	attribute_index_mask: u32,
	bone_start_index: u16,
	bone_count: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct TerrainShadowMesh {
	index_count: u32,
	start_index: u32,
	vertex_buffer_offset: u32,
	vertex_count: u16,
	sub_mesh_index: u16,
	sub_mesh_count: u16,
	#[br(pad_after = 1)]
	vertex_buffer_stride: u8,
	// padding: u8,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct TerrainShadowSubmesh {
	index_offset: u32,
	index_count: u32,
	unknown1: u16,
	unknown2: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct BoneTable {
	bone_index: [u16; 64],
	#[br(pad_after = 3)]
	bone_count: u8,
	// padding: [u8; 3],
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Shape {
	string_offset: u32,
	shape_mesh_start_index: [u16; MAX_LODS],
	shape_mesh_count: [u16; MAX_LODS],
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct ShapeMesh {
	start_index: u32,
	shape_value_count: u32,
	shape_value_offset: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct ShapeValue {
	offset: u16,
	value: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct BoundingBox {
	min: [f32; 4],
	max: [f32; 4],
}
