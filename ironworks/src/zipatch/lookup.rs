use std::{
	fs,
	hash::Hash,
	io,
	num::ParseIntError,
	path::{Path, PathBuf},
};

use binrw::{BinRead, BinWrite, binrw};

use crate::{
	file::patch::{Chunk, FileOperation, FileOperationCommand, SqPackChunk},
	sqpack,
};

use super::{
	chunks::ChunkIterator,
	utility::{BrwMap, BrwVec},
};

#[derive(Debug)]
pub struct PatchLookup {
	pub path: PathBuf,
	pub data: PatchLookupData,
}

impl PatchLookup {
	pub fn build(path: &Path) -> sqpack::Result<Self> {
		read_lookup(path)
	}

	pub fn from_cache(path: &Path, cache: &Path) -> sqpack::Result<Self> {
		// Try to read data from an existing cache.
		let data = match fs::File::open(cache) {
			// File exists. Try to read, but bail if it's an old version or corrupt.
			Ok(mut file) => match VersionedPatchLookupData::read(&mut file) {
				Ok(VersionedPatchLookupData::V2(data)) => Some(data),
				_other => None,
			},

			// No cache yet.
			Err(error) if error.kind() == io::ErrorKind::NotFound => None,

			// FS error, fail out.
			Err(error) => Err(error)?,
		};

		match data {
			Some(data) => Ok(Self {
				path: path.to_owned(),
				data,
			}),

			None => {
				let lookup = Self::build(path)?;

				let mut file = fs::File::create(cache)?;
				let versioned_data = VersionedPatchLookupData::V2(lookup.data);
				versioned_data.write(&mut file)?;

				let VersionedPatchLookupData::V2(data) = versioned_data else {
					unreachable!()
				};
				Ok(Self {
					path: path.to_owned(),
					data,
				})
			}
		}
	}
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
enum VersionedPatchLookupData {
	#[brw(magic = b"1")]
	V1,

	#[brw(magic = b"2")]
	V2(PatchLookupData),
}

#[binrw]
#[brw(little)]
#[derive(Debug, Default)]
pub struct PatchLookupData {
	pub file_chunks: BrwMap<SqPackSpecifier, BrwVec<FileChunk>>,
	pub resource_chunks: BrwMap<(SqPackSpecifier, u64), ResourceChunk>,
}

#[binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SqPackSpecifier {
	pub repository: u8,
	pub category: u8,
	pub chunk: u8,
	pub extension: SqPackFileExtension,
}

#[binrw]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SqPackFileExtension {
	#[brw(magic = b"I")]
	Index(u8),

	#[brw(magic = b"D")]
	Dat(u8),
}

#[binrw]
#[derive(Debug, Clone)]
pub struct FileChunk {
	pub target_offset: u64,
	pub target_size: u64,
	pub blocks: BrwVec<FileBlock>,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct FileBlock {
	pub source_offset: u64,
	pub compressed_size: u32,
	pub decompressed_size: u32,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ResourceChunk {
	pub offset: u64,
	pub size: u64,
}

fn read_lookup(path: &Path) -> sqpack::Result<PatchLookup> {
	let file = io::BufReader::new(fs::File::open(path)?);
	let mut chunks = ChunkIterator::new(file);

	// TODO: Retry on failure?
	let data = chunks.try_fold(
		PatchLookupData::default(),
		|mut data, chunk| -> sqpack::Result<_> {
			match chunk? {
				Chunk::SqPack(SqPackChunk::FileOperation(command)) => {
					process_file_operation(&mut data, command)?
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

					let old_value = data
						.resource_chunks
						.insert((specifier, command.target_offset()), chunk);

					if old_value.is_some() {
						panic!(
							"Assumption broken! Multiple chunks in one patch file writing to same offset."
						)
					}
				}

				_ => {}
			};

			Ok(data)
		},
	)?;

	Ok(PatchLookup {
		path: path.to_owned(),
		data,
	})
}

fn process_file_operation(
	data: &mut PatchLookupData,
	command: FileOperationCommand,
) -> sqpack::Result<()> {
	let path = command.path().to_string();
	if !path.starts_with("sqpack/") {
		return Ok(());
	}

	let FileOperation::AddFile(blocks) = command.operation() else {
		return Ok(());
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

	data.file_chunks
		.entry(path_to_specifier(&command.path().to_string())?)
		.or_insert_with(Default::default)
		.push(chunk);

	Ok(())
}

fn path_to_specifier(path: &str) -> sqpack::Result<SqPackSpecifier> {
	let path = PathBuf::from(path);

	let file_name = path
		.file_stem()
		.and_then(|osstr| osstr.to_str())
		.ok_or_else(|| path_error(&path, "invalid unicode", None))?;

	let category = u8::from_str_radix(&file_name[0..2], 16)
		.map_err(|err| path_error(&path, "invalid category", err))?;
	let repository = u8::from_str_radix(&file_name[2..4], 16)
		.map_err(|err| path_error(&path, "invalid repository", err))?;
	let chunk = u8::from_str_radix(&file_name[4..6], 16)
		.map_err(|err| path_error(&path, "invalid chunk", err))?;

	let extension = match path.extension().and_then(|osstr| osstr.to_str()) {
		Some("index") => SqPackFileExtension::Index(1),
		Some("index2") => SqPackFileExtension::Index(2),
		Some(dat) if dat.starts_with("dat") => {
			let dat_number = dat[3..]
				.parse::<u8>()
				.map_err(|_err| path_error(&path, "unhandled file extension", None))?;
			SqPackFileExtension::Dat(dat_number)
		}
		_ => return Err(path_error(&path, "unhandled file extension", None)),
	};

	Ok(SqPackSpecifier {
		repository,
		category,
		chunk,
		extension,
	})
}

#[derive(Debug, thiserror::Error)]
#[error("zipatch command path {path} is malformed: {reason}")]
struct MalformedPathError {
	path: PathBuf,
	reason: &'static str,

	#[source]
	source: Option<ParseIntError>,
}

fn path_error(
	path: &Path,
	reason: &'static str,
	source: impl Into<Option<ParseIntError>>,
) -> sqpack::Error {
	let error = MalformedPathError {
		path: path.to_owned(),
		reason,
		source: source.into(),
	};
	sqpack::Error::Malformed(error.into())
}
