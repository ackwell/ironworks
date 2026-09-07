// TODO: REMOVE
#![allow(dead_code, unused_parens, clippy::identity_op)]

use std::io::{Cursor, Read, Seek};

use binrw::helpers::until_eof;
use binrw::{BinRead, BinResult, Endian, binread};
use derivative::Derivative;
use modular_bitfield::bitfield;

const MAX_LODS: usize = 3;

/// The version before bone tables moved their indices into a shared array.
const VERSION_5: u32 = 0x0100_0005;

// TODO: this is currently inlining a bunch of structures - look into if it's worth pulling it apart at all.
#[binread]
#[br(little)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct File {
	// Model file header
	version: u32,
	stack_size: u32,
	runtime_size: u32,
	#[br(temp)]
	vertex_declaration_count: u16,
	material_count: u16,
	pub vertex_offset: [u32; MAX_LODS],
	pub index_offset: [u32; MAX_LODS],
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
	pub vertex_declarations: Vec<VertexDeclaration>,

	#[br(temp)]
	_string_count: u16,
	// padding: u16,
	#[br(pad_before = 2, temp)]
	string_size: u32,
	// TODO: lumina eagerly builds a map of offset -> string. worth doing?
	#[br(count = string_size)]
	pub string_buffer: Vec<u8>,

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
	#[br(temp)]
	culling_grid_count: u16,
	#[br(temp)]
	terrain_shadow_submesh_count: u16,
	flags3: u8,
	bg_change_material_index: u8,
	bg_crest_change_material_index: u8,
	#[br(temp)]
	neck_morph_count: u8,
	#[br(temp)]
	bone_table_array_count_total: u16,
	unknown8: u16,
	#[br(pad_after = 6, temp)]
	face_data_count: u16,

	// padding: [u8; 6],
	#[br(count = element_id_count)]
	element_ids: Vec<ElementId>,

	pub lods: [Lod; MAX_LODS],
	#[br(if(flags2.extra_lod_enabled()))]
	pub extra_lods: Option<[ExtraLod; MAX_LODS]>,

	#[br(count = mesh_count)]
	pub meshes: Vec<Mesh>,

	#[br(count = attribute_count)]
	pub attribute_name_offsets: Vec<u32>,

	#[br(count = terrain_shadow_mesh_count)]
	terrain_shadow_meshes: Vec<TerrainShadowMesh>,

	#[br(count = submesh_count)]
	pub submeshes: Vec<Submesh>,

	#[br(count = terrain_shadow_submesh_count)]
	terrain_shadow_submeshes: Vec<TerrainShadowSubmesh>,

	#[br(count = material_count_2)]
	pub material_name_offsets: Vec<u32>,

	#[br(count = bone_count)]
	pub bone_name_offsets: Vec<u32>,

	#[br(count = bone_table_count, args { inner: (version,) })]
	pub bone_tables: Vec<BoneTable>,

	#[br(count = bone_table_array_count_total)]
	pub bone_table_indices: Vec<u16>,

	#[br(count = shape_count)]
	pub shapes: Vec<Shape>,

	#[br(count = shape_mesh_count)]
	pub shape_meshes: Vec<ShapeMesh>,

	#[br(count = shape_value_count)]
	pub shape_values: Vec<ShapeValue>,

	#[br(temp)]
	submesh_bone_map_size: u32,
	#[br(count = submesh_bone_map_size / 2)]
	submesh_bone_map: Vec<u16>,

	#[br(count = neck_morph_count)]
	neck_morphs: Vec<NeckMorph>,

	#[br(count = face_data_count)]
	face_data: Vec<FaceVertex>,

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

	#[br(count = culling_grid_count)]
	culling_grid: Vec<BoundingBox>,

	// ??????
	// this is going to be a collection of smaller buffers - i'll probably be better off with manual accessors to fetch specific parts of it
	#[br(parse_with = current_position)]
	pub data_offset: u64,

	#[br(parse_with = until_eof)]
	#[derivative(Debug = "ignore")]
	pub data: Vec<u8>,
}

