// TEMP
#![allow(missing_docs)]

use std::{
	collections::HashMap,
	fs,
	io::{self, BufReader, Cursor, Read, Seek, SeekFrom},
};

use flate2::read::DeflateDecoder;
use rust_lapper::{Interval, Lapper};

use crate::{
	error::{Error, ErrorValue, Result},
	file::{
		patch::{
			BlockHeader, Chunk, FileOperation, FileOperationCommand, SqPackChunk,
			ZiPatch as ZiPatchFile,
		},
		File,
	},
	sqpack,
};

#[derive(Debug)]
pub struct ZiPatch {}

impl ZiPatch {
	pub fn new() -> Self {
		Self {}
	}

	pub fn version(&self /* TODO: what's this api? */) -> ZiPatchVersion {
		ZiPatchVersion::new()
	}
}

#[derive(Debug)]
pub struct ZiPatchVersion {
	// TODO: the various caches should probably live on the main ZiPatch and be shared by all versions
	// TODO: doing the above will probably then best be fit by a single hash key for each patch file. cam be built on-request and such.
	patch_data: HashMap<String, PatchData>,
}

impl ZiPatchVersion {
	fn new() -> Self {
		// NOTE: so this is technically the "warm up" process - it can probably be done semi-lazily, but it forms a basis that needs to exist before lookups are performed
		let patch_data = PATCH_LIST
			.iter()
			// atm patch list is in oldest-first, and we want to get the newest copy of a file
			.rev()
			// hardcoding because lazy
			.map(|filename| format!("C:/Users/ackwell/code/xiv/patches/game/4e9a232b/{filename}"))
			// get the zipatch struct for each file
			.try_fold(HashMap::new(), |mut patch_data, filepath| -> Result<_> {
				eprintln!("checking {filepath}");

				let file = fs::File::open(&filepath).expect("TODO");
				let buf = BufReader::new(file);

				// operating on full patches at a time - this makes the (safe?) assumption that the granularity of a game _version_ is at _minimum_ one patch.
				let zipatch = ZiPatchFile::read(buf).expect("TODO");

				// we can assume that a non-1.6m byte file is EOF, but we _can't_ assume that 1.6m _isn't_ EOF (technically)

				let patch_chunks = collect_chunks(zipatch)?;

				// TODO: filepath correct way to do this? - probably not - should store just the patch name, and map that to fs path via indirection when hitting the fs itself
				patch_data.insert(filepath, patch_chunks);

				Ok(patch_data)
			})
			.expect("TODO");

		Self { patch_data }
	}
}

