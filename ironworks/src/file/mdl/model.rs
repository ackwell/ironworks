use std::sync::Arc;

use num_enum::IntoPrimitive;

use crate::error::Result;

use super::{mesh::Mesh, structs};

// TODO: consider if it makes sense to keep Lod around as it's enum repr for anything beyond user facing api
/// Level of detail.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, IntoPrimitive)]
#[repr(usize)]
pub enum Lod {
	High = 0,
	Medium = 1,
	Low = 2,
}

/// A single model, consisting of one or more seperate meshes.
#[derive(Debug)]
pub struct Model {
	pub(super) file: Arc<structs::File>,

	pub(super) level: Lod,
}

impl Model {
	/// Whether the engine may sway this model with the wind.
	pub fn waving(&self) -> bool {
		self.file.waving_animation()
	}

	/// Whether the sun's shadow pass draws this model.
	pub fn shadowing(&self) -> bool {
		self.file.shadow()
	}

	/// The box the file states this model fills, in the model's own space, as `(min, max)`.
	pub fn bounds(&self) -> ([f32; 3], [f32; 3]) {
		self.file.bounds()
	}

	// TODO: Expose mesh kinds
	// TODO: Maybe mesh filter?
	// TODO: iterator?
	/// Get a vector of all meshes within this model.
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
			.map(|(mesh_index, kinds)| Mesh {
				file: self.file.clone(),

				level: self.level,
				mesh_index,
				kinds,
			})
			.collect()
	}

	/// The bones the model is skinned to, in the order a mesh's
	/// [`bone_table`](super::Mesh::bone_table) indexes them. The names are a skeleton's own.
	pub fn bone_names(&self) -> Result<Vec<String>> {
		self.file
			.bone_name_offsets
			.iter()
			.map(|offset| self.file.string(*offset))
			.collect()
	}

	/// The names a submesh's [`attributes`](super::Submesh::attributes) mask picks out of, in the
	/// order its bits run.
	pub fn attribute_names(&self) -> Result<Vec<String>> {
		self.file
			.attribute_name_offsets
			.iter()
			.map(|offset| self.file.string(*offset))
			.collect()
	}

	/// The shape keys the model declares, whether or not any of them reaches this detail level.
	pub fn shapes(&self) -> Vec<Shape> {
		(0..self.file.shapes.len())
			.map(|index| Shape {
				file: self.file.clone(),
				level: self.level,
				index,
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

/// One shape key: a set of index rewrites that swap part of a mesh out for geometry sitting further
/// along the same vertex buffer, which is how a face carries its expressions.
#[derive(Debug)]
pub struct Shape {
	file: Arc<structs::File>,
	level: Lod,
	index: usize,
}

impl Shape {
	pub fn name(&self) -> Result<String> {
		self.file.string(self.file.shapes[self.index].string_offset)
	}

	/// Where the shape rewrites `mesh`, as pairs of which of the mesh's own indices to replace and
	/// the vertex to draw in place of the one there. Empty where the shape does not reach this mesh
	/// or this detail level.
	pub fn rewrites(&self, mesh: &Mesh) -> Vec<(u16, u16)> {
		let shape = &self.file.shapes[self.index];
		let level = usize::from(self.level);
		let first = usize::from(shape.shape_mesh_start_index[level]);
		let start_index = self.file.meshes[mesh.mesh_index].start_index;

		self.file
			.shape_meshes
			.get(first..first + usize::from(shape.shape_mesh_count[level]))
			.unwrap_or_default()
			.iter()
			.filter(|shape_mesh| shape_mesh.mesh_start_index == start_index)
			.flat_map(|shape_mesh| {
				let at = shape_mesh.shape_value_offset as usize;
				self.file
					.shape_values
					.get(at..at + shape_mesh.shape_value_count as usize)
					.unwrap_or_default()
			})
			.map(|value| (value.offset, value.vertex))
			.collect()
	}
}

/// What a mesh is drawn for.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
