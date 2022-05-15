// TODO: REMOVE
#![allow(missing_docs)]

use std::{
	io::{Cursor, Read, Seek, SeekFrom},
	sync::Arc,
};

use binrw::{BinRead, VecArgs};
use half::f16;
use num_enum::IntoPrimitive;

use crate::{error::Result, file::File};

use super::structs::{self, VertexAttributeKind, VertexFormat};

#[derive(Debug)]
pub struct ModelContainer {
	file: Arc<structs::File>,
}

impl File for ModelContainer {
	fn read(data: Vec<u8>) -> Result<Self> {
		let file = structs::File::read(&mut Cursor::new(data))?;

		Ok(ModelContainer { file: file.into() })
	}
}

impl ModelContainer {
	// TODO: name? do we call it "lod" because it fetches a lod model, or "model" because it fetches a model of a lod?
	pub fn lod(&self, level: Lod) -> Model {
		Model {
			file: self.file.clone(),

			level,
		}
	}
}

// TODO: consider if it makes sense to keep Lod around as it's enum repr for anything beyond user facing api
#[derive(Clone, Copy, Debug, IntoPrimitive)]
#[repr(usize)]
pub enum Lod {
	High = 0,
	Medium = 1,
	Low = 2,
}

#[derive(Debug)]
pub struct Model {
	file: Arc<structs::File>,

	level: Lod,
}

impl Model {
	// TODO: Expose mesh kinds
	// TODO: Maybe mesh filter?
	// TODO: iterator?
	pub fn meshes(&self) -> Vec<Mesh> {
		let ranges = self.get_ranges();

		(0..self.file.meshes.len())
			// Get a vector of the kinds of each map at this lod, filtering any with none.
			.map(|index| {
				let u16_index = u16::try_from(index).unwrap();

				let kinds = ranges
					.iter()
					.filter(|(_, start, count)| u16_index >= *start && u16_index < start + count)
					.map(|(kind, _, _)| *kind)
					.collect::<Vec<_>>();

				(index, kinds)
			})
			.filter(|(_, kinds)| !kinds.is_empty())
			// Build the final mesh structs.
			.map(|(mesh_index, _kinds)| Mesh {
				file: self.file.clone(),

				level: self.level,
				mesh_index,
			})
			.collect()
	}

	fn get_ranges(&self) -> Vec<(MeshKind, u16, u16)> {
		let level = usize::from(self.level);
		let current_lod = &self.file.lods[level];

		let mut ranges = vec![
			(
				MeshKind::Standard,
				current_lod.mesh_index,
				current_lod.mesh_count,
			),
			(
				MeshKind::Water,
				current_lod.water_mesh_index,
				current_lod.water_mesh_count,
			),
			(
				MeshKind::Shadow,
				current_lod.shadow_mesh_index,
				current_lod.shadow_mesh_count,
			),
			(
				MeshKind::Terrain,
				current_lod.terrain_shadow_mesh_index,
				current_lod.terrain_shadow_mesh_count,
			),
			(
				MeshKind::VerticalFog,
				current_lod.vertical_fog_mesh_index,
				current_lod.vertical_fog_mesh_count,
			),
		];

		if let Some(ref extra_lods) = self.file.extra_lods {
			let extra_lod = &extra_lods[level];
			ranges.append(&mut vec![
				(
					MeshKind::LightShaft,
					extra_lod.light_shaft_mesh_index,
					extra_lod.light_shaft_mesh_count,
				),
				(
					MeshKind::Glass,
					extra_lod.glass_mesh_index,
					extra_lod.glass_mesh_count,
				),
				(
					MeshKind::MaterialChange,
					extra_lod.material_change_mesh_index,
					extra_lod.material_change_mesh_count,
				),
				(
					MeshKind::CrestChange,
					extra_lod.crest_change_mesh_index,
					extra_lod.crest_change_mesh_count,
				),
			])
		}

		ranges
	}
}

#[derive(Clone, Copy, Debug)]
pub enum MeshKind {
	Standard,
	Water,
	Shadow,
	Terrain,
	VerticalFog,
	LightShaft,
	Glass,
	MaterialChange,
	CrestChange,
}

#[derive(Debug)]
pub struct Mesh {
	file: Arc<structs::File>,

	level: Lod,
	mesh_index: usize,
}

impl Mesh {
	// TODO: bones
	// TODO: submeshes

	// TODO: iterator?
	pub fn indices(&self) -> Result<Vec<u16>> {
		// Get the offset of the indices within the file. The `start_index` on `mesh`
		// is representative of an already-ready array of u16, ergo *2.
		let mesh = &self.file.meshes[self.mesh_index];
		let offset = self.file.index_offset[usize::from(self.level)] + mesh.start_index * 2;

		// Read in the indices.
		let mut cursor = Cursor::new(&self.file.data);
		cursor.set_position(u64::from(offset) - self.file.data_offset);

		let indices = <Vec<u16>>::read_args(
			&mut cursor,
			VecArgs {
				count: mesh.index_count.try_into().unwrap(),
				inner: (),
			},
		)?;

		Ok(indices)
	}