impl sqpack::Resource for ZiPatchVersion {
	fn path_metadata(&self, path: &str) -> Option<(u8, u8)> {
		let split = path.split('/').take(2).collect::<Vec<_>>();

		match split[..] {
			[path_category, _path_repository] => Some((
				// TODO: i'm hardcoding this to repo 0 (ffxiv) for now - this should be based on the patch repositories available
				0,
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
		todo!("SQPACK VERSION {repository}")
	}

	type Index = Cursor<Vec<u8>>;
	fn index(&self, repository: u8, category: u8, chunk: u8) -> Result<Self::Index> {
		// TODO: assumptions go brr
		let platform = "win32";
		let extension = "index";

		// TODO: this is partially copied from ffxiv::fs - how do i reuse this? the prefix is diff.
		let target = format!(
			"sqpack/ffxiv/{category:02x}{repository:02x}{chunk:02x}.{platform}.{extension}"
		);
		eprintln!("LOOKING FOR {target:?}");

		// todo: this should probably be reusing something on the main impl
		let patch_targets = PATCH_LIST
			.iter()
			.rev()
			// TODO: def. need a better approach for this obv.
			.map(|filename| format!("C:/Users/ackwell/code/xiv/patches/game/4e9a232b/{filename}"))
			.filter_map(|filepath| {
				// NOTE: using getkeyvalue so that the key string is a reference, which allows it to stay a single ref elsewhere
				let (filepath, patch_data) =
					self.patch_data.get_key_value(&filepath).expect("TODO");
				patch_data
					.index_commands
					.get(&target)
					.map(|commands| (filepath.as_str(), commands))
			});

		let mut final_commands = Vec::<(&str, &FileOperationCommand)>::new();
		for (filepath, commands) in patch_targets {
			final_commands = commands
				.iter()
				.map(|command| (filepath, command))
				.chain(final_commands.into_iter())
				.collect::<Vec<_>>();

			// ASSUMPTION: an off:0 write chunk will always be the first chunk within a given patch for that file: any writes before it would be zeroed by the off:0.
			if !final_commands.is_empty() && final_commands[0].1.target_offset() == 0 {
				break;
			}
		}

		// if there's no commands for the specified file we can assume it doesn't exist
		if final_commands.is_empty() {
			return Err(Error::NotFound(ErrorValue::Other(format!(
				"zipatch target {target}"
			))));
		}

		// try reading a cursor of the index
		// TODO: would be nice to get a total size for this?
		let mut cursor = Cursor::new(Vec::<u8>::new());
		for (filepath, command) in final_commands {
			// TODO: Probably shouldn't open a new file handle for every single command
			let file = fs::File::open(filepath).expect("TODO");
			let mut file = BufReader::new(file);

			cursor.set_position(command.target_offset());

			let blocks = match command.operation() {
				FileOperation::AddFile(blocks) => blocks,
				_ => unreachable!(),
			};

			for block in blocks {
				let mut reader = read_block(&mut file, block).expect("TODO");
				io::copy(&mut reader, &mut cursor).expect("TODO");
			}
		}

		cursor.set_position(0);

		Ok(cursor)
	}

	type Index2 = io::Empty;
	fn index2(&self, _repository: u8, _category: u8, _chunk: u8) -> Result<Self::Index2> {
		// TODO: lmao.
		Err(Error::NotFound(ErrorValue::Other(
			"TODO: zipatch .index2 lookup".to_string(),
		)))
	}

	type File = Cursor<Vec<u8>>;
	fn file(&self, repository: u8, category: u8, location: sqpack::Location) -> Result<Self::File> {
		// TODO: this is disgusting.
		let key = format!(
			"{}|{}|{}",
			category,
			(u16::from(repository) * 256) + u16::from(location.chunk()),
			location.data_file()
		);

		// todo using u32max; realistically it should fall back to file size - but does it matter?
		let interval_stop = match location.size() {
			Some(size) => size + location.offset(),
			None => u32::MAX,
		};

		let patch_targets = PATCH_LIST
			.iter()
			.rev()
			// TODO: def. need a better approach for this obv.
			.map(|filename| format!("C:/Users/ackwell/code/xiv/patches/game/4e9a232b/{filename}"))
			.filter_map(|filepath| {
				// NOTE: using getkeyvalue so that the key string is a reference, which allows it to stay a single ref elsewhere
				let (filepath, patch_data) =
					self.patch_data.get_key_value(&filepath).expect("TODO");

				let tree = patch_data.dat_trees.get(&key)?;

				// TODO: this is querying the entire range; even if we'll only need a subset - effectively meaning the tree lookup and the later RangeThing are double handling the querying; in a sense. if i can rework the interval tree to be intersect-able; might be simpler to do that

				let intervals = tree
					.find(location.offset(), interval_stop)
					.collect::<Vec<_>>();

				match intervals.is_empty() {
					false => Some((filepath, intervals)),
					true => None,
				}
			});

		// todo: this might work as a scan?
		// let thing = RangeThing::new(location.offset()..);
		let mut out = Cursor::new(Vec::<u8>::new());
		#[allow(clippy::never_loop)]
		for (filepath, intervals) in patch_targets {
			// ASSUMPTION: it's safe to act with intervals from a single patch file in a theoretically unordered fashion. IIF this assumption is broken; the intervals will likely need to be sorted by _chunk_ order (by adding an index to the interval value i assume) before being applied in reverse order.
			// FUCK IT TIER ASSUMPTION: square practically _only_ operates at a file level - they never seem to ship a partial file (maybe bar a file being broken over blocks in H2017). For now; I'm just going to check that the first patch we hit has exactly one interval, and if shit blows up later i'll revisit it. I'm so not in the mood for trying to work out how to do otherwise right now.
			// TODO: I can probably get rid of the entire interval tree if i do this.
			if intervals.len() > 1 {
				todo!("OOPSIE WOOPSIE!! Uwu We made a fucky wucky!! A wittle fucko boingo! The code monkeys at our headquarters are working VEWY HAWD to fix this!")
			}
			let interval = intervals[0];

			let start_inset = u64::from(location.offset() - interval.start);
			let size = u64::from(interval.stop - interval.start) - start_inset;

			let mut file = fs::File::open(filepath)?;
			file.seek(SeekFrom::Start(interval.val + start_inset))?;
			io::copy(&mut file.take(size), &mut out)?;
			break;
		}

		out.set_position(0);

		Ok(out)
	}
}

// TODO: this is copied from ffxiv::fs. i need to work out the ffxiv boundary for zipatch - and not copy this.
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
struct PatchData {
	index_commands: HashMap<String, Vec<FileOperationCommand>>,
	dat_trees: HashMap<String, Lapper<u32, u64>>,
}

// TODO: better name
fn collect_chunks(zipatch: ZiPatchFile) -> Result<PatchData> {
	// TODO: retry on failure?
	// ASSUMPTION: IndexUpdate chunks are unused, new indexes will always be distributed via FileOperation::AddFile.
	let (index_commands, intervals) = zipatch.chunks().try_fold(
		(HashMap::new(), HashMap::new()),
		|(mut index_commands, mut intervals), chunk| -> Result<_> {
			match chunk? {
				Chunk::SqPack(SqPackChunk::FileOperation(command))
					if is_index_command(&command) =>
				{
					index_commands
						.entry(command.path().to_string())
						.or_insert_with(Vec::new)
						.push(command);
				}

				Chunk::SqPack(SqPackChunk::Add(command)) => {
					// TODO: how do i want to handle this? realistically; some files will fall all the way back to the FileOps in H2017, which will have the disk filename rather than the sqpackfile struct. i'm tempted to say we should translate towards the struct rather than a string; but using string for brevity for now. If going with struct; index should be updated to align. That said; there's multiple dats per index, so can't share a top level hashmap.
					let file = command.file();
					let key = format!("{}|{}|{}", file.main_id(), file.sub_id(), file.file_id());

					intervals
						.entry(key)
						.or_insert_with(Vec::new)
						.push(Interval {
							start: command.target_offset(),
							stop: command.target_offset() + command.data_size(),
							val: command.source_offset(),
						});
				}

				_ => {}
			};

			Ok((index_commands, intervals))
		},
	)?;

	let dat_trees = intervals
		.into_iter()
		.map(|(key, intervals)| (key, Lapper::new(intervals)))
		.collect::<HashMap<_, _>>();

	Ok(PatchData {
		index_commands,
		dat_trees,
	})
}

fn is_index_command(command: &FileOperationCommand) -> bool {
	// TODO: do i want index2 as well?
	static TARGET_EXTENSION: &str = ".index";

	matches!(command.operation(), FileOperation::AddFile(_))
		&& command.path().to_string().ends_with(TARGET_EXTENSION)
}

// TODO: This is pretty much a copy paste from sqpack::file::shared - work out how this can be reused
fn read_block<'a, R: Read + Seek>(
	reader: &'a mut R,
	header: &BlockHeader,
) -> io::Result<BlockReader<'a, R>> {
	// Seek to the block and read its header so we know how much to expect in the rest of the block.
	// reader.seek(SeekFrom::Start(offset.into()))?;
	// let block_header =
	// 	BlockHeader::read(reader).map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;
	reader.seek(SeekFrom::Start(header.offset()))?;

	// TODO: Look into the padding on compressed blocks, there's some funky stuff going on in some cases. Ref. Coinach/IO/File & Lumina.

	// Build a reader for the block.
	let reader = match header.is_compressed() {
		true => BlockReader::Compressed(DeflateDecoder::new(
			reader.take(header.compressed_size().into()),
		)),
		false => BlockReader::Loose(reader.take(header.decompressed_size().into())),
	};

	Ok(reader)
}

enum BlockReader<'a, R> {
	Loose(io::Take<&'a mut R>),
	Compressed(DeflateDecoder<io::Take<&'a mut R>>),
}

impl<R: Read> Read for BlockReader<'_, R> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match self {
			Self::Loose(reader) => reader.read(buf),
			Self::Compressed(reader) => reader.read(buf),
		}
	}
}

