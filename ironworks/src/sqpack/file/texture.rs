use std::io::{self, Cursor, Read, Seek, SeekFrom};

use binrw::{binread, BinRead, VecArgs};

use crate::error::Result;

use super::shared::{read_block, Header};

#[binread]
#[br(little)]
#[derive(Debug)]
struct SurfaceBlockInfo {
	compressed_offset: u32,
	_compressed_size: u32,
	_decompressed_size: u32,
	block_offset: u32,
	block_count: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct TexHeader {
	attribute: u32,
	// format: u32,
	// width: u16,
	// height: u16,
	// depth: u16,
	// mip_levels: u16,
	// lod_offsets: [u32; 3],
	#[br(pad_before = 24)]
	surface_offsets: [u32; 13],
}

pub fn read(mut reader: impl Read + Seek, offset: u32, header: Header) -> Result<Vec<u8>> {
	// Eagerly read the block info.
	let blocks = <Vec<SurfaceBlockInfo>>::read_args(
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

	// Create a writer with capacity for the full file.
	let mut writer = Cursor::new(Vec::<u8>::with_capacity(
		header.raw_file_size.try_into().unwrap(),
	));

	// If the first block has an offset, it's likely that there's a .tex header
	// outside the compressed blocks - read it in for further info, and pass it
	// over into the writer.
	let mut texture_header = None::<TexHeader>;
	let raw_header_size = blocks[0].compressed_offset;
	if raw_header_size > 0 {
		reader.seek(SeekFrom::Start(offset.into()))?;
		texture_header = Some(TexHeader::read(&mut reader)?);

		reader.seek(SeekFrom::Start(offset.into()))?;
		io::copy(
			&mut reader.by_ref().take(raw_header_size.into()),
			&mut writer,
		)?;
	}

	// The attribute field is a bitset, index 26 signifies if the texture is a cube. Check `file/tex` for full bitset definition.
	let stride = match texture_header {
		Some(TexHeader { attribute, .. }) if (attribute >> 25) & 1 == 1 => 6,
		_ => 1,
	};

	for (index, block) in blocks.iter().enumerate() {
		// Move to the expected start position of the block.
		if let Some(ref header) = texture_header {
			if index % stride == 0 {
				writer.set_position(header.surface_offsets[index / stride].into());
			}
		}

		// Read each sub-block into the writer.
		let mut data_offset = block.compressed_offset + offset;
		for sub_block_offset in sub_block_offsets
			.iter()
			.skip(usize::try_from(block.block_offset).unwrap())
			.take(usize::try_from(block.block_count).unwrap())
		{
			io::copy(&mut read_block(&mut reader, data_offset)?, &mut writer)?;
			data_offset += u32::from(*sub_block_offset);
		}
	}

	Ok(writer.into_inner())
}
