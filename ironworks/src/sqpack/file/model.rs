use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};

use binrw::{binread, BinRead, BinWriterExt, VecArgs};

use crate::error::Result;

use super::shared::{read_block, Header};

const MAX_LODS: usize = 3;

#[binread]
#[derive(Debug)]
#[br(little)]
struct ModelHeader {
	_size: SectionInfo<u32>,
	_compressed_size: SectionInfo<u32>,
	offset: SectionInfo<u32>,
	block_index: SectionInfo<u16>,
	block_count: SectionInfo<u16>,
	vertex_declaration_count: u16,
	material_count: u16,
	lod_count: u8,
	index_buffer_streaming_enabled: u8, //bool
	edge_geometry_enabled: u8,          //bool
	_padding: u8,
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct SectionInfo<T: BinRead<Args = ()> + 'static> {
	stack: T,
	runtime: T,
	vertex_buffer: [T; MAX_LODS],
	edge_geometry_vertex_buffer: [T; MAX_LODS],
	index_buffer: [T; MAX_LODS],
}

pub fn read(mut reader: impl Read + Seek, offset: u32, header: Header) -> Result<Vec<u8>> {
	let model_header = ModelHeader::read(&mut reader)?;

	// Model header is followed by an array of block sizes.
	let block_counts = &model_header.block_count;
	let total_blocks = block_counts.stack
		+ block_counts.runtime
		+ block_counts.vertex_buffer.iter().sum::<u16>()
		+ block_counts.edge_geometry_vertex_buffer.iter().sum::<u16>()
		+ block_counts.index_buffer.iter().sum::<u16>();

	// TODO: i should probably make an impl for this it's pretty repetetive
	let block_sizes = <Vec<u16>>::read_args(
		&mut reader,
		VecArgs {
			count: total_blocks.try_into().unwrap(),
			inner: (),
		},
	)?;

	// Build a writer for the output file.
	let mut writer = Cursor::new(Vec::<u8>::with_capacity(
		header.raw_file_size.try_into().unwrap(),
	));

	// First 0x44 is the header, which will be filled at the end
	writer.seek(SeekFrom::Start(0x44))?;

	// Stack
	let stack_size = read_blocks(
		model_header.block_count.stack,
		model_header.block_index.stack,
		offset + model_header.offset.stack,
		&block_sizes,
		&mut reader,
		&mut writer,
	)?;

	// Runtime
	let runtime_size = read_blocks(
		model_header.block_count.runtime,
		model_header.block_index.runtime,
		offset + model_header.offset.runtime,
		&block_sizes,
		&mut reader,
		&mut writer,
	)?;

	// LOD level data
	let mut vertex_data_offsets = [0u32; MAX_LODS];
	let mut vertex_buffer_sizes = [0u32; MAX_LODS];

	let mut index_data_offsets = [0u32; MAX_LODS];
	let mut index_buffer_sizes = [0u32; MAX_LODS];

	for lod_index in 0..MAX_LODS {
		// Vertex buffer
		let block_count = model_header.block_count.vertex_buffer[lod_index];
		if block_count != 0 {
			if lod_index == 0 || block_count > 0 {
				vertex_data_offsets[lod_index] = writer.position().try_into().unwrap();
			}

			vertex_buffer_sizes[lod_index] = read_blocks(
				block_count,
				model_header.block_index.vertex_buffer[lod_index],
				offset + model_header.offset.vertex_buffer[lod_index],
				&block_sizes,
				&mut reader,
				&mut writer,
			)?;
		}

		// Edge geometry vertex buffer
		let block_count = model_header.block_count.edge_geometry_vertex_buffer[lod_index];
		if block_count != 0 {
			read_blocks(
				block_count,
				model_header.block_index.edge_geometry_vertex_buffer[lod_index],
				offset + model_header.offset.edge_geometry_vertex_buffer[lod_index],
				&block_sizes,
				&mut reader,
				&mut writer,
			)?;
		}

		// Index buffer
		let block_count = model_header.block_count.index_buffer[lod_index];
		if block_count != 0 {
			if lod_index == 0 || block_count > 0 {
				index_data_offsets[lod_index] = writer.position().try_into().unwrap();
			}

			index_buffer_sizes[lod_index] = read_blocks(
				block_count,
				model_header.block_index.index_buffer[lod_index],
				offset + model_header.offset.index_buffer[lod_index],
				&block_sizes,
				&mut reader,
				&mut writer,
			)?;
		}
	}

	// Write out the header now we've collected the info for it.
	writer.seek(SeekFrom::Start(0))?;
	writer.write_le(&header.block_count)?; // version
	writer.write_le(&stack_size)?;
	writer.write_le(&runtime_size)?;
	writer.write_le(&model_header.vertex_declaration_count)?;
	writer.write_le(&model_header.material_count)?;
	writer.write_le(&vertex_data_offsets)?;
	writer.write_le(&index_data_offsets)?;
	writer.write_le(&vertex_buffer_sizes)?;
	writer.write_le(&index_buffer_sizes)?;
	writer.write_le(&model_header.lod_count)?;
	writer.write_le(&model_header.index_buffer_streaming_enabled)?;
	writer.write_le(&model_header.edge_geometry_enabled)?;
	writer.write_le(&0u8)?;

	Ok(writer.into_inner())
}

fn read_blocks(
	block_count: u16,
	block_index: u16,
	section_offset: u32,
	block_sizes: &[u16],
	reader: &mut (impl Read + Seek),
	writer: &mut impl Write,
) -> Result<u32> {
	let size = (0..block_count)
		// Calculate the offsets for the blocks.
		.scan(section_offset, |offset, index| {
			let current_offset = *offset;
			*offset += u32::from(block_sizes[usize::from(block_index + index)]);
			Some(current_offset)
		})
		// Read the blocks into the cursor, recording the read byte count.
		.try_fold(0u32, |size, offset| -> Result<u32> {
			let bytes_read = io::copy(&mut read_block(reader, offset)?, writer)?;
			Ok(size + u32::try_from(bytes_read).unwrap())
		})?;

	Ok(size)
}
