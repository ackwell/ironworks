use std::sync::Arc;

use num_enum::IntoPrimitive;

use super::{mesh::Mesh, structs};

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
	pub(super) file: Arc<structs::File>,

	pub(super) level: Lod,
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
