use std::io::Cursor;

use binrw::{BinRead, VecArgs};

use crate::{error::Result, file::File};

use super::structs;

#[derive(Debug)]
pub struct ModelContainer {
	file: structs::File,
}

impl File for ModelContainer {
	fn read(data: Vec<u8>) -> Result<Self> {
		let file = structs::File::read(&mut Cursor::new(data))?;

		Ok(ModelContainer { file })
	}
}

impl ModelContainer {
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

		// todo: this really should not be here
		let (mesh, mesh_index, _types) = &lod_meshes[0];
		// todo bone table
		// todo submeshes
		// indexes first because it looks easier kill me
		let mut cursor = Cursor::new(&self.file.data);
		// what's the *2 for? i'm guessing that it treats the index buffer as a single block of indexes, and it's the start index of the index within that buffer, hence *2 for u16?
		// todo this can use the lod index offset probably
		cursor.set_position(
			u64::from(self.file.index_offset[lod_level] + mesh.start_index * 2)
				- self.file.data_offset,
		);
		let indicies = <Vec<u16>>::read_args(
			&mut cursor,
			VecArgs {
				count: mesh.index_count.try_into().unwrap(),
				inner: (),
			},
		)?;
		// println!("{:?}", indicies);

		// verticies
		let decl = &self.file.vertex_declarations[*mesh_index];

		// sort the vertex elements in the decl so we can read in-order from the cursor
		let mut ordecl = decl.0.iter().collect::<Vec<_>>();
		ordecl.sort_unstable_by_key(|element| element.offset);
		// println!("{mesh:#?} {:#?}", ordecl);

		// yikes
		let posel = *ordecl.iter().find(|el| el.usage == 0).unwrap();

		let mut cursors = (0..usize::from(mesh.vertex_stream_count))
			.map(|stream_index| {
				let mut cursor = Cursor::new(&self.file.data);
				cursor.set_position(
					// todo this can use the lod index offset probably
					u64::from(
						self.file.vertex_offset[lod_level]
							+ mesh.vertex_buffer_offset[stream_index],
					) - self.file.data_offset,
				);
				cursor
			})
			.collect::<Vec<_>>();

		// ok so the idea is that we loop through 0..vertex count
		// and then, for each vertex, read in data once for each element
		// reading the first 10 just to... _see_ something
		let verticies = (0..mesh.vertex_count)
			// let verticies = (0..10)
			.map(|vertex_index| {
				// ordecl
				// 	.iter()
				// 	.map(|el| {
				// 		// todo properly with enums and all that jazz
				// 		// type, usage
				// 		let cursor = &mut cursors[usize::from(el.stream)];
				// 		match el.type_ {
				// 			8 => [
				// 				f32::from(u8::read(cursor).unwrap()) / 255f32,
				// 				f32::from(u8::read(cursor).unwrap()) / 255f32,
				// 				f32::from(u8::read(cursor).unwrap()) / 255f32,
				// 				f32::from(u8::read(cursor).unwrap()) / 255f32,
				// 			],
				// 			// 13 => 1,
				// 			// 14 => 1,
				// 			_ => todo!(),
				// 		}
				// 	})
				// 	.collect::<Vec<_>>()

				let cursor = &mut cursors[usize::from(posel.stream)];
				match posel.type_ {
					// 8 => [
					// 	f32::from(u8::read(cursor).unwrap()) / 255f32,
					// 	f32::from(u8::read(cursor).unwrap()) / 255f32,
					// 	f32::from(u8::read(cursor).unwrap()) / 255f32,
					// 	f32::from(u8::read(cursor).unwrap()) / 255f32,
					// ],
					// 13 => 1,
					// ??? i have no idea if this will work. at all.
					14 => [
						// should i expose these af f32 of half?
						half::f16::from_bits(u16::read(cursor).unwrap()).to_f32(),
						half::f16::from_bits(u16::read(cursor).unwrap()).to_f32(),
						half::f16::from_bits(u16::read(cursor).unwrap()).to_f32(),
						half::f16::from_bits(u16::read(cursor).unwrap()).to_f32(),
					],
					_ => todo!("{}", posel.type_),
				}
			})
			.collect::<Vec<_>>();
		// println!("{verticies:#?}");

		// foo
		Ok((indicies, verticies))
	}
}