	// TODO: fn to get a specific attr?
	// TODO: iterator?
	pub fn attributes(&self) -> Result<Vec<VertexAttribute>> {
		let mesh = &self.file.meshes[self.mesh_index];

		// Get the elements for this mesh's vertices.
		let elements = &self.file.vertex_declarations[self.mesh_index].0;

		// Vertices are stored across multipe streams of data - set up a cursor for each.
		let mut streams = (0..usize::from(mesh.vertex_stream_count))
			.map(|index| {
				let cursor = Cursor::new(&self.file.data);
				let offset = self.file.vertex_offset[usize::from(self.level)]
					+ mesh.vertex_buffer_offset[index];
				(cursor, u64::from(offset) - self.file.data_offset)
			})
			.collect::<Vec<_>>();

		// Read in the vertices
		// TODO: keep an eye on perf here - could thrash cache a bit if llvm doesn't magic it enough
		elements
			.iter()
			.map(|element| -> Result<_> {
				let stream = usize::from(element.stream);
				let (ref mut cursor, base_offset) = streams[stream];
				let stride = u64::from(mesh.vertex_buffer_stride[stream]);

				let offsets = (0..mesh.vertex_count).scan(base_offset, |offset, _index| {
					let current = *offset;
					*offset += stride;
					Some(current)
				});

				use VertexFormat as K;
				use VertexValues as V;
				let values = match &element.format {
					K::Single3 => V::Vector3(read_values(offsets, cursor, single3)?),
					K::Single4 => V::Vector4(read_values(offsets, cursor, single4)?),
					K::Uint => V::Uint(read_values(offsets, cursor, uint)?),
					K::ByteFloat4 => V::Vector4(read_values(offsets, cursor, bfloat4)?),
					K::Half2 => V::Vector2(read_values(offsets, cursor, half2)?),
					K::Half4 => V::Vector4(read_values(offsets, cursor, half4)?),
					other => todo!("Vertex kind: {other:?}"),
				};

				Ok(VertexAttribute {
					kind: element.attribute,
					values,
				})
			})
			.collect::<Result<Vec<_>>>()
	}
}

fn read_values<R, F, O>(
	offsets: impl Iterator<Item = u64>,
	reader: &mut R,
	map_fn: F,
) -> Result<Vec<O>>
where
	R: Read + Seek,
	F: Fn(&mut R) -> Result<O>,
{
	offsets
		.map(|offset| {
			reader.seek(SeekFrom::Start(offset))?;
			map_fn(reader)
		})
		.collect::<Result<Vec<_>>>()
}

fn single3(reader: &mut (impl Read + Seek)) -> Result<[f32; 3]> {
	Ok([f32::read(reader)?, f32::read(reader)?, f32::read(reader)?])
}

fn single4(reader: &mut (impl Read + Seek)) -> Result<[f32; 4]> {
	Ok([
		f32::read(reader)?,
		f32::read(reader)?,
		f32::read(reader)?,
		f32::read(reader)?,
	])
}

fn uint(reader: &mut (impl Read + Seek)) -> Result<u32> {
	Ok(u32::read(reader)?)
}

fn bfloat4(reader: &mut (impl Read + Seek)) -> Result<[f32; 4]> {
	Ok([
		f32::from(u8::read(reader)?) / 255.,
		f32::from(u8::read(reader)?) / 255.,
		f32::from(u8::read(reader)?) / 255.,
		f32::from(u8::read(reader)?) / 255.,
	])
}

fn half2(reader: &mut (impl Read + Seek)) -> Result<[f32; 2]> {
	Ok([
		f16::from_bits(u16::read(reader)?).to_f32(),
		f16::from_bits(u16::read(reader)?).to_f32(),
	])
}

fn half4(reader: &mut (impl Read + Seek)) -> Result<[f32; 4]> {
	Ok([
		f16::from_bits(u16::read(reader)?).to_f32(),
		f16::from_bits(u16::read(reader)?).to_f32(),
		f16::from_bits(u16::read(reader)?).to_f32(),
		f16::from_bits(u16::read(reader)?).to_f32(),
	])
}

// todo: public contents?
#[derive(Debug)]
pub struct VertexAttribute {
	// todo i'm really not convinced on the name here
	pub kind: VertexAttributeKind,
	pub values: VertexValues,
}

#[derive(Debug)]
pub enum VertexValues {
	Uint(Vec<u32>),
	Vector2(Vec<[f32; 2]>),
	Vector3(Vec<[f32; 3]>),
	Vector4(Vec<[f32; 4]>),
}
