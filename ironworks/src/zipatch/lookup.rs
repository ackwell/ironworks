use std::{
	collections::HashMap,
	fs,
	io::BufReader,
	path::{Path, PathBuf},
};

use crate::{
	error::{Error, ErrorValue, Result},
	file::{
		patch::{
			AddCommand, Chunk, FileOperation, FileOperationCommand, SqPackChunk,
			ZiPatch as ZiPatchFile,
		},
		File,
	},
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SqPackFileExtension {
	Index(u8),
	Dat(u8),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SqPackSpecifier {
	pub repository: u8,
	pub category: u8,
	pub chunk: u8,
	pub extension: SqPackFileExtension,
}

#[derive(Debug)]
pub struct PatchLookup {
	// TODO: if i move the reader generation to consumer-provided in some manner; this can probably be a ref or w/e to that. As-is, this is horrid.
	pub path: PathBuf,

	// TODO: consider storing a slightly more ergonomic struct instead of commands
	pub add_operations: HashMap<SqPackSpecifier, Vec<FileOperationCommand>>,
	// (specifier, offset)
	pub add_commands: HashMap<(SqPackSpecifier, u32), AddCommand>,
}

impl PatchLookup {
	// TODO: clean this up a bit
	pub fn new(path: &Path) -> Result<Self> {
		read_lookup(path)
	}
}

fn read_lookup(path: &Path) -> Result<PatchLookup> {
	// TODO: this should be log:: or something
	println!("reading patch: {path:?}");

	let file = BufReader::new(fs::File::open(path)?);
	let zipatch = ZiPatchFile::read(file)?;

	// TODO: Retry on failure?
	zipatch.chunks().try_fold(
		PatchLookup {
			path: path.to_owned(),
			add_operations: Default::default(),
			add_commands: Default::default(),
		},
		|mut lookup, chunk| -> Result<_> {
			match chunk? {
				Chunk::SqPack(SqPackChunk::FileOperation(command))
					if is_sqpack_command(&command) =>
				{
					lookup
						.add_operations
						.entry(path_to_specifier(&command.path().to_string())?)
						.or_insert_with(Vec::new)
						.push(command)
				}

				Chunk::SqPack(SqPackChunk::Add(command)) => {
					let file = command.file();
					let specifier = SqPackSpecifier {
						repository: (file.sub_id() >> 8).try_into().unwrap(),
						category: file.main_id().try_into().unwrap(),
						chunk: (file.sub_id() & 0xFF).try_into().unwrap(),
						extension: SqPackFileExtension::Dat(file.file_id().try_into().unwrap()),
					};

					// ASSUMPTION: Square seemingly never breaks new files up across multiple
					// chunks - an entire file can be read by looking for the single add
					// command starting at the precise offset we're looking for.

					let old_value = lookup
						.add_commands
						.insert((specifier, command.target_offset()), command);

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

fn is_sqpack_command(command: &FileOperationCommand) -> bool {
	matches!(command.operation(), FileOperation::AddFile(_))
		&& command.path().to_string().starts_with("sqpack/")
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
