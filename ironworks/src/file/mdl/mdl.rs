// TODO: REMOVE
#![allow(missing_docs)]

use std::{io::Cursor, sync::Arc};

use binrw::{BinRead, VecArgs};
use half::f16;
use num_enum::IntoPrimitive;

use crate::{error::Result, file::File};

use super::structs::{self, VertexKind, VertexUsage};

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

	pub fn meshes_temp(&self) -> Result<(Vec<u16>, Vec<[f32; 4]>)> {
		// temp
		let lod_level = 0;

		let current_lod = &self.file.lods[lod_level];
		// println!("{curlod:?}");
		let mut ranges = vec![
			(current_lod.mesh_index, current_lod.mesh_count),
			(current_lod.water_mesh_index, current_lod.water_mesh_count),
			(current_lod.shadow_mesh_index, current_lod.shadow_mesh_count),
			(
				current_lod.terrain_shadow_mesh_index,
				current_lod.terrain_shadow_mesh_count,
			),
			(
				current_lod.vertical_fog_mesh_index,
				current_lod.vertical_fog_mesh_count,
			),
		];

		if let Some(ref extra_lods) = self.file.extra_lods {
			let extra_lod = &extra_lods[lod_level];
			ranges.append(&mut vec![
				(
					extra_lod.light_shaft_mesh_index,
					extra_lod.light_shaft_mesh_count,
				),
				(extra_lod.glass_mesh_index, extra_lod.glass_mesh_count),
				(
					extra_lod.material_change_mesh_index,
					extra_lod.material_change_mesh_count,
				),
				(
					extra_lod.crest_change_mesh_index,
					extra_lod.crest_change_mesh_count,
				),
			])
		}

		// TODO: i can simplify most of this with some nice arrays and a primitive enum with index=name for the types
		// that'd be so much cleaner than whatever the fuck the above trash is doing

		// let fsda = self.meshes.iter().enumerate().map(|(index, mesh)| {});
		// loomina precalculates all the mesh shit but handles it per-lod at a model level?
		let lod_meshes = (0..self.file.meshes.len())
			.map(|index| {
				let u16_index = u16::try_from(index).unwrap();
				// todo: enumerate here is just so i get the range index - need to change when i'm actually doing this properly
				let types = ranges
					.iter()
					.enumerate()
					.filter(|(_, (start, count))| u16_index >= *start && u16_index < start + count)
					.map(|(range_index, _)| range_index)
					.collect::<Vec<_>>();
				(index, types)
			})
			.filter(|(_index, types)| !types.is_empty())
			.map(|(index, types)| (&self.file.meshes[index], index, types))
			.collect::<Vec<_>>();

		Ok((vec![], vec![]))
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
	// TODO: Consider the api here, there's >1 type of mesh for any given model, and consumers might want to query for a particular type.
	//       Are they going to want a single .mesh at all?
	//       ... the index param is a total copout for now. fix.
	pub fn mesh(&self, mesh_index: usize) -> Mesh {
		Mesh {
			file: self.file.clone(),

			level: self.level,
			mesh_index,
		}
	}
}

#[derive(Debug)]
pub struct Mesh {
	file: Arc<structs::File>,

	// TODO: can probably abstract lod level out of the mesh level, as it's only being used for the index/vertex offset info, which is also available on the Lod struct (it needs a better name)
	level: Lod,
	mesh_index: usize,
}

impl Mesh {
	// TODO: bones
	// TODO: submeshes

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

	// TODO: how do we handle the kind of vertex in this api?
	pub fn vertices(&self) -> Vec<[f32; 4]> {
		let mesh = &self.file.meshes[self.mesh_index];

		// Get the elements for this mesh's vertices and sort them by their offset
		// - we'll be relying on that sort order to ensure reading picks them up in
		// the right order.
		let elements = &self.file.vertex_declarations[self.mesh_index].0;

		// TODO: is it possible to avoid the vec clone?
		let mut ordered_elements = elements.iter().collect::<Vec<_>>();
		ordered_elements.sort_unstable_by_key(|element| element.offset);

		// Vertices are stored across multipe streams of data - set up a cursor for each.
		let mut cursors = (0..usize::from(mesh.vertex_stream_count))
			.map(|index| {
				let offset = self.file.vertex_offset[usize::from(self.level)]
					+ mesh.vertex_buffer_offset[index];
				let mut cursor = Cursor::new(&self.file.data);
				cursor.set_position(u64::from(offset) - self.file.data_offset);
				cursor
			})
			.collect::<Vec<_>>();

		// TODO: remove this. just forcing to positions only temporarily
		let pos_el = ordered_elements
			.into_iter()
			.find(|el| matches!(el.usage, VertexUsage::Position))
			.unwrap();

		// Read in the vertices
		(0..mesh.vertex_count)
			.map(|_index| {
				let cursor = &mut cursors[usize::from(pos_el.stream)];
				// TODO: Usage
				// TODO: Other reader types
				match &pos_el.kind {
					// TODO: might be able to use half::slice to read in u16s first and convert in batch.
					// TODO: error handling
					VertexKind::Half4 => [
						f16::from_bits(u16::read(cursor).unwrap()).to_f32(),
						f16::from_bits(u16::read(cursor).unwrap()).to_f32(),
						f16::from_bits(u16::read(cursor).unwrap()).to_f32(),
						f16::from_bits(u16::read(cursor).unwrap()).to_f32(),
					],
					other => todo!("{other:?}"),
				}
			})
			.collect::<Vec<_>>()
	}
}
