use std::{
	collections::HashMap,
	fs,
	io::BufReader,
	path::{Path, PathBuf},
};

use crate::{
	error::{Error, ErrorValue, Result},
	file::{
		patch::{Chunk, FileOperation, FileOperationCommand, SqPackChunk, ZiPatch as ZiPatchFile},
		File,
	},
};

#[derive(Debug)]
pub struct PatchLookup {
	pub path: PathBuf,

	pub file_chunks: HashMap<SqPackSpecifier, Vec<FileChunk>>,
	// (specifier, offset)
	pub resource_chunks: HashMap<(SqPackSpecifier, u32), ResourceChunk>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SqPackSpecifier {
	pub repository: u8,
	pub category: u8,
	pub chunk: u8,
	pub extension: SqPackFileExtension,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SqPackFileExtension {
	Index(u8),
	Dat(u8),
}

#[derive(Debug)]
pub struct FileChunk {
	pub target_offset: u64,
	pub target_size: u64,
	pub blocks: Vec<FileBlock>,
}

#[derive(Debug)]
pub struct FileBlock {
	pub source_offset: u64,
	pub compressed_size: u32,
	pub decompressed_size: u32,
}

#[derive(Debug)]
pub struct ResourceChunk {
	pub offset: u64,
	pub size: u64,
}

impl PatchLookup {
	pub fn new(path: &Path) -> Result<Self> {
		read_lookup(path)
	}
}

fn read_lookup(path: &Path) -> Result<PatchLookup> {
	let file = BufReader::new(fs::File::open(path)?);
	let zipatch = ZiPatchFile::read(file)?;

	// TODO: Retry on failure?
	zipatch.chunks().try_fold(
		PatchLookup {
			path: path.to_owned(),
			file_chunks: Default::default(),
			resource_chunks: Default::default(),
		},
		|mut lookup, chunk| -> Result<_> {
			match chunk? {
				Chunk::SqPack(SqPackChunk::FileOperation(command)) => {
					process_file_operation(&mut lookup, command)?
				}

				Chunk::SqPack(SqPackChunk::Add(command)) => {
					let file = command.file();
					let specifier = SqPackSpecifier {
						repository: (file.sub_id() >> 8).try_into().unwrap(),
						category: file.main_id().try_into().unwrap(),
						chunk: (file.sub_id() & 0xFF).try_into().unwrap(),
						extension: SqPackFileExtension::Dat(file.file_id().try_into().unwrap()),
					};

					let chunk = ResourceChunk {
						offset: command.source_offset(),
						size: command.data_size().into(),
					};

					let old_value = lookup
						.resource_chunks
						.insert((specifier, command.target_offset()), chunk);

					if old_value.is_some() {
						panic!("Assumption broken! Multiple chunks in one patch file writing to same offset.")
					}
				}

				_ => {}
			};

			Ok(lookup)
		},
	)
}

fn process_file_operation(lookup: &mut PatchLookup, command: FileOperationCommand) -> Result<()> {
	let path = command.path().to_string();
	if !path.starts_with("sqpack/") {
		return Ok(());
	}

	let FileOperation::AddFile(blocks) = command.operation() else {
		return Ok(())
	};

	let chunk = FileChunk {
		target_offset: command.target_offset(),
		target_size: command.target_size(),
		blocks: blocks
			.iter()
			.map(|block| FileBlock {
				source_offset: block.offset(),
				compressed_size: block.compressed_size(),
				decompressed_size: block.decompressed_size(),
			})
			.collect(),
	};

	lookup
		.file_chunks
		.entry(path_to_specifier(&command.path().to_string())?)
		.or_insert_with(Vec::new)
		.push(chunk);

	Ok(())
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
		Some("index") => SqPackFileExtension::Index(1),
		Some("index2") => SqPackFileExtension::Index(2),
		Some(dat) if dat.starts_with("dat") => {
			let dat_number = dat[3..]
				.parse::<u8>()
				.map_err(|_err| path_error(&path, "unhandled file extension"))?;
			SqPackFileExtension::Dat(dat_number)
		}
		_ => return Err(path_error(&path, "unhandled file extension")),
	};

	Ok(SqPackSpecifier {
		repository,
		category,
		chunk,
		extension,
	})
}