fn current_position<R: Read + Seek>(reader: &mut R, _: Endian, _: ()) -> BinResult<u64> {
	Ok(reader.stream_position()?)
}

impl File {
	/// Whether the engine may sway this model with the wind. The header states it as a disable.
	pub fn waving_animation(&self) -> bool {
		!self.flags1.waving_animation_disabled()
	}

	/// Whether the sun casts this model. The header states it as a disable.
	pub fn shadow(&self) -> bool {
		!self.flags1.shadow_disabled()
	}

	/// The bones one table names, as indices into [`Self::bone_name_offsets`]. A span states its
	/// position in 4 byte units from its own entry, and the shared array follows the entries.
	pub fn bone_table(&self, index: usize) -> &[u16] {
		match self.bone_tables.get(index) {
			Some(BoneTable::Inline {
				bone_index,
				bone_count,
			}) => bone_index
				.get(..usize::from(*bone_count))
				.unwrap_or_default(),
			Some(BoneTable::Span { offset, size }) => {
				let at = (index + usize::from(*offset)).saturating_sub(self.bone_tables.len()) * 2;
				self.bone_table_indices
					.get(at..at + usize::from(*size))
					.unwrap_or_default()
			}
			None => &[],
		}
	}

	/// The null-terminated name at `offset` into the shared string buffer, which is how every table
	/// of names in the file points at one.
	pub fn string(&self, offset: u32) -> crate::error::Result<String> {
		let mut cursor = Cursor::new(&self.string_buffer);
		cursor.set_position(offset.into());
		Ok(binrw::NullString::read(&mut cursor)?.to_string())
	}
}

fn to_bool(value: u8) -> bool {
	value != 0
}

#[derive(Debug)]
pub struct VertexDeclaration(pub Vec<VertexElement>);
impl BinRead for VertexDeclaration {
	type Args<'a> = ();

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: Endian,
		args: Self::Args<'_>,
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
pub struct VertexElement {
	// todo names
	pub stream: u8,
	pub offset: u8,
	pub format: VertexFormat,
	pub attribute: VertexAttributeKind,
	/// Distinguishes elements sharing an attribute kind, such as a mesh's second UV set.
	#[br(pad_after = 3)]
	pub usage_index: u8,
}

#[binread]
#[br(repr = u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VertexFormat {
	None = 0,
	Single3 = 2,
	Single4 = 3,
	Uint = 5,
	ByteFloat4 = 8,
	Half2 = 13,
	Half4 = 14,
	/// Eight bytes, carrying a blend weight or bone index each.
	UByte8 = 17,
}

/// The kind of data represented by a vertex attribute.
#[allow(missing_docs)]
#[binread]
#[br(repr = u8)]
#[derive(Clone, Copy, Debug)]
pub enum VertexAttributeKind {
	Position = 0,
	BlendWeights = 1,
	BlendIndices = 2,
	Normal = 3,
	Uv = 4,
	Tangent2 = 5,
	Tangent1 = 6,
	Color = 7,
}

// Fields are declared least significant bit first.
#[bitfield]
#[binread]
#[derive(Debug)]
#[br(map = Self::from_bytes)]
struct Flags1 {
	shadow_disabled: bool,
	light_shadow_disabled: bool,
	waving_animation_disabled: bool,
	lighting_reflection_enabled: bool,
	unknown1: bool,
	rain_occlusion_enabled: bool,
	snow_occlusion_enabled: bool,
	dust_occlusion_enabled: bool,
}

