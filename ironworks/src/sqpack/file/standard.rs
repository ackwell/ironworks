use std::io::{Read, Seek, SeekFrom};

use binrw::{BinRead, VecArgs, binread};

use crate::{
	error::Result,
	sqpack::block::{BlockHeader, BlockMetadata, BlockStream},
};

use super::shared::Header;

#[binread]
#[derive(Debug)]
#[br(little)]
struct BlockInfo {
	offset: u32,
	_input_size: u16,
	output_size: u16,
}

pub fn read<R: Read + Seek>(mut reader: R, offset: u32, header: Header) -> Result<BlockStream<R>> {
	// Eagerly read the block info.
	let blocks = <Vec<BlockInfo>>::read_args(
		&mut reader,
		VecArgs {
			count: header.block_count.try_into().unwrap(),
			inner: (),
		},
	)?;

	// Closure for subsequent scan to allow cleaner error handling.
	let mut read_block_metadata = |previous: &mut usize, info: &BlockInfo| -> Result<_> {
		let output_offset = *previous;
		*previous += usize::from(info.output_size);

		let header_offset = offset + info.offset;
		reader.seek(SeekFrom::Start(header_offset.into()))?;
		let header = BlockHeader::read(&mut reader)?;

		Ok(BlockMetadata {
			input_offset: (header_offset + header.size).try_into().unwrap(),
			input_size: header.compressed_size.try_into().unwrap(),
			output_offset,
			output_size: info.output_size.into(),
		})
	};

	// Read in the block headers to build the metadata needed for the reader.
	let metadata = blocks
		.iter()
		.scan(0usize, |previous, info| {
			Some(read_block_metadata(previous, info))
		})
		.collect::<Result<Vec<_>>>()?;

	Ok(BlockStream::new(reader, 0, metadata))
}
