use std::{
	collections::HashMap,
	fs,
	io::{self, BufReader, Cursor, Seek, SeekFrom},
	sync::Arc,
};

use either::Either;

use crate::{
	error::{Error, ErrorValue, Result},
	file::patch::{AddCommand, FileOperation, FileOperationCommand},
	sqpack,
	utility::{TakeSeekable, TakeSeekableExt},
};

use super::{
	lookup::{PatchLookup, SqPackFileExtension, SqPackSpecifier},
	repository::PatchRepository,
	zipatch::LookupCache,
};

// TODO: These (and path_metadata itself) should be moved into sqpack proper once and for all
const REPOSITORIES: &[&str] = &[
	"ffxiv", "ex1", "ex2", "ex3", "ex4", "ex5", "ex6", "ex7", "ex8", "ex9",
];

const CATEGORIES: &[Option<&str>] = &[
	/* 0x00 */ Some("common"),
	/* 0x01 */ Some("bgcommon"),
	/* 0x02 */ Some("bg"),
	/* 0x03 */ Some("cut"),
	/* 0x04 */ Some("chara"),
	/* 0x05 */ Some("shader"),
	/* 0x06 */ Some("ui"),
	/* 0x07 */ Some("sound"),
	/* 0x08 */ Some("vfx"),
	/* 0x09 */ Some("ui_script"),
	/* 0x0a */ Some("exd"),
	/* 0x0b */ Some("game_script"),
	/* 0x0c */ Some("music"),
	/* 0x0d */ None,
	/* 0x0e */ None,
	/* 0x0f */ None,
	/* 0x10 */ None,
	/* 0x11 */ None,
	/* 0x12 */ Some("sqpack_test"),
	/* 0x13 */ Some("debug"),
];

type FileReader =
	Either<TakeSeekable<BufReader<fs::File>>, sqpack::BlockStream<BufReader<fs::File>>>;

#[derive(Debug)]
pub struct VersionSpecifier {
	patches: HashMap<u8, String>,
}

impl VersionSpecifier {
	pub fn latest() -> Self {
		Self {
			patches: HashMap::new(),
		}
	}

	pub fn with_patches(patches: HashMap<u8, String>) -> Self {
		Self { patches }
	}
}

#[derive(Debug)]
pub struct Version {
	specifier: VersionSpecifier,
	repositories: HashMap<u8, Arc<PatchRepository>>,
	cache: Arc<LookupCache>,
}