#[bitfield]
#[binread]
#[derive(Debug)]
#[br(map = Self::from_bytes)]
struct Flags2 {
	unknown3: bool,
	edge_geometry_enabled: bool,
	force_lod_range_enabled: bool,
	shadow_mask_enabled: bool,
	extra_lod_enabled: bool,
	enable_force_non_resident: bool,
	bg_uv_scroll_enabled: bool,
	unknown2: bool,
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
//       ...it's not contiguous, and spread across two structs - could be fiddly. maybe a parse=skip or something that post-processes it into a vec or w/e?
#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Lod {
	pub mesh_index: u16,
	pub mesh_count: u16,
	model_lod_range: f32,
	texture_lod_range: f32,
	pub water_mesh_index: u16,
	pub water_mesh_count: u16,
	pub shadow_mesh_index: u16,
	pub shadow_mesh_count: u16,
	pub terrain_shadow_mesh_index: u16,
	pub terrain_shadow_mesh_count: u16,
	pub vertical_fog_mesh_index: u16,
	pub vertical_fog_mesh_count: u16,
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
pub struct ExtraLod {
	pub light_shaft_mesh_index: u16,
	pub light_shaft_mesh_count: u16,
	pub glass_mesh_index: u16,
	pub glass_mesh_count: u16,
	pub material_change_mesh_index: u16,
	pub material_change_mesh_count: u16,
	pub crest_change_mesh_index: u16,
	pub crest_change_mesh_count: u16,
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

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Mesh {
	pub vertex_count: u16,
	//padding:u16,
	#[br(pad_before = 2)]
	pub index_count: u32,
	pub material_index: u16,
	pub sub_mesh_index: u16,
	pub sub_mesh_count: u16,
	pub bone_table_index: u16,
	pub start_index: u32,
	// TODO: the 3 here is the no. of streams
	pub vertex_buffer_offset: [u32; 3],
	pub vertex_buffer_stride: [u8; 3],
	pub vertex_stream_count: u8,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Submesh {
	pub index_offset: u32,
	pub index_count: u32,
	pub attribute_index_mask: u32,
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
#[br(little, import(version: u32))]
#[derive(Debug)]
pub enum BoneTable {
	#[br(pre_assert(version == VERSION_5))]
	Inline {
		bone_index: [u16; 64],
		#[br(pad_after = 3)]
		bone_count: u8,
		// padding: [u8; 3],
	},

	/// A span of [`File::bone_table_indices`], offset in 4 byte units from this entry's own
	/// position.
	Span { offset: u16, size: u16 },
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct NeckMorph {
	position: [f32; 3],
	/// Weights over [`Self::bone_index`], summing to 255.
	bone_weight: [u8; 4],
	normal: [f32; 3],
	bone_index: [u8; 4],
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct FaceVertex {
	position: [f32; 3],
	unknown1: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Shape {
	pub string_offset: u32,
	pub shape_mesh_start_index: [u16; MAX_LODS],
	pub shape_mesh_count: [u16; MAX_LODS],
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct ShapeMesh {
	/// The [`start_index`](Mesh::start_index) of the mesh this rewrites, which is what names it.
	pub mesh_start_index: u32,
	pub shape_value_count: u32,
	pub shape_value_offset: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct ShapeValue {
	/// Which of the mesh's own indices to rewrite.
	pub offset: u16,
	/// The vertex to draw in place of the one there.
	pub vertex: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct BoundingBox {
	min: [f32; 4],
	max: [f32; 4],
}

impl File {
	/// The box the file states the whole model fills, in the model's own space.
	pub(super) fn bounds(&self) -> ([f32; 3], [f32; 3]) {
		let held = &self.bounding_boxes;
		(
			[held.min[0], held.min[1], held.min[2]],
			[held.max[0], held.max[1], held.max[2]],
		)
	}
}

#[cfg(test)]
mod test {
	use std::io::Cursor;

	use binrw::BinRead;

	use super::{File, VERSION_5};

	/// A model carrying one of each section the header's counts can turn on. The vertex offset
	/// it declares is the length of everything laid out before the vertex buffer, so reading it
	/// back checks the reader consumes exactly the sections written.
	#[derive(Default)]
	struct Model {
		version: u32,
		flags2: u8,
		/// Bone index counts, one per table.
		bone_tables: Vec<u16>,
		neck_morph_count: u8,
		face_data_count: u16,
		culling_grid_count: u16,
		attribute_count: u16,
		shape_count: u16,
		shape_mesh_count: u16,
		shape_value_count: u16,
	}

	impl Model {
		fn bytes(&self) -> Vec<u8> {
			let bone_count = 2u16;
			// Only the shared array is counted, and the older layout has none.
			let table_indices = match self.version {
				VERSION_5 => 0,
				_ => self
					.bone_tables
					.iter()
					.map(|count| count.next_multiple_of(2))
					.sum::<u16>(),
			};

			let mut body = Vec::new();
			// One vertex declaration, every element past the stream sentinel unused.
			body.extend([255u8, 0, 0, 0, 0, 0, 0, 0].repeat(17));
			// String section, holding one terminator.
			body.extend(1u16.to_le_bytes());
			body.extend([0; 2]);
			body.extend(1u32.to_le_bytes());
			body.push(0);

			// Model header.
			body.extend(1.0f32.to_le_bytes());
			for count in [
				0u16,
				self.attribute_count,
				0,
				0,
				bone_count,
				self.bone_tables.len() as u16,
				self.shape_count,
				self.shape_mesh_count,
				self.shape_value_count,
			] {
				body.extend(count.to_le_bytes());
			}
			body.extend([1, 0]);
			body.extend(0u16.to_le_bytes());
			body.extend([0, self.flags2]);
			body.extend(0.0f32.to_le_bytes());
			body.extend(0.0f32.to_le_bytes());
			body.extend(self.culling_grid_count.to_le_bytes());
			body.extend(0u16.to_le_bytes());
			body.extend([0, 0, 0, self.neck_morph_count]);
			body.extend(table_indices.to_le_bytes());
			body.extend(0u16.to_le_bytes());
			body.extend(self.face_data_count.to_le_bytes());
			body.extend([0; 6]);

			body.extend([0; 3 * 60]);
			if self.flags2 & 0x10 != 0 {
				body.extend([0; 3 * 40]);
			}
			body.extend(vec![0; usize::from(self.attribute_count) * 4]);
			// Bone name offsets.
			body.extend([0; 8]);

			if self.version == VERSION_5 {
				for count in &self.bone_tables {
					body.extend([0; 128]);
					body.extend([*count as u8, 0, 0, 0]);
				}
			} else {
				let mut offset = self.bone_tables.len() as u16;
				for (index, count) in self.bone_tables.iter().enumerate() {
					// Offsets run in 4 byte units from the entry's own position.
					body.extend((offset - index as u16).to_le_bytes());
					body.extend(count.to_le_bytes());
					offset += count.next_multiple_of(2) / 2;
				}
				body.extend((0..table_indices).flat_map(u16::to_le_bytes));
			}

			body.extend(vec![0; usize::from(self.shape_count) * 16]);
			body.extend(vec![0; usize::from(self.shape_mesh_count) * 12]);
			body.extend(vec![0; usize::from(self.shape_value_count) * 4]);
			// Submesh bone map, empty.
			body.extend(0u32.to_le_bytes());
			body.extend(vec![0; usize::from(self.neck_morph_count) * 32]);
			body.extend(vec![0; usize::from(self.face_data_count) * 16]);
			// Padding, which the reader takes from the byte before it.
			body.push(4);
			body.extend([0; 4]);
			body.extend(vec![0; (4 + usize::from(bone_count)) * 32]);
			body.extend(vec![0; usize::from(self.culling_grid_count) * 32]);

			let mut bytes = Vec::new();
			bytes.extend(self.version.to_le_bytes());
			bytes.extend(136u32.to_le_bytes());
			bytes.extend((body.len() as u32 - 136).to_le_bytes());
			bytes.extend(1u16.to_le_bytes());
			bytes.extend(0u16.to_le_bytes());
			let vertex_offset = 0x44 + body.len() as u32;
			bytes.extend(vertex_offset.to_le_bytes());
			bytes.extend([0; 8 + 12 + 12 + 12]);
			bytes.extend([1, 0, 0, 0]);
			bytes.extend(body);
			// A byte of vertex buffer, so the reader has something to stop at.
			bytes.push(0);
			bytes
		}

		fn read(&self) -> File {
			File::read(&mut Cursor::new(self.bytes())).unwrap()
		}
	}

	/// Every section ends where the vertex buffer begins.
	fn ends_at_the_vertex_buffer(model: &Model) {
		let file = model.read();
		assert_eq!(file.data_offset, u64::from(file.vertex_offset[0]));
	}

	#[test]
	fn reads_bone_tables_as_spans_of_a_shared_array() {
		let model = Model {
			version: 0x0100_0006,
			bone_tables: vec![3, 8, 1],
			..Default::default()
		};
		ends_at_the_vertex_buffer(&model);

		let file = model.read();
		// Index arrays pad up to an even count, so the total exceeds the sum of the sizes.
		assert_eq!(file.bone_table_indices.len(), 14);
		assert!(matches!(
			file.bone_tables[1],
			super::BoneTable::Span { offset: 4, size: 8 }
		));
		assert_eq!(file.bone_table(0), [0, 1, 2]);
		assert_eq!(file.bone_table(1), [4, 5, 6, 7, 8, 9, 10, 11]);
		assert_eq!(file.bone_table(2), [12]);
	}

	/// The version before it holds a fixed 132 byte table each, with no shared array.
	#[test]
	fn reads_bone_tables_inline() {
		let model = Model {
			version: VERSION_5,
			bone_tables: vec![3, 8, 1],
			..Default::default()
		};
		let file = model.read();
		assert!(file.bone_table_indices.is_empty());
		assert!(matches!(
			file.bone_tables[2],
			super::BoneTable::Inline { bone_count: 1, .. }
		));
	}

	#[test]
	fn gates_extra_lods_on_the_fifth_bit() {
		for flags2 in [0x10, 0x1F, 0xF0] {
			let model = Model {
				version: 0x0100_0006,
				flags2,
				..Default::default()
			};
			ends_at_the_vertex_buffer(&model);
			assert!(model.read().extra_lods.is_some());
		}
		for flags2 in [0x08, 0x0F, 0xEF] {
			let model = Model {
				version: 0x0100_0006,
				flags2,
				..Default::default()
			};
			ends_at_the_vertex_buffer(&model);
			assert!(model.read().extra_lods.is_none());
		}
	}

	/// The attribute names and the three shape tables, which sit either side of the bone tables.
	#[test]
	fn reads_the_attribute_and_shape_tables() {
		let model = Model {
			version: 0x0100_0006,
			bone_tables: vec![4],
			attribute_count: 8,
			shape_count: 23,
			shape_mesh_count: 61,
			shape_value_count: 1814,
			..Default::default()
		};
		ends_at_the_vertex_buffer(&model);

		let file = model.read();
		assert_eq!(file.attribute_name_offsets.len(), 8);
		assert_eq!(file.shapes.len(), 23);
		assert_eq!(file.shape_meshes.len(), 61);
		assert_eq!(file.shape_values.len(), 1814);
	}

	#[test]
	fn reads_the_sections_trailing_the_submesh_bone_map() {
		let model = Model {
			version: 0x0100_0006,
			bone_tables: vec![4],
			neck_morph_count: 10,
			face_data_count: 6489,
			culling_grid_count: 5,
			..Default::default()
		};
		ends_at_the_vertex_buffer(&model);

		let file = model.read();
		assert_eq!(file.neck_morphs.len(), 10);
		assert_eq!(file.face_data.len(), 6489);
		assert_eq!(file.culling_grid.len(), 5);
	}
}
