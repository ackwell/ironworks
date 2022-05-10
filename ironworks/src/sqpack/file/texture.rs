use std::io::{self, Read, Seek, SeekFrom};

use binrw::{binread, BinRead, VecArgs};

use crate::error::Result;

use super::shared::{read_block, Header};

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

pub fn read(mut reader: impl Read + Seek, offset: u32, header: Header) -> Result<Vec<u8>> {
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