impl Version {
	pub(super) fn new(
		specifier: VersionSpecifier,
		repositories: HashMap<u8, Arc<PatchRepository>>,
		cache: Arc<LookupCache>,
	) -> Self {
		Self {
			specifier,
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

		let target_patch = self.specifier.patches.get(&repository_id);

		// We're operating at a patch-by-patch granularity here, with the (very safe)
		// assumption that a game version is at minimum one patch.
		let iterator = repository
			.patches
			.iter()
			.rev()
			.skip_while(move |patch| {
				match target_patch {
					// None implies the latest patch available, never skip.
					None => false,
					// Skip while the patch doesn't match.
					Some(target) => *patch != target,
				}
			})
			.map(move |patch| self.cache.lookup(repository_id, repository, patch));

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
			let commands = match lookup.add_operations.get(&target_specifier) {
				Some(commands) => commands,
				None => continue,
			};

			// Read the commands for this patch.
			let mut file = BufReader::new(fs::File::open(&lookup.path)?);
			for command in commands {
				empty = false;
				cursor.set_position(command.target_offset());
				let blocks = match command.operation() {
					FileOperation::AddFile(blocks) => blocks,
					_ => unreachable!(),
				};

				for block in blocks {
					file.seek(SeekFrom::Start(block.offset()))?;
					let mut reader = sqpack::BlockPayload::new(
						&mut file,
						block.compressed_size(),
						block.decompressed_size(),
					);
					io::copy(&mut reader, &mut cursor)?;
				}
			}

			// ASSUMPTION: The offset:0 (first) chunk for a file, even if split across
			// multiple patches, will _always_ be the first chunk touching that file
			// within the patch it is in, as any prior file operations would be negated
			// by the truncation of the file caused by an offset:0 chunk.

			// If this patch started with offset:0, we can stop reading.
			if !commands.is_empty() && commands[0].target_offset() == 0 {
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

impl sqpack::Resource for Version {
	fn path_metadata(&self, path: &str) -> Option<(u8, u8)> {
		let split = path.split('/').take(2).collect::<Vec<_>>();

		match split[..] {
			[path_category, path_repository] => Some((
				REPOSITORIES
					.iter()
					.position(|repository| repository == &path_repository)
					.unwrap_or(0)
					.try_into()
					.unwrap(),
				CATEGORIES
					.iter()
					.position(|category| category == &Some(path_category))?
					.try_into()
					.unwrap(),
			)),
			_ => None,
		}
	}

	fn version(&self, repository: u8) -> Result<String> {
		self.specifier
			.patches
			.get(&repository)
			.cloned()
			.ok_or_else(|| {
				Error::Invalid(
					ErrorValue::Other(format!("repository {repository}")),
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
			if let Some(command) = lookup.add_commands.get(&target) {
				return read_add_command(&lookup, command);
			};

			// Check if the file could be found in the file operations.
			// ASSUMPTION: Target sqpack files read from a FileCommand-provided .dat
			// file will not be split across a patch file boundary. While this is
			// realistically possible, the chances of it occuring are vanishingly
			// remote. If everything has blown up in your face because of this and you
			// find this comment, bap me.
			if let Some(commands) = lookup.add_operations.get(&target.0) {
				return read_file_commands(&lookup, target.1, commands);
			};
		}

		Err(Error::NotFound(ErrorValue::Other(format!(
			"zipatch target {:?}",
			target
		))))
	}
}

fn read_add_command(lookup: &PatchLookup, command: &AddCommand) -> Result<FileReader> {
	let mut file = BufReader::new(fs::File::open(&lookup.path)?);
	file.seek(SeekFrom::Start(command.source_offset()))?;
	let out = file.take_seekable(command.data_size().into())?;
	Ok(Either::Left(out))
}

fn read_file_commands(
	lookup: &PatchLookup,
	offset: u32,
	commands: &[FileOperationCommand],
) -> Result<FileReader> {
	// Build an iterator over the commands. We're skipping any commands that sit
	// entirely before the target offset to minimise how much needs to be read.
	let commands_iter = commands
		.iter()
		.skip_while(|command| (command.target_offset() + command.target_size()) < offset.into());

	// Extract the metadata for each block in each command.
	let block_iter = commands_iter.flat_map(|command| {
		let blocks = match command.operation() {
			FileOperation::AddFile(blocks) => blocks,
			other => panic!("unexpected {other:?}"),
		};

		blocks.iter().scan(0u64, |file_offset, block| {
			let current_offset = *file_offset;
			*file_offset += u64::from(block.decompressed_size());

			Some(sqpack::BlockMetadata {
				input_offset: block.offset().try_into().unwrap(),
				input_size: block.compressed_size().try_into().unwrap(),
				output_offset: (command.target_offset() + current_offset)
					.try_into()
					.unwrap(),
				output_size: block.decompressed_size().try_into().unwrap(),
			})
		})
	});

	// ASSUMPTION: FileOperation commands will apply their data in sequential order.

	// Do another skip pass, filtering out any remaining metadata (from AddFile
	// blocks) that fall entirely before the target offset.
	let metadata = block_iter
		.skip_while(|meta| {
			(meta.output_offset + meta.output_size) < usize::try_from(offset).unwrap()
		})
		.collect::<Vec<_>>();

	// Build the readers & complete
	let file_reader = BufReader::new(fs::File::open(&lookup.path)?);
	let block_stream = sqpack::BlockStream::new(file_reader, offset.try_into().unwrap(), metadata);

	Ok(Either::Right(block_stream))
}
