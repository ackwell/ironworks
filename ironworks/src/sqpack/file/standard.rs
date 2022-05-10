use std::io::{self, Read, Seek};

use binrw::{binread, BinRead, VecArgs};

use crate::error::Result;

use super::shared::{read_block, read_failed, Header};

#[binread]
#[derive(Debug)]
#[br(little)]
struct BlockInfo {
	offset: u32,
	_compressed_size: u16,
	decompressed_size: u16,
}

pub fn read(mut reader: impl Read + Seek, offset: u32, header: Header) -> Result<Vec<u8>> {
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
