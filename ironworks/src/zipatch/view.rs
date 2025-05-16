use std::{
	collections::HashMap,
	fs,
	io::{self, BufReader, Cursor, Seek, SeekFrom},
	sync::Arc,
};

use either::Either;

use crate::{
	error::{Error, ErrorValue, Result},
	sqpack,
	utility::{TakeSeekable, TakeSeekableExt},
};

use super::{
	lookup::{FileChunk, PatchLookup, ResourceChunk, SqPackFileExtension, SqPackSpecifier},
	repository::PatchRepository,
	zipatch::LookupCache,
};

type FileReader =
	Either<TakeSeekable<BufReader<fs::File>>, sqpack::BlockStream<BufReader<fs::File>>>;

#[derive(Debug)]
pub struct ViewBuilder {
	repositories: HashMap<u8, Arc<PatchRepository>>,
	cache: Arc<LookupCache>,
}

impl ViewBuilder {
	pub(super) fn new(cache: Arc<LookupCache>) -> Self {
		Self {
			repositories: Default::default(),
			cache,
		}
	}

	/// Add a patch repository for the given SqPack repository ID.
	pub fn with_repository(mut self, id: u8, repository: PatchRepository) -> Self {
		self.add_repository(id, repository);
		self
	}

	/// Add a patch repository for the given SqPack repository ID.
	pub fn add_repository(&mut self, id: u8, repository: PatchRepository) {
		self.repositories.insert(id, Arc::new(repository));
	}

	pub fn build(self) -> View {
		View::new(self.repositories, self.cache)
	}
}

/// A snapshot into the data available in patch files as of a specified set of patches.
#[derive(Debug)]
pub struct View {
	repositories: HashMap<u8, Arc<PatchRepository>>,
	cache: Arc<LookupCache>,
}

impl View {
	pub(super) fn new(
		repositories: HashMap<u8, Arc<PatchRepository>>,
		cache: Arc<LookupCache>,
	) -> Self {
		Self {
			repositories,
			cache,
		}
	}

	fn lookups(
		&self,
		repository_id: u8,
	) -> Result<impl Iterator<Item = Result<Arc<PatchLookup>>> + '_> {
		let repository = self.repositories.get(&repository_id).ok_or_else(|| {
			Error::NotFound(ErrorValue::Other(format!("repository {repository_id}")))
		})?;

		// We're operating at a patch-by-patch granularity here, with the (very safe)
		// assumption that a game version is at minimum one patch.
		let iterator = repository
			.patches
			.iter()
			.rev()
			.map(move |patch| self.cache.lookup(patch));

		Ok(iterator)
	}

	fn read_index(
		&self,
		repository: u8,
		category: u8,
		chunk: u8,
		index_version: u8,
	) -> Result<Cursor<Vec<u8>>> {
		let target_specifier = SqPackSpecifier {
			repository,
			category,
			chunk,
			extension: SqPackFileExtension::Index(index_version),
		};

		let mut empty = true;
		let mut cursor = Cursor::new(Vec::<u8>::new());

		for maybe_lookup in self.lookups(repository)? {
			// Grab the commands for the requested target, if any exist in this patch.
			let lookup = maybe_lookup?;
			let chunks = match lookup.data.file_chunks.get(&target_specifier) {
				Some(chunks) => chunks,
				None => continue,
			};

			// Read the commands for this patch.
			let mut file = BufReader::new(fs::File::open(&lookup.path)?);
			for chunk in chunks.iter() {
				empty = false;
				cursor.set_position(chunk.target_offset);

				for block in chunk.blocks.iter() {
					file.seek(SeekFrom::Start(block.source_offset))?;
					let mut reader = sqpack::BlockPayload::new(
						&mut file,
						block.compressed_size,
						block.decompressed_size,
					);
					io::copy(&mut reader, &mut cursor)?;
				}
			}

			// ASSUMPTION: The offset:0 (first) chunk for a file, even if split across
			// multiple patches, will _always_ be the first chunk touching that file
			// within the patch it is in, as any prior file operations would be negated
			// by the truncation of the file caused by an offset:0 chunk.

			// If this patch started with offset:0, we can stop reading.
			if !chunks.is_empty() && chunks[0].target_offset == 0 {
				break;
			}
		}

		// If nothing was read, we mark this index as not found.
		if empty {
			// TODO: Improve the error value.
			return Err(Error::NotFound(ErrorValue::Other(format!(
				"zipatch target {target_specifier:?}"
			))));
		}

		// Done - reset the cursor's position and return it as a view of the index.
		cursor.set_position(0);
		Ok(cursor)
	}
}

impl sqpack::Resource for View {
	fn version(&self, repository_id: u8) -> Result<String> {
		let repository = self.repositories.get(&repository_id).ok_or_else(|| {
			Error::NotFound(ErrorValue::Other(format!("repository {repository_id}")))
		})?;

		repository
			.patches
			.last()
			.map(|x| x.name.clone())
			.ok_or_else(|| {
				Error::Invalid(
					ErrorValue::Other(format!("repository {repository_id}")),
					"unspecified repository version".to_string(),
				)
			})
	}