// Temp listing because i'm being lazy and don't want to handle the ordering side of things right now
const PATCH_LIST: &[&str] = &[
	// "H2017.06.06.0000.0001a.patch",
	// "H2017.06.06.0000.0001b.patch",
	// "H2017.06.06.0000.0001c.patch",
	// "H2017.06.06.0000.0001d.patch",
	// "H2017.06.06.0000.0001e.patch",
	// "H2017.06.06.0000.0001f.patch",
	// "H2017.06.06.0000.0001g.patch",
	// "H2017.06.06.0000.0001h.patch",
	// "H2017.06.06.0000.0001i.patch",
	// "H2017.06.06.0000.0001j.patch",
	// "H2017.06.06.0000.0001k.patch",
	// "H2017.06.06.0000.0001l.patch",
	// "H2017.06.06.0000.0001m.patch",
	// "H2017.06.06.0000.0001n.patch",
	// "D2017.07.11.0000.0001.patch",
	// "D2017.09.24.0000.0001.patch",
	// "D2017.10.11.0000.0001.patch",
	// "D2017.10.31.0000.0001.patch",
	// "D2017.11.24.0000.0001.patch",
	// "D2018.01.12.0000.0001.patch",
	// "D2018.02.09.0000.0001.patch",
	// "D2018.04.27.0000.0001.patch",
	// "D2018.05.26.0000.0001.patch",
	// "D2018.06.19.0000.0001.patch",
	// "D2018.07.18.0000.0001.patch",
	// "D2018.09.05.0000.0001.patch",
	// "D2018.10.19.0000.0001.patch",
	// "D2018.12.15.0000.0001.patch",
	// "D2019.01.26.0000.0001.patch",
	// "D2019.03.12.0000.0001.patch",
	// "D2019.03.15.0000.0001.patch",
	// "D2019.04.16.0000.0001.patch",
	// "D2019.05.09.0000.0001.patch",
	// "D2019.05.29.0000.0000.patch",
	// "D2019.05.29.0001.0000.patch",
	// "D2019.05.31.0000.0001.patch",
	// "D2019.06.07.0000.0001.patch",
	// "D2019.07.09.0000.0001.patch",
	// "D2019.07.10.0001.0001.patch",
	// "D2019.10.11.0000.0001.patch",
	// "D2019.10.16.0000.0001.patch",
	// "D2019.11.02.0000.0001.patch",
	// "D2019.11.19.0000.0001.patch",
	// "D2019.12.19.0000.0001.patch",
	// "D2020.01.31.0000.0000.patch",
	// "D2020.01.31.0001.0000.patch",
	// "D2020.02.27.0000.0001.patch",
	// "D2020.03.24.0000.0001.patch",
	// "D2020.03.27.0000.0001.patch",
	// "D2020.07.21.0000.0000.patch",
	// "D2020.09.15.0000.0001.patch",
	// "D2020.10.06.0000.0001.patch",
	// "D2020.11.24.0000.0001.patch",
	// "D2020.12.15.0000.0001.patch",
	// "D2021.01.14.0000.0001.patch",
	// "D2021.02.10.0000.0001.patch",
	// "D2021.03.26.0000.0000.patch",
	// "D2021.03.26.0001.0000.patch",
	// "D2021.03.26.0002.0000.patch",
	// "D2021.03.26.0003.0000.patch",
	// "D2021.03.30.0000.0000.patch",
	// "D2021.04.29.0000.0001.patch",
	// "D2021.05.07.0000.0001.patch",
	// "D2021.06.16.0000.0001.patch",
	// "D2021.08.17.0000.0001.patch",
	// "D2021.11.17.0000.0000.patch",
	// "D2021.11.17.0001.0000.patch",
	// "D2021.11.17.0002.0000.patch",
	// "D2021.11.20.0000.0001.patch",
	// "D2021.11.27.0000.0001.patch",
	// "D2021.11.28.0000.0000.patch",
	// "D2021.12.15.0000.0001.patch",
	// "D2021.12.16.0000.0000.patch",
	// "D2021.12.23.0000.0000.patch",
	// "D2021.12.24.0000.0000.patch",
	"D2022.01.19.0000.0001.patch",
	"D2022.01.25.0000.0000.patch",
	"D2022.03.01.0000.0000.patch",
	"D2022.03.25.0000.0000.patch",
	"D2022.03.25.0001.0000.patch",
	"D2022.03.31.0000.0001.patch",
	"D2022.04.03.0000.0001.patch",
	"D2022.04.06.0000.0001.patch",
	"D2022.04.07.0000.0000.patch",
	"D2022.04.19.0000.0000.patch",
	"D2022.04.20.0000.0000.patch",
	"D2022.05.18.0000.0000.patch",
	"D2022.05.19.0000.0000.patch",
	"D2022.05.26.0000.0000.patch",
	"D2022.05.27.0000.0000.patch",
	"D2022.06.17.0000.0001.patch",
	"D2022.06.21.0000.0000.patch",
	"D2022.07.08.0000.0000.patch",
	"D2022.08.05.0000.0000.patch",
	"D2022.08.05.0001.0000.patch",
	"D2022.08.10.0000.0001.patch",
	"D2022.08.12.0000.0001.patch",
	"D2022.08.16.0000.0001.patch",
	"D2022.08.17.0000.0000.patch",
	"D2022.08.25.0000.0000.patch",
	"D2022.09.06.0000.0000.patch",
	"D2022.09.07.0000.0000.patch",
	"D2022.09.29.0000.0000.patch",
	"D2022.10.01.0000.0001.patch",
	"D2022.10.04.0000.0000.patch",
	"D2022.10.05.0000.0000.patch",
];
