use std::{
	fmt::Display,
	io::{self, Cursor, Read, Seek, SeekFrom, Write},
};

use binrw::{binread, BinRead, BinWriterExt, VecArgs};
use flate2::read::DeflateDecoder;

use crate::error::{Error, Result};

const MAX_COMPRESSED_BLOCK_SIZE: u32 = 16_000;

pub fn read_file(mut reader: impl Read + Seek, offset: u32) -> Result<Vec<u8>> {
	// Move to the start of the file and read in the header.
	reader.seek(SeekFrom::Start(offset.into()))?;
	let header = Header::read(&mut reader)?;

	let expected_file_size = header.raw_file_size;

	// TODO: if type 1 and first 64 == second 64, RSF
	//       if type 1 and first 64 == [0..], empty

	let file_offset = offset + header.size;
	let out_buffer = match &header.kind {
		FileKind::Standard => read_standard(reader, file_offset, header),
		FileKind::Model => read_model(reader, file_offset, header),
		FileKind::Texture => read_texture(reader, file_offset, header),
		_ => todo!("File kind: {:?}", header.kind),
	}?;

	match out_buffer.len() == expected_file_size.try_into().unwrap() {
		true => Ok(out_buffer),
		false => Err(Error::Resource(
			read_failed("file", expected_file_size, out_buffer.len()).into(),
		)),
	}
}

fn read_standard(mut reader: impl Read + Seek, offset: u32, header: Header) -> Result<Vec<u8>> {
	// Eagerly read the block info.
	let blocks = <Vec<BlockInfo>>::read_args(
		&mut reader,
		VecArgs {
			count: header.block_count.try_into().unwrap(),
			inner: (),
		},
	)?;

	// Read each block into a final byte vector.
	let out_buffer = blocks.iter().try_fold(
		Vec::<u8>::with_capacity(header.raw_file_size.try_into().unwrap()),
		|mut vec, block_info| -> io::Result<Vec<u8>> {
			let mut block_reader = read_block(&mut reader, offset + block_info.offset)?;

			// Check we read the expected size.
			let count = block_reader.read_to_end(&mut vec)?;
			if count != block_info.decompressed_size.into() {
				return Err(io::Error::new(
					io::ErrorKind::Other,
					read_failed("block", block_info.decompressed_size, count),
				));
			}

			Ok(vec)
		},
	)?;

	Ok(out_buffer)
}

fn read_model(mut reader: impl Read + Seek, offset: u32, header: Header) -> Result<Vec<u8>> {
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
	let mut vertex_data_offsets = [0u32; 3];
	let mut vertex_buffer_sizes = [0u32; 3];

	let mut index_data_offsets = [0u32; 3];
	let mut index_buffer_sizes = [0u32; 3];

	for lod_index in 0..3 {
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

fn read_texture(mut reader: impl Read + Seek, offset: u32, header: Header) -> Result<Vec<u8>> {
	// Eagerly read the block info.
	let blocks = <Vec<LodBlockInfo>>::read_args(
		&mut reader,
		VecArgs {
			count: header.block_count.try_into().unwrap(),
			inner: (),
		},
	)?;

	// Directly after the block info, texture files have a table of sub-block offsets.
	let sub_block_count = blocks
		.iter()
		.fold(0, |total, block| total + block.block_count);

	let sub_block_offsets = <Vec<u16>>::read_args(
		&mut reader,
		VecArgs {
			count: sub_block_count.try_into().unwrap(),
			inner: (),
		},
	)?;

	// Create a vec with capacity for the full file.
	let mut out_basis = Vec::<u8>::with_capacity(header.raw_file_size.try_into().unwrap());

	// If the first block has an offset, it's likely that there's a .tex header
	// outside the compressed blocks - read the delta into the buffer as raw bytes.
	let raw_header_size = blocks[0].compressed_offset;
	if raw_header_size > 0 {
		reader.seek(SeekFrom::Start(offset.into()))?;
		reader
			.by_ref()
			.take(raw_header_size.into())
			.read_to_end(&mut out_basis)?;
	}

	let out_buffer = blocks
		.iter()
		// Scan the LOD sub block info to get the expected offsets.
		.flat_map(|lod_block| {
			let index_offset = usize::try_from(lod_block.block_offset).unwrap();
			(index_offset..usize::try_from(lod_block.block_count).unwrap() + index_offset).scan(
				lod_block.compressed_offset + offset,
				|next, index| {
					let offset = *next;
					*next += u32::from(sub_block_offsets[index]);
					Some(offset)
				},
			)
		})
		// Read the block data.
		.try_fold(out_basis, |mut vec, offset| -> io::Result<Vec<u8>> {
			let mut block_reader = read_block(&mut reader, offset)?;
			block_reader.read_to_end(&mut vec)?;
			Ok(vec)
		})?;

	Ok(out_buffer)
}

fn read_failed(item: impl Display, expected: impl Display, got: impl Display) -> String {
	format!("Failed to read {item}. Expected {expected} bytes, got {got}.",)
}

// TODO: move this into a block struct of some kind if we do lazy reading?
fn read_block<R: Read + Seek>(reader: &mut R, offset: u32) -> io::Result<BlockReader<R>> {
	// Seek to the block and read its header so we know how much to expect in the rest of the block.
	reader.seek(SeekFrom::Start(offset.into()))?;
	let block_header =
		BlockHeader::read(reader).map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

	// TODO: Look into the padding on compressed blocks, there's some funky stuff going on in some cases. Ref. Coinach/IO/File & Lumina.

	// Build a reader for the block.
	let reader = match block_header.compressed_size > MAX_COMPRESSED_BLOCK_SIZE {
		true => BlockReader::Loose(reader.take(block_header.decompressed_size.into())),
		false => BlockReader::Compressed(DeflateDecoder::new(
			reader.take(block_header.compressed_size.into()),
		)),
	};

	Ok(reader)
}

enum BlockReader<'a, R> {
	Loose(io::Take<&'a mut R>),
	Compressed(DeflateDecoder<io::Take<&'a mut R>>),
}

impl<R: Read> Read for BlockReader<'_, R> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match self {
			Self::Loose(reader) => reader.read(buf),
			Self::Compressed(reader) => reader.read(buf),
		}
	}
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct Header {
	size: u32,
	kind: FileKind,
	raw_file_size: u32,
	// num_blocks: u32,
	// block_buffer_size: u32,
	#[br(pad_before = 8)]
	block_count: u32,
}

#[binread]
#[derive(Debug)]
#[br(little, repr = u32)]
enum FileKind {
	Empty = 1,
	Standard,
	Model,
	Texture,
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct BlockInfo {
	offset: u32,
	_compressed_size: u16,
	decompressed_size: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct LodBlockInfo {
	compressed_offset: u32,
	_compressed_size: u32,
	_decompressed_size: u32,
	block_offset: u32,
	block_count: u32,
}

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

// todo: the 3s are due to the lod count. abstract?
#[binread]
#[derive(Debug)]
#[br(little)]
struct SectionInfo<T: BinRead<Args = ()> + 'static> {
	stack: T,
	runtime: T,
	vertex_buffer: [T; 3],
	edge_geometry_vertex_buffer: [T; 3],
	index_buffer: [T; 3],
}

#[binread]
#[derive(Debug)]
#[br(little)]
struct BlockHeader {
	_size: u32,
	// unknown1: u32,
	#[br(pad_before = 4)]
	compressed_size: u32,
	decompressed_size: u32,
}
