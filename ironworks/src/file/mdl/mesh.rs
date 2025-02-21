use std::{
	io::{Cursor, Read, Seek, SeekFrom},
	sync::Arc,
};

use binrw::{BinRead, NullString, VecArgs};
use half::f16;

use crate::error::Result;

use super::{model::Lod, structs};

// TODO: improve the debug output of these things
/// A single mesh within a model.
#[derive(Debug)]
pub struct Mesh {
	pub(super) file: Arc<structs::File>,

	pub(super) level: Lod,
	pub(super) mesh_index: usize,
}

impl Mesh {
	// TODO: bones
	// TODO: submeshes

	// TODO: i'm not sure this should be specific to mesh - the list of materials on the model might be useful in some cases. should i use a ref to the parent model and read off that, rather than the arc of a file?
	/// Path to the material associated with this mesh.
	pub fn material(&self) -> Result<String> {
		let mesh = &self.file.meshes[self.mesh_index];
		let name_offset = self.file.material_name_offsets[usize::from(mesh.material_index)];

		// todo: this logic should probably be abstracted in the structs impl, and the buffer hidden?
		let mut cursor = Cursor::new(&self.file.string_buffer);
		cursor.set_position(name_offset.into());

		let name = NullString::read(&mut cursor)?.to_string();
		Ok(name)
	}

	// TODO: iterator?
	/// Indices of vertices within the mesh. Vertices are laid out in a triangle list topology.
	pub fn indices(&self) -> Result<Vec<u16>> {
		// Get the offset of the indices within the file. The `start_index` on `mesh`
		// is representative of an already-ready array of u16, ergo *2.
		let mesh = &self.file.meshes[self.mesh_index];
		let offset = self.file.index_offset[usize::from(self.level)] + mesh.start_index * 2;

		// Read in the indices.
		let mut cursor = Cursor::new(&self.file.data);
		cursor.set_position(u64::from(offset) - self.file.data_offset);

		let indices = <Vec<u16>>::read_le_args(
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
	/// Get the vertex attributes for all vertices in the mesh.
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

				let offsets = (0..mesh.vertex_count).scan(
					base_offset + u64::from(element.offset),
					|offset, _index| {
						let current = *offset;
						*offset += stride;
						Some(current)
					},
				);

				use VertexValues as V;
				use structs::VertexFormat as K;
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
	Ok([
		f32::read_le(reader)?,
		f32::read_le(reader)?,
		f32::read_le(reader)?,
	])
}

fn single4(reader: &mut (impl Read + Seek)) -> Result<[f32; 4]> {
	Ok([
		f32::read_le(reader)?,
		f32::read_le(reader)?,
		f32::read_le(reader)?,
		f32::read_le(reader)?,
	])
}

fn uint(reader: &mut (impl Read + Seek)) -> Result<u32> {
	Ok(u32::read_le(reader)?)
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
		f16::from_bits(u16::read_le(reader)?).to_f32(),
		f16::from_bits(u16::read_le(reader)?).to_f32(),
	])
}

fn half4(reader: &mut (impl Read + Seek)) -> Result<[f32; 4]> {
	Ok([
		f16::from_bits(u16::read_le(reader)?).to_f32(),
		f16::from_bits(u16::read_le(reader)?).to_f32(),
		f16::from_bits(u16::read_le(reader)?).to_f32(),
		f16::from_bits(u16::read_le(reader)?).to_f32(),
	])
}

// todo: public contents? - i mean, it makes sense to an extent.
/// A vertex attribute of a mesh.
#[derive(Debug)]
pub struct VertexAttribute {
	// todo i'm really not convinced on the name here
	/// The kind of data represented by this attribute.
	pub kind: structs::VertexAttributeKind,
	/// Attribute data values.
	pub values: VertexValues,
}

/// Values of a vertex attribute.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum VertexValues {
	Uint(Vec<u32>),
	Vector2(Vec<[f32; 2]>),
	Vector3(Vec<[f32; 3]>),
	Vector4(Vec<[f32; 4]>),
}
