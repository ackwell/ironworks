use std::io::{self, Cursor, Read, Seek, SeekFrom};

use binrw::{binread, BinRead, VecArgs};

use crate::{error::Result, sqpack::file::shared::BlockReader};

use super::shared::{read_block, read_failed, Header};

#[binread]
#[derive(Debug)]
#[br(little)]
struct BlockInfo {
	#[br(try_map = |value: u32| value.try_into())]
	offset: usize,
	// TODO: rename these
	// represents the size in the sqpack archive that this block occupies
	_compressed_size: u16,
	// represents the data size
	#[br(map = |value: u16| value.into())]
	decompressed_size: usize,
}

pub fn read_old(mut reader: impl Read + Seek, offset: u32, header: Header) -> Result<Vec<u8>> {
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
			let mut block_reader = read_block(
				&mut reader,
				offset + u32::try_from(block_info.offset).unwrap(),
			)?;

			let fffff = match block_reader {
				BlockReader::Compressed(_) => "Compressed",
				_ => "Loose",
			};
			println!(
				"READ: {fffff:?}: {}, {}",
				block_info._compressed_size, block_info.decompressed_size
			);

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

// todo this should probably be readerthing::new or something to that tune
// todo: should that offset be a usize? will need changes up the code path obviosuly
pub fn read(mut reader: impl Read + Seek, offset: u32, header: Header) -> Result<Vec<u8>> {
	// Eagerly read the block info.
	let blocks = <Vec<BlockInfo>>::read_args(
		&mut reader,
		VecArgs {
			count: header.block_count.try_into().unwrap(),
			inner: (),
		},
	)?;

	// let readers = blocks
	// 	.into_iter()
	// 	.map(|block_info| -> io::Result<_> {
	// 		let reader = read_block(&mut reader, offset + block_info.offset)?;

	// 		Ok(reader)
	// 	})
	// 	.collect::<io::Result<Vec<_>>>()?;

	let mut r = TodoNameReaderThing {
		reader,
		offset,
		metadata: blocks,
		position: 0,
		current_block: 0,
		block_data: None,
	};

	let mut temp_buf = Vec::new();
	r.read_to_end(&mut temp_buf)?;

	Ok(temp_buf)
}

/*
thoughts;
- can't seek within a single block, given it might be deflated, and hence not seekable
- can make a struct that holds block info + readers, seeks can be performed based on decompressed size &c
- check if it's more performant to only hold buffer for most recent reader, or easier for all
- major spanner in the works from model, which needs read info to build the header, which is at the start!
- the size values should be... doable, given we know the block count + offset for each of them, and can calculate the combined size from that
- position could be gnarly? i mean should be _doable_ given the data from above
*/

struct TodoNameReaderThing<R> {
	// reader for the dat file
	reader: R,
	// offset from the start of the dat file that this file actually starts
	offset: u32,
	// data about the blocks we're going to read
	metadata: Vec<BlockInfo>,
	// position within _this_ file
	position: usize,
	current_block: usize,
	// do.. i want to cache _all_ opened blocks, or just the most recent? probably only most recent
	block_data: Option<Cursor<Vec<u8>>>,
	// we'll need
	// - position of this reader
	// - metadata for the blocks we intend to read - possibly only need offsets? nope. offsets + sizes
	//   - standard file type includes block sizes in it's header which might be useful - but model doesn't have this
	//   - yes it does, but it's not quite as ergonomic - still possible though
	// dont need
	// - position of the inner reader, it's tracked seperately
}

impl<R> TodoNameReaderThing<R> {}

impl<R> Read for TodoNameReaderThing<R>
where
	R: Read + Seek,
{
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		// get the current position
		let position = self.position;
		let mut meta = &self.metadata[self.current_block];

		// check if the current position can be obtained from the current block (or if there is 0 bytes left)
		// todo: shit about something about fucking all the usize shit i need and shit
		// todo: can probably use try_map in binrw to avoid this shit. also not-try map
		// TODO: NOPE THIS IS WRONG< OFFSET IS POSITION IN THE COMPRESSED STREAM, NEED TO MAINTAIN POSITION IN DECOMP STREAM TOO
		// let min_bound = meta.offset;
		// TODO: clean this up, and this can probably be pre-computed once wheneever current block changes
		let mut min_bound = self
			.metadata
			.iter()
			.take(self.current_block)
			.fold(0usize, |acc, cur| acc + cur.decompressed_size);
		let mut max_bound = min_bound + meta.decompressed_size;

		// if the position is at the end of the file we can return 0 to signal the eof
		if position == max_bound && self.current_block == self.metadata.len() - 1 {
			println!("EOF!");
			return Ok(0);
		}

		if position < min_bound || position >= max_bound {
			// if it isn't, update to the correct block position and clear caches (unless keeping all?)
			// find which block the position is actually in

			// let fsda = self
			// 	.metadata
			// 	.iter()
			// 	.find(|info| position >= info.offset && position);
			let new_block = self
				.metadata
				.iter()
				.scan(0usize, |acc, cur| {
					let decom_off = *acc;
					*acc += cur.decompressed_size;
					Some((cur, decom_off))
				})
				.enumerate()
				.find(|(_index, (info, decomp_offset))| {
					position >= *decomp_offset && position < decomp_offset + info.decompressed_size
				});

			// what if it's not in any block?
			let (index, (info, decompofffset)) = match new_block {
				Some(v) => v,
				None => todo!(
					"raise error about block not being found, should be an io error for oob access"
				),
			};

			// update the block index
			self.current_block = index;
			// update the meta shit (this will need changes on the bounds too?)
			meta = info;
			min_bound = decompofffset;
			max_bound = min_bound + info.decompressed_size;
			// clear the cursor cache
			self.block_data = None;

			// todo!("update block! {self:#?}");
		}

		// check if the current block is cached
		// something something not using get_or_insert because i need the ? shit
		let block = match &mut self.block_data {
			Some(value) => value,
			None => {
				println!("READING BLOCK {meta:?}");
				let mut reader = read_block(
					&mut self.reader,
					self.offset + u32::try_from(meta.offset).unwrap(),
				)?;
				let mut buffer = Vec::with_capacity(meta.decompressed_size);
				reader.read_to_end(&mut buffer)?;
				self.block_data.insert(Cursor::new(buffer))
			}
		};

		// read from the buffer (how much?)
		// how much -> Min(buf.len, bytes to next block)
		// let remaining_stuff = max_bound - position;
		// let fuck = buf.len();
		// let shit = std::cmp::min(remaining_stuff, fuck);
		// todo: this... _is_ supposed to read into it from 0... right?
		// todo: to the actual read, lmao? am i going to be better off using a cursor for the buffer and relying on stuff like io::copy to move data from the cursor into the vuffer? or rather, the read ig uess...

		// ensure we're at the current pos within current block because we have no idea if block changed since last read
		// todo: seeking a cursor is effectively free, right?
		block.seek(SeekFrom::Start((position - min_bound).try_into().unwrap()))?;

		let bytes_read = block.read(buf)?;

		if bytes_read == 0 {
			todo!("what the fuck happened here? {self:#?}");
		}

		// update position
		self.position += bytes_read;

		Ok(bytes_read)
	}
}

impl<R> core::fmt::Debug for TodoNameReaderThing<R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TodoNameReaderThing")
			// .field("reader", &self.reader)
			.field("offset", &self.offset)
			.field("metadata", &self.metadata)
			.field("position", &self.position)
			.field("current_block", &self.current_block)
			.field(
				"block_data",
				match &self.block_data {
					None => &"NONE",
					Some(_) => &"SOME",
				},
			)
			.finish()
	}
}
