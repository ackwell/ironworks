use std::io::{self, Cursor, Read, Seek, SeekFrom};

use binrw::{binread, BinRead, VecArgs};

use crate::error::Result;

use super::shared::{BlockHeader, BlockPayload, Header};

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

#[derive(Debug)]
pub struct BlockMetadata {
	pub input_offset: usize,
	pub input_size: usize,
	pub output_offset: usize,
	pub output_size: usize,
}

#[derive(Debug)]
pub struct BlockStream<R> {
	/// Reader for the full dat file that the sqpack file is being read from.
	dat_reader: R,
	/// Offset within the block data that should be considered the "Start" of the stream.
	origin: usize,
	/// Metadata about the blocks comprising the file.
	metadata: Vec<BlockMetadata>,

	/// Stream's position within the sqpack file.
	position: usize,
	/// Block index currently being read.
	current_block: usize,
	/// Cached reader for the current block.
	block_data: Option<Cursor<Vec<u8>>>,
}

impl<R> BlockStream<R>
where
	R: Read + Seek,
{
	pub fn new(dat_reader: R, origin: usize, metadata: Vec<BlockMetadata>) -> Self {
		// TODO: i can probably omit any metadata that exists purely prior to the origin. that said, I control all consumers - so only bother doing this if it would actually be useful.

		Self {
			dat_reader,
			origin,
			metadata,

			position: 0,
			current_block: 0,
			block_data: None,
		}
	}
}

impl<R> Read for BlockStream<R>
where
	R: Read + Seek,
{
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		// Get a ref to the expected current block metadata.
		let mut meta = &self.metadata[self.current_block];

		// The actual read position within the blocks needs to be offset by the origin.
		let position = self.position + self.origin;

		// If we've reached the end of the last block, signal EOF.
		if position == meta.output_offset + meta.output_size
			&& self.current_block == self.metadata.len() - 1
		{
			return Ok(0);
		}

		// If the position has moved outside of the current block, update to a block
		// that contains the expected position.
		if position < meta.output_offset || position >= meta.output_offset + meta.output_size {
			let (new_index, new_meta) = self
				.metadata
				.iter()
				.enumerate()
				.find(|(_index, meta)| {
					position >= meta.output_offset
						&& position < meta.output_offset + meta.output_size
				})
				.ok_or_else(|| {
					io::Error::new(
						io::ErrorKind::InvalidInput,
						"reader position outside known range",
					)
				})?;

			self.current_block = new_index;
			meta = new_meta;

			self.block_data = None;
		}

		// Ensure that the block we're reading from has been read into cache. This
		// is implemented as a match so internals can be shortcut out.
		let block = match &mut self.block_data {
			Some(value) => value,
			None => {
				// Seek to the start of the payload and read it in.
				self.dat_reader
					.seek(SeekFrom::Start(meta.input_offset.try_into().unwrap()))?;
				let mut reader = BlockPayload::new(
					&mut self.dat_reader,
					meta.input_size.try_into().unwrap(),
					meta.output_size.try_into().unwrap(),
				);

				let mut buffer = Vec::with_capacity(meta.output_size);
				let count = reader.read_to_end(&mut buffer)?;

				// Check we read the expected size.
				if count != meta.output_size {
					return Err(io::Error::new(
						io::ErrorKind::Other,
						format!(
							"failed to read block: expected {} bytes, got {}",
							meta.output_size, count
						),
					));
				}

				self.block_data.insert(Cursor::new(buffer))
			}
		};

		// The position may have changed externally since the last read, seek to the
		// expected position within the block cache before reading - given the cache
		// is a cursor, this is a cheap operation.
		block.set_position((position - meta.output_offset).try_into().unwrap());

		// TODO: Do I need to handle an `Ok(0)` at this point or is returning it to the consumer fine?
		let bytes_read = block.read(buf)?;
		self.position += bytes_read;
		Ok(bytes_read)
	}
}

impl<R> Seek for BlockStream<R> {
	fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
		let (base, position) = match position {
			SeekFrom::Start(position) => {
				self.position = position.try_into().unwrap();
				return Ok(position);
			}
			SeekFrom::Current(position) => (self.position, position),
			SeekFrom::End(position) => {
				let base = match self.metadata.last() {
					Some(meta) => meta.output_offset + meta.output_size,
					None => 0,
				};
				(base, position)
			}
		};

		// All of this because the easy way is unstable. Still.
		let ibase = i64::try_from(base).unwrap();
		let ioffset = ibase.checked_add(position).ok_or_else(|| {
			io::Error::new(
				io::ErrorKind::InvalidInput,
				"invalid seek to an overflowing position",
			)
		})?;
		if ioffset < 0 {
			return Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				"invalid seek to a negative position",
			));
		}
		let offset = u64::try_from(ioffset).unwrap();
		self.position = offset.try_into().unwrap();
		Ok(offset)
	}
}
