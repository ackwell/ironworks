// TODO: if zipatch doesn't grow much this might make more sense over there.

use std::{
	collections::HashMap,
	fs,
	io::BufReader,
	path::{Path, PathBuf},
	sync::Arc,
};

use crate::{
	error::{Error, ErrorValue, Result},
	file::{
		patch::{Chunk, FileOperation, FileOperationCommand, SqPackChunk, ZiPatch as ZiPatchFile},
		File,
	},
	utility::{HashMapCache, HashMapCacheExt},
};

#[derive(Debug)]
pub struct PatchCache {
	repositories: HashMap<u8, (PathBuf, Vec<String>)>,
	cache: HashMapCache<(u8, String), PatchMetadata>,
}

impl PatchCache {
	pub fn new(repositories: HashMap<u8, (PathBuf, Vec<String>)>) -> Self {
		Self {
			repositories,
			cache: Default::default(),
		}
	}

	pub fn todonameme(
		&self,
		repository_id: u8,
	) -> Result<impl Iterator<Item = Result<Arc<PatchMetadata>>> + '_> {
		let (base_dir, patches) = self.repositories.get(&repository_id).ok_or_else(|| {
			Error::NotFound(ErrorValue::Other(format!("repository {repository_id}")))
		})?;

		// We're operating at a patch-by-patch granularity here, with the (very safe)
		// assumption that a game version is at minimum one patch.
		let iterator = patches.iter().rev().map(move |patch| {
			// TODO: this will lock the cache for the entire time it's building the cache for a patch - consider if that should be resolved.
			self.cache
				.try_get_or_insert((repository_id, patch.clone()), || {
					read_metadata(&base_dir.join(format!("{patch}.patch")))
				})
		});
		Ok(iterator)
	}
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct SqPackSpecifier {
	repository: u8,
	category: u8,
	chunk: u8,
	extension: u8,
}

#[derive(Debug, Default)]
pub struct PatchMetadata {
	// TODO: consider storing a slightly more ergonomic struct instead of commands
	index_commands: HashMap<SqPackSpecifier, Vec<FileOperationCommand>>,
}

fn read_metadata(path: &Path) -> Result<PatchMetadata> {
	// TODO: this should be log:: or something
	println!("reading patch: {path:?}");

	let file = BufReader::new(fs::File::open(path)?);
	let zipatch = ZiPatchFile::read(file)?;

	// TODO: Retry on failure?
	zipatch.chunks().try_fold(
		PatchMetadata::default(),
		|mut metadata, chunk| -> Result<_> {
			match chunk? {
				Chunk::SqPack(SqPackChunk::FileOperation(command))
					if is_index_command(&command) =>
				{
					metadata
						.index_commands
						.entry(path_to_specifier(&command.path().to_string())?)
						.or_insert_with(Vec::new)
						.push(command)
				}
				_ => {}
			};
			Ok(metadata)
		},
	)
}

fn is_index_command(command: &FileOperationCommand) -> bool {
	// TODO: do i want index2 as well?
	static TARGET_EXTENSION: &str = ".index";

	matches!(command.operation(), FileOperation::AddFile(_))
		&& command.path().to_string().ends_with(TARGET_EXTENSION)
}

fn path_to_specifier(path: &str) -> Result<SqPackSpecifier> {
	let path = PathBuf::from(path);

	fn path_error(path: &Path, reason: &str) -> Error {
		Error::Invalid(
			ErrorValue::Other(format!("patch path {path:?}")),
			reason.into(),
		)
	}

	let file_name = path
		.file_stem()
		.and_then(|osstr| osstr.to_str())
		.ok_or_else(|| path_error(&path, "malformed file name"))?;

	let category = u8::from_str_radix(&file_name[0..2], 16)
		.map_err(|err| path_error(&path, &format!("{err}")))?;
	let repository = u8::from_str_radix(&file_name[2..4], 16)
		.map_err(|err| path_error(&path, &format!("{err}")))?;
	let chunk = u8::from_str_radix(&file_name[4..6], 16)
		.map_err(|err| path_error(&path, &format!("{err}")))?;

	let extension = match path.extension().and_then(|osstr| osstr.to_str()) {
		Some("index") => 1,
		Some(dat) if dat.starts_with("dat") => dat[3..]
			.parse::<u8>()
			.map_err(|_err| path_error(&path, "unhandled file extension"))?,
		_ => return Err(path_error(&path, "unhandled file extension")),
	};

	Ok(SqPackSpecifier {
		repository,
		category,
		chunk,
		extension,
	})
}
