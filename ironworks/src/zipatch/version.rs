use std::{
	collections::HashMap,
	fs,
	io::{self, BufReader, Cursor, Seek, SeekFrom},
	sync::Arc,
};

use crate::{
	error::{Error, ErrorValue, Result},
	file::patch::FileOperation,
	sqpack,
	utility::{TakeSeekable, TakeSeekableExt},
};

use super::{
	lookup::{PatchLookup, SqPackSpecifier},
	repository::PatchRepository,
	temp_sqpack::read_block,
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

#[derive(Debug)]
pub struct Version {
	repositories: HashMap<u8, Arc<PatchRepository>>,
	cache: Arc<LookupCache>,
}

impl Version {
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
		// TODO: this needs to skip_while patches prior to the conf'd version
		let iterator = repository
			.patches
			.iter()
			.rev()
			.map(move |patch| self.cache.lookup(repository_id, repository, patch));

		Ok(iterator)
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
		todo!("version({repository})")
	}

	type Index = Cursor<Vec<u8>>;
	fn index(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index> {
		let target_specifier = SqPackSpecifier {
			repository,
			category,
			chunk,
			extension: 1,
		};

		let mut empty = true;
		let mut cursor = Cursor::new(Vec::<u8>::new());

		for maybe_lookup in self.lookups(repository)? {
			// Grab the commands for the requested target, if any exist in this patch.
			let lookup = maybe_lookup?;
			let commands = match lookup.index_commands.get(&target_specifier) {
				Some(commands) => commands,
				None => continue,
			};

			// Read the commands for this patch.
			// TODO: This construction of a file reader here is _very_ janky. Should be removed, and pulled from the cache in some way.
			let mut file = BufReader::new(fs::File::open(&lookup.path)?);
			for command in commands {
				empty = false;
				cursor.set_position(command.target_offset());
				let blocks = match command.operation() {
					FileOperation::AddFile(blocks) => blocks,
					_ => unreachable!(),
				};

				// TODO: this should be brought in from sqpack proper
				for block in blocks {
					let mut reader = read_block(&mut file, block)?;
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

	type Index2 = io::Empty;
	fn index2(&self, _repository: u8, _category: u8, _chunk: u8) -> Result<Self::Index2> {
		Err(Error::NotFound(ErrorValue::Other(
			"TODO: zipatch .index2 lookup".to_string(),
		)))
	}

	type File = TakeSeekable<BufReader<fs::File>>;
	fn file(&self, repository: u8, category: u8, location: sqpack::Location) -> Result<Self::File> {
		let target = (
			SqPackSpecifier {
				repository,
				category,
				chunk: location.chunk(),
				extension: location.data_file(),
			},
			location.offset(),
		);

		for maybe_lookup in self.lookups(repository)? {
			let lookup = maybe_lookup?;
			let command = match lookup.add_commands.get(&target) {
				Some(command) => command,
				None => continue,
			};

			let mut file = BufReader::new(fs::File::open(&lookup.path)?);
			file.seek(SeekFrom::Start(command.source_offset()))?;
			let out = file.take_seekable(command.data_size().into())?;

			return Ok(out);
		}

		Err(Error::NotFound(ErrorValue::Other(format!(
			"zipatch target {:?}",
			target.0
		))))
	}
}