	// ASSUMPTION: IndexUpdate chunks are unused, new indexes will always be distributed via FileOperation::AddFile.
	type Index = Cursor<Vec<u8>>;
	fn index(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index> {
		self.read_index(repository, category, chunk, 1)
	}

	type Index2 = Cursor<Vec<u8>>;
	fn index2(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index2> {
		self.read_index(repository, category, chunk, 2)
	}

	type File = FileReader;
	fn file(&self, repository: u8, category: u8, location: sqpack::Location) -> Result<Self::File> {
		let target = (
			SqPackSpecifier {
				repository,
				category,
				chunk: location.chunk(),
				extension: SqPackFileExtension::Dat(location.data_file()),
			},
			location.offset(),
		);

		for maybe_lookup in self.lookups(repository)? {
			let lookup = maybe_lookup?;

			// Try to get the file from the add commands first.
			// ASSUMPTION: Square seemingly never breaks new files up across multiple
			// chunks - an entire file can be read by looking for the single add
			// command starting at the precise offset we're looking for.
			if let Some(command) = lookup.data.resource_chunks.get(&target) {
				return read_resource_chunk(&lookup, command);
			};

			// Check if the file could be found in the file operations.
			// ASSUMPTION: Target sqpack files read from a FileCommand-provided .dat
			// file will not be split across a patch file boundary. While this is
			// realistically possible, the chances of it occuring are vanishingly
			// remote. If everything has blown up in your face because of this and you
			// find this comment, bap me.
			// NOTE: Broken in 7.0 HIST for ui/icon/195000/195006_hr1.tex
			if let Some(chunks) = lookup.data.file_chunks.get(&target.0) {
				// File chunks for one target dat file may be spread across multiple
				// patch files - if the target couldn't be found in this lookup, continue
				// to the next.
				match find_file_blocks(&location, chunks) {
					Err(Error::NotFound(_)) => {}
					Err(error) => return Err(error),
					Ok(metadata) => {
						// Build the readers & complete
						let file_reader = BufReader::new(fs::File::open(&lookup.path)?);
						let block_stream = sqpack::BlockStream::new(
							file_reader,
							location.offset().try_into().unwrap(),
							metadata,
						);

						return Ok(Either::Right(block_stream));
					}
				};
			};
		}

		Err(Error::NotFound(ErrorValue::Other(format!(
			"zipatch target {:?}",
			target
		))))
	}
}

fn read_resource_chunk(lookup: &PatchLookup, command: &ResourceChunk) -> Result<FileReader> {
	let mut file = BufReader::new(fs::File::open(&lookup.path)?);
	file.seek(SeekFrom::Start(command.offset))?;
	let out = file.take_seekable(command.size)?;
	Ok(Either::Left(out))
}

fn find_file_blocks(
	location: &sqpack::Location,
	chunks: &[FileChunk],
) -> Result<Vec<sqpack::BlockMetadata>> {
	let outside_target = |offset: u64, size: u64| {
		// If the size is available, filter out commands that sit beyond that size -
		// otherwise, assume the file could be infintely long.
		let before_end = location
			.size()
			.map(|size| offset < (location.offset() + size).into())
			.unwrap_or(true);

		let after_start = (offset + size) > location.offset().into();

		after_start && before_end
	};

	// Build an iterator over the commands. We're filtering any commands that sit
	// outside the target range to minimise further processing.
	let chunks_iter = chunks
		.iter()
		.filter(|chunk| outside_target(chunk.target_offset, chunk.target_size));

	// Extract the metadata for each block in each command.
	let block_iter = chunks_iter.flat_map(|chunk| {
		chunk.blocks.iter().scan(0u64, |file_offset, block| {
			let current_offset = *file_offset;
			*file_offset += u64::from(block.decompressed_size);

			Some(sqpack::BlockMetadata {
				input_offset: block.source_offset.try_into().unwrap(),
				input_size: block.compressed_size.try_into().unwrap(),
				output_offset: (chunk.target_offset + current_offset).try_into().unwrap(),
				output_size: block.decompressed_size.try_into().unwrap(),
			})
		})
	});

	// ASSUMPTION: FileOperation commands will apply their data in sequential order.

	// Do another pass, filtering out any remaining metadata (from AddFile blocks)
	// that fall entirely outside the target range.
	let metadata = block_iter
		.filter(|meta| {
			outside_target(
				meta.output_offset.try_into().unwrap(),
				meta.output_size.try_into().unwrap(),
			)
		})
		.collect::<Vec<_>>();

	// If there are 0 blocks that match the target, the provided lookup does not
	// contain the requested target.
	if metadata.is_empty() {
		return Err(Error::NotFound(ErrorValue::Other(format!(
			"sqpack location {location:?}"
		))));
	}

	Ok(metadata)
}
