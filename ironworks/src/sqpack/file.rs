use std::{
	fmt::Display,
	io::{self, Cursor, Read, Seek, SeekFrom},
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
	// todo we're technically starting halfway through the header here - models seem to have a different header? slightly?
	// todo naming is copied from lumina rn - fix up and standardise with textures
	#[binread]
	#[derive(Debug)]
	#[br(little)]
	struct Test {
		size: TestInner<u32>,
		compressed_size: TestInner<u32>,
		offset: TestInner<u32>,
		block_index: TestInner<u16>,
		block_num: TestInner<u16>,
		vertex_declaration_num: u16,
		material_num: u16,
		num_lods: u8,
		index_buffer_streaming_enabled: u8, //bool
		edge_geometry_enabled: u8,          //bool
		padding: u8,
	}

	// todo: the 3s are due to the lod count. abstract?
	#[binread]
	#[derive(Debug)]
	#[br(little)]
	struct TestInner<T: BinRead<Args = ()> + 'static> {
		stack: T,
		runtime: T,
		vertex_buffer: [T; 3],
		edge_geometry_vertex_buffer: [T; 3],
		index_buffer: [T; 3],
	}

	let test = Test::read(&mut reader)?;

	// Model header is followed by an array of block sizes.
	let block_nums = &test.block_num;
	let total_blocks = block_nums.stack
		+ block_nums.runtime
		+ block_nums.vertex_buffer.iter().sum::<u16>()
		+ block_nums.edge_geometry_vertex_buffer.iter().sum::<u16>()
		+ block_nums.index_buffer.iter().sum::<u16>();

	// TODO: i should probably make an impl for this it's pretty repetetive
	let block_sizes = <Vec<u16>>::read_args(
		&mut reader,
		VecArgs {
			count: total_blocks.try_into().unwrap(),
			inner: (),
		},
	)?;

	let out_buffer = Vec::<u8>::with_capacity(header.raw_file_size.try_into().unwrap());
	let mut out_cursor = Cursor::new(out_buffer);

	// First 0x44 is the header, which will be filled at the end
	out_cursor.seek(SeekFrom::Start(0x44))?;

	// everything below this point is disgusting and needs to be cleaned up

	// stack
	let mut running = 0;
	let mut stack_size = 0;
	for index in 0..test.block_num.stack {
		let mut block_reader = read_block(&mut reader, offset + test.offset.stack + running)?;
		stack_size += io::copy(&mut block_reader, &mut out_cursor)?;

		let jndex = test.block_index.stack + index;
		running += u32::from(block_sizes[usize::from(jndex)]);
	}

	// runtime
	let mut running = 0;
	let mut runtime_size = 0;
	for index in 0..test.block_num.runtime {
		let mut block_reader = read_block(&mut reader, offset + test.offset.runtime + running)?;
		runtime_size += io::copy(&mut block_reader, &mut out_cursor)?;

		let jndex = test.block_index.runtime + index;
		running += u32::from(block_sizes[usize::from(jndex)]);
	}

	// stuff with lod levels
	let mut vertex_data_offsets = [0u32; 3];
	let mut vertex_buffer_sizes = [0u32; 3];

	let mut index_data_offsets = [0u32; 3];
	let mut index_buffer_sizes = [0u32; 3];

	for lod_index in 0..3 {
		if test.block_num.vertex_buffer[lod_index] != 0 {
			// todo handle storing position for the header
			let block_num = test.block_num.vertex_buffer[lod_index];
			if lod_index == 0 || block_num > 0 {
				vertex_data_offsets[lod_index] = out_cursor.position().try_into().unwrap();
			}

			let mut running = 0;
			for index in 0..block_num {
				let mut block_reader = read_block(
					&mut reader,
					offset + test.offset.vertex_buffer[lod_index] + running,
				)?;
				vertex_buffer_sizes[lod_index] +=
					u32::try_from(io::copy(&mut block_reader, &mut out_cursor)?).unwrap();

				let jndex = test.block_index.vertex_buffer[lod_index] + index;
				running += u32::from(block_sizes[usize::from(jndex)]);
			}
		}

		if test.block_num.edge_geometry_vertex_buffer[lod_index] != 0 {
			let mut running = 0;
			for index in 0..test.block_num.edge_geometry_vertex_buffer[lod_index] {
				let mut block_reader = read_block(
					&mut reader,
					offset + test.offset.edge_geometry_vertex_buffer[lod_index] + running,
				)?;
				io::copy(&mut block_reader, &mut out_cursor)?;

				let jndex = test.block_index.edge_geometry_vertex_buffer[lod_index] + index;
				running += u32::from(block_sizes[usize::from(jndex)]);
			}
		}

		if test.block_num.index_buffer[lod_index] != 0 {
			// todo handle storing position for the header
			let block_num = test.block_num.index_buffer[lod_index];
			if lod_index == 0 || block_num > 0 {
				index_data_offsets[lod_index] = out_cursor.position().try_into().unwrap();
			}

			let mut running = 0;
			for index in 0..block_num {
				let mut block_reader = read_block(
					&mut reader,
					offset + test.offset.index_buffer[lod_index] + running,
				)?;
				index_buffer_sizes[lod_index] +=
					u32::try_from(io::copy(&mut block_reader, &mut out_cursor)?).unwrap();

				let jndex = test.block_index.index_buffer[lod_index] + index;
				running += u32::from(block_sizes[usize::from(jndex)]);
			}
		}
	}

	// header shit
	out_cursor.seek(SeekFrom::Start(0))?;
	out_cursor.write_le(&header.block_count)?; // version
	out_cursor.write_le(&u32::try_from(stack_size).unwrap())?;
	out_cursor.write_le(&u32::try_from(runtime_size).unwrap())?;
	out_cursor.write_le(&test.vertex_declaration_num)?;
	out_cursor.write_le(&test.material_num)?;
	out_cursor.write_le(&vertex_data_offsets)?;
	out_cursor.write_le(&index_data_offsets)?;
	out_cursor.write_le(&vertex_buffer_sizes)?;
	out_cursor.write_le(&index_buffer_sizes)?;
	out_cursor.write_le(&test.num_lods)?;
	out_cursor.write_le(&test.index_buffer_streaming_enabled)?;
	out_cursor.write_le(&test.edge_geometry_enabled)?;
	out_cursor.write_le(&0u8)?;

	Ok(out_cursor.into_inner())
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
struct BlockHeader {
	_size: u32,
	// unknown1: u32,
	#[br(pad_before = 4)]
	compressed_size: u32,
	decompressed_size: u32,
}
