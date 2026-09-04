//! Structs and utilities for parsing .cutb files.

use std::io::Cursor;

use binrw::BinRead;
use getset::CopyGetters;

use crate::{
	FileStream,
	error::{Error, ErrorValue, Result},
};

use super::{File, layer::Instance, tmb::Timeline};

/// Bytes ahead of the node table.
const HEADER: u64 = 12;

/// Bytes one node table entry takes.
const ENTRY: u64 = 16;

/// Where a `CTDS` holds the point the cutscene plays around.
const SCENE_ORIGIN: u64 = 0x10;

/// Where a `CTDS` states how many entries it holds.
const SCENE_COUNT: u64 = 0x40;

/// Where a `CTDS` starts them.
const SCENE_ENTRIES: u64 = 0x54;

/// Where a `CTAL` reaches the records it holds, ahead of their count.
const PARTICIPANT_TABLE: u64 = 0x08;

/// Where a `CTEX` starts the table of runs it holds, and how many bytes one entry of it takes.
const TRACK_TABLE: u64 = 0x28;
const TRACK: u64 = 24;

/// Where a `CTEX` states how many runs it holds.
const TRACK_COUNT: u64 = 0x14;

fn invalid(reason: impl Into<String>) -> Error {
	Error::Invalid(ErrorValue::Other("cutscene".into()), reason.into())
}

/// The `size` bytes at `at`, taking the file's word for neither.
fn region(bytes: &[u8], at: u64, size: u64) -> Result<&[u8]> {
	let end = at
		.checked_add(size)
		.filter(|end| *end <= bytes.len() as u64)
		.ok_or_else(|| {
			invalid(format!(
				"{size} bytes at {at:#x} leave the {} byte file",
				bytes.len()
			))
		})?;
	Ok(&bytes[at as usize..end as usize])
}

fn u32_at(bytes: &[u8], at: u64) -> Result<u32> {
	region(bytes, at, 4).map(|raw| u32::from_le_bytes(raw.try_into().unwrap()))
}

fn f32_at(bytes: &[u8], at: u64) -> Result<f32> {
	region(bytes, at, 4).map(|raw| f32::from_le_bytes(raw.try_into().unwrap()))
}

/// Where the offset at `field` reaches, which the format writes from the field itself.
fn target(bytes: &[u8], field: u64) -> Result<u64> {
	let offset = u32_at(bytes, field)? as i32;
	field
		.checked_add_signed(offset.into())
		.filter(|at| *at < bytes.len() as u64)
		.ok_or_else(|| invalid(format!("offset {offset} from {field:#x} leaves the file")))
}

/// A null-terminated string at `at`.
fn string(bytes: &[u8], at: u64) -> Result<String> {
	let rest = usize::try_from(at)
		.ok()
		.and_then(|at| bytes.get(at..))
		.ok_or_else(|| {
			invalid(format!(
				"a string at {at:#x} of a {} byte file",
				bytes.len()
			))
		})?;
	let end = rest
		.iter()
		.position(|byte| *byte == 0)
		.unwrap_or(rest.len());
	Ok(String::from_utf8_lossy(&rest[..end]).into_owned())
}

/// A cutscene: the files it loads, the level it plays in, and a timeline per shot.
#[derive(Debug)]
pub struct Cutscene {
	nodes: Vec<Node>,
}

impl Cutscene {
	/// Every node, in the order the file lists them.
	pub fn nodes(&self) -> &[Node] {
		&self.nodes
	}
}

impl File for Cutscene {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		let mut bytes = Vec::new();
		stream.read_to_end(&mut bytes)?;

		if bytes.get(..4) != Some(b"CUTB".as_slice()) {
			return Err(invalid("a file that does not open with CUTB"));
		}

		let count = u64::from(u32_at(&bytes, 8)?);
		region(&bytes, HEADER, count * ENTRY)?;

		let nodes = (0..count)
			.map(|index| node(&bytes, HEADER + index * ENTRY))
			.collect::<Result<_>>()?;
		Ok(Self { nodes })
	}
}

/// One node of a cutscene.
#[derive(Debug)]
pub enum Node {
	/// `CTRL`: the files the cutscene loads.
	Resources(Vec<Resource>),

	/// `CTIS`: the sheet the cutscene reads its dialogue from.
	Sheet(String),

	/// `CTDS`: the level the cutscene plays in, and what it drives there.
	Scene(Scene),

	/// `CTAL`: the participants the cutscene drives, as the layer instances they are written as.
	/// Every one the game ships is an [`InstanceKind::HelperObject`](super::layer::InstanceKind).
	Participants(Vec<Instance>),

	/// `CTPA`: groups of twelve-byte records. Their fields are not modelled.
	Groups(Vec<Group>),

	/// `CTEX`: runs of per-frame scalars.
	Tracks(Vec<Track>),

	/// `CTTL`: one timeline, in the format `.tmb` files hold.
	Timeline(Timeline),

	/// A node whose magic this crate does not model.
	Unknown(Unknown),
}

/// One file a cutscene loads.
#[derive(Debug, CopyGetters)]
pub struct Resource {
	path: String,

	/// Zero or 255 in every file the game ships.
	#[get_copy = "pub"]
	unknown_1: u32,
}

impl Resource {
	/// Path of the file.
	pub fn path(&self) -> &str {
		&self.path
	}
}

/// `CTDS`: the level a cutscene plays in, and what it drives there.
#[derive(Debug, CopyGetters)]
pub struct Scene {
	level: String,

	#[get_copy = "pub"]
	origin: [f32; 3],

	entries: Vec<[u32; 2]>,
}

impl Scene {
	/// Where the level's own files sit, under `bg/`.
	pub fn level(&self) -> &str {
		&self.level
	}

	/// The pairs of ids the cutscene drives, which this crate does not name.
	pub fn entries(&self) -> &[[u32; 2]] {
		&self.entries
	}
}

/// `CTPA`: one group of records.
#[derive(Debug, CopyGetters)]
pub struct Group {
	/// What the group holds records for, which is usually but not reliably its index.
	#[get_copy = "pub"]
	id: u32,

	records: Vec<[u8; 12]>,
}

impl Group {
	/// The records, uninterpreted.
	pub fn records(&self) -> &[[u8; 12]] {
		&self.records
	}
}

/// One run of per-frame scalars a `CTEX` holds, which a cutscene plays over something the file does
/// not name.
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct Track {
	/// Which of a pair of runs this is: one or two.
	lane: u32,

	/// What the pair of runs belongs to. Nothing else in the file carries the same ids.
	id: u32,

	unknown_1: u32,

	/// Rises across the node, holding across a pair of lanes.
	unknown_2: u32,

	#[getset(skip)]
	values: Vec<f32>,
}

impl Track {
	/// The values the run holds, one a frame.
	pub fn values(&self) -> &[f32] {
		&self.values
	}
}

/// A node whose magic this crate does not model.
///
/// `CTCB` is the one the game ships, along with the `CTEX` whose table of runs does not read. The
/// node table counts a `CTCB`'s records rather than its bytes, so that one's extent comes from its
/// own header.
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct Unknown {
	magic: [u8; 4],

	#[getset(skip)]
	body: Vec<u8>,
}

impl Unknown {
	/// Everything the node holds.
	pub fn body(&self) -> &[u8] {
		&self.body
	}
}

fn node(bytes: &[u8], entry: u64) -> Result<Node> {
	let magic: [u8; 4] = region(bytes, entry, 4)?.try_into().unwrap();
	let field = entry + 8;
	let at = field + u64::from(u32_at(bytes, field)?);
	let declared = u64::from(u32_at(bytes, field + 4)?);

	// `CTCB` is the one node the table counts records for rather than bytes.
	let size = match &magic {
		b"CTCB" => u64::from(u32_at(bytes, at)?),
		_ => declared,
	};
	let body = region(bytes, at, size)?;

	Ok(match &magic {
		b"CTRL" => Node::Resources(resources(bytes, at)?),
		b"CTIS" => Node::Sheet(string(bytes, at + u64::from(u32_at(bytes, at)?))?),
		b"CTDS" => Node::Scene(scene(bytes, at)?),
		b"CTAL" => Node::Participants(participants(body)?),
		b"CTPA" => Node::Groups(groups(bytes, at)?),
		b"CTEX" => match tracks(bytes, at, size) {
			Some(held) => Node::Tracks(held),
			None => Node::Unknown(Unknown {
				magic,
				body: body.to_vec(),
			}),
		},
		// Read over the node alone, so an offset inside the timeline cannot reach another node.
		b"CTTL" => Node::Timeline(<Timeline as BinRead>::read(&mut Cursor::new(body))?),
		_ => Node::Unknown(Unknown {
			magic,
			body: body.to_vec(),
		}),
	})
}

/// Reads the runs a `CTEX` holds, where its table reads: the entries name where each run starts,
/// they rise, the first lands where the table ends and the last runs to the node's own end.
///
/// A node laying its table out another way reads as no runs at all rather than as the wrong ones.
fn tracks(bytes: &[u8], at: u64, size: u64) -> Option<Vec<Track>> {
	let count = u64::from(u32_at(bytes, at + TRACK_COUNT).ok()?);
	let body = count.checked_mul(TRACK)?.checked_add(TRACK_TABLE)?;
	if count == 0 || body > size {
		return None;
	}
	let table = at + TRACK_TABLE;
	let starts = (0..count)
		.map(|index| Some(u64::from(u32_at(bytes, table + index * TRACK + 4).ok()?)))
		.collect::<Option<Vec<_>>>()?;
	let tiles = starts[0] == count * TRACK
		&& starts.windows(2).all(|held| held[0] < held[1])
		&& starts.iter().all(|held| held % 4 == 0)
		&& *starts.last()? < size - TRACK_TABLE;
	if !tiles {
		return None;
	}

	(0..count)
		.map(|index| {
			let record = table + index * TRACK;
			let start = table + starts[index as usize];
			let end = match starts.get(index as usize + 1) {
				Some(held) => table + held,
				None => at + size,
			};
			Some(Track {
				lane: u32_at(bytes, record).ok()?,
				id: u32_at(bytes, record + 8).ok()?,
				unknown_1: u32_at(bytes, record + 12).ok()?,
				unknown_2: u32_at(bytes, record + 16).ok()?,
				values: region(bytes, start, end - start)
					.ok()?
					.chunks_exact(4)
					.map(|held| f32::from_le_bytes(held.try_into().unwrap()))
					.collect(),
			})
		})
		.collect()
}

fn resources(bytes: &[u8], at: u64) -> Result<Vec<Resource>> {
	let table = at + u64::from(u32_at(bytes, at)?);
	let count = u64::from(u32_at(bytes, at + 4)?);
	region(bytes, table, count * 8)?;

	(0..count)
		.map(|index| {
			let field = table + index * 8;
			Ok(Resource {
				path: string(bytes, target(bytes, field)?)?,
				unknown_1: u32_at(bytes, field + 4)?,
			})
		})
		.collect()
}

fn scene(bytes: &[u8], at: u64) -> Result<Scene> {
	let level = string(bytes, at + u64::from(u32_at(bytes, at)?))?;
	let count = u64::from(u32_at(bytes, at + SCENE_COUNT)?);
	let table = at + SCENE_ENTRIES;
	region(bytes, table, count * 8)?;

	let entries = (0..count)
		.map(|index| {
			let record = table + index * 8;
			Ok([u32_at(bytes, record)?, u32_at(bytes, record + 4)?])
		})
		.collect::<Result<_>>()?;
	Ok(Scene {
		level,
		origin: floats(bytes, at + SCENE_ORIGIN)?,
		entries,
	})
}

/// The three floats at `at`.
fn floats(bytes: &[u8], at: u64) -> Result<[f32; 3]> {
	Ok([
		f32_at(bytes, at)?,
		f32_at(bytes, at + 4)?,
		f32_at(bytes, at + 8)?,
	])
}

/// Reads the participants a `CTAL` names, each running to where the next starts.
///
/// The last one's extent is not stated, so it carries the strings the node ends with. Records are
/// read over the node alone, so an offset inside one cannot reach another node.
fn participants(node: &[u8]) -> Result<Vec<Instance>> {
	let table = u64::from(u32_at(node, PARTICIPANT_TABLE)?);
	let count = u64::from(u32_at(node, PARTICIPANT_TABLE + 4)?);
	region(node, table, count * 4)?;

	let starts = (0..count)
		.map(|index| Ok((table + u64::from(u32_at(node, table + index * 4)?)) as usize))
		.collect::<Result<Vec<_>>>()?;

	starts
		.iter()
		.enumerate()
		.map(|(index, start)| {
			let end = starts.get(index + 1).copied().unwrap_or(node.len());
			Instance::parse(node, *start, end)
		})
		.collect()
}

fn groups(bytes: &[u8], at: u64) -> Result<Vec<Group>> {
	let table = at + u64::from(u32_at(bytes, at)?);
	let count = u64::from(u32_at(bytes, at + 4)?);
	region(bytes, table, count * 12)?;

	(0..count)
		.map(|index| {
			let entry = table + index * 12;
			let start = table + u64::from(u32_at(bytes, entry + 4)?);
			let records = u64::from(u32_at(bytes, entry + 8)?);
			Ok(Group {
				id: u32_at(bytes, entry)?,
				records: region(bytes, start, records * 12)?
					.chunks_exact(12)
					.map(|record| record.try_into().unwrap())
					.collect(),
			})
		})
		.collect()
}

#[cfg(test)]
mod test {
	use std::io::{self, Cursor};

	use crate::{
		error::Error,
		file::{
			File,
			layer::{HelperKind, InstanceData, InstanceKind},
		},
	};

	use super::{Cutscene, Node};

	/// Lays the node bodies out contiguously behind the table, each entry naming its own.
	fn cutscene(nodes: &[(&[u8; 4], Vec<u8>, Option<u32>)]) -> Vec<u8> {
		let mut bytes = Vec::from(*b"CUTB");
		bytes.extend([0; 4]);
		bytes.extend(u32::try_from(nodes.len()).unwrap().to_le_bytes());
		bytes.resize(12 + 16 * nodes.len(), 0);

		for (index, (magic, body, declared)) in nodes.iter().enumerate() {
			let entry = 12 + 16 * index;
			let field = entry + 8;
			let start = bytes.len();
			bytes.extend(body);

			bytes[entry..entry + 4].copy_from_slice(*magic);
			bytes[entry + 4..field].copy_from_slice(&16u32.to_le_bytes());
			bytes[field..field + 4]
				.copy_from_slice(&u32::try_from(start - field).unwrap().to_le_bytes());
			let size = declared.unwrap_or_else(|| u32::try_from(body.len()).unwrap());
			bytes[field + 4..field + 8].copy_from_slice(&size.to_le_bytes());
		}

		let length = u32::try_from(bytes.len()).unwrap();
		bytes[4..8].copy_from_slice(&length.to_le_bytes());
		bytes
	}

	fn control(paths: &[&str]) -> Vec<u8> {
		let mut bytes = Vec::from(24u32.to_le_bytes());
		bytes.extend(u32::try_from(paths.len()).unwrap().to_le_bytes());
		bytes.extend([0; 16]);

		let heap = 24 + paths.len() * 8;
		let mut strings = Vec::new();
		for (index, path) in paths.iter().enumerate() {
			let field = 24 + index * 8;
			let offset =
				i32::try_from(heap + strings.len()).unwrap() - i32::try_from(field).unwrap();
			bytes.extend(offset.to_le_bytes());
			bytes.extend(255u32.to_le_bytes());
			strings.extend(path.as_bytes());
			strings.push(0);
		}

		bytes.extend(strings);
		bytes
	}

	fn info(sheet: &str) -> Vec<u8> {
		let mut bytes = Vec::from(8u32.to_le_bytes());
		bytes.extend(1u32.to_le_bytes());
		bytes.extend(sheet.as_bytes());
		bytes.push(0);
		bytes
	}

	fn scene(level: &str, origin: [f32; 3], entries: &[[u32; 2]], extra: &[u32]) -> Vec<u8> {
		let heap = 0x54 + entries.len() * 8 + extra.len() * 4;
		let mut bytes = vec![0; 0x54];
		bytes[0x00..0x04].copy_from_slice(&u32::try_from(heap).unwrap().to_le_bytes());
		for (index, axis) in origin.iter().enumerate() {
			let at = 0x10 + index * 4;
			bytes[at..at + 4].copy_from_slice(&axis.to_le_bytes());
		}
		bytes[0x40..0x44].copy_from_slice(&u32::try_from(entries.len()).unwrap().to_le_bytes());
		let named = 0x54 + entries.len() * 8;
		bytes[0x48..0x4C].copy_from_slice(&u32::try_from(named).unwrap().to_le_bytes());
		bytes[0x4C..0x50].copy_from_slice(&u32::try_from(extra.len()).unwrap().to_le_bytes());

		bytes.extend(entries.iter().flatten().flat_map(|id| id.to_le_bytes()));
		bytes.extend(extra.iter().flat_map(|value| value.to_le_bytes()));
		bytes.extend(level.as_bytes());
		bytes.push(0);
		bytes
	}

	/// One helper record: what it stands for, where it stands, and the base row it names. `asset`
	/// nests a background part behind the record and puts a placement behind that.
	fn helper(id: u32, kind: u32, position: [f32; 3], base_id: u32, asset: Option<&str>) -> Vec<u8> {
		let mut record = vec![0; 0x8C];
		record[0x00..0x04].copy_from_slice(&15u32.to_le_bytes());
		record[0x04..0x08].copy_from_slice(&id.to_le_bytes());
		for (index, axis) in position.iter().enumerate() {
			let at = 0x0C + index * 4;
			record[at..at + 4].copy_from_slice(&axis.to_le_bytes());
		}
		record[0x34..0x38].copy_from_slice(&kind.to_le_bytes());
		record[0x39] = 3;
		record[0x40..0x44].copy_from_slice(&base_id.to_le_bytes());

		let Some(asset) = asset else {
			return record;
		};

		let mut nested = vec![0; 0x5C];
		nested[0x00..0x04].copy_from_slice(&1u32.to_le_bytes());
		nested[0x30..0x34].copy_from_slice(&0x5Cu32.to_le_bytes());
		nested.extend(asset.as_bytes());
		nested.push(0);

		record[0x54..0x58].copy_from_slice(&0x8Cu32.to_le_bytes());
		let placement = 0x8C + nested.len();
		record[0x88..0x8C].copy_from_slice(&u32::try_from(placement).unwrap().to_le_bytes());
		record.extend(nested);

		let mut block = vec![0; 0x38];
		block[0x00..0x04].copy_from_slice(&7.0f32.to_le_bytes());
		block[0x24..0x28].copy_from_slice(&1u32.to_le_bytes());
		record.extend(block);
		record
	}

	fn participants(records: &[Vec<u8>]) -> Vec<u8> {
		// The first pair the header holds is empty in every file the game ships, and the records
		// come through the second.
		let mut bytes = Vec::from(16u32.to_le_bytes());
		bytes.extend([0; 4]);
		bytes.extend(16u32.to_le_bytes());
		bytes.extend(u32::try_from(records.len()).unwrap().to_le_bytes());

		let mut at = records.len() * 4;
		for record in records {
			bytes.extend(u32::try_from(at).unwrap().to_le_bytes());
			at += record.len();
		}

		bytes.extend(records.concat());
		// The strings the node ends with, which no record's extent covers.
		bytes.push(0);
		bytes
	}

	fn groups(counts: &[(u32, usize)]) -> Vec<u8> {
		let mut bytes = Vec::from(8u32.to_le_bytes());
		bytes.extend(u32::try_from(counts.len()).unwrap().to_le_bytes());

		let mut at = counts.len() * 12;
		let mut records = Vec::new();
		for (id, count) in counts {
			bytes.extend(id.to_le_bytes());
			bytes.extend(u32::try_from(at).unwrap().to_le_bytes());
			bytes.extend(u32::try_from(*count).unwrap().to_le_bytes());
			records.extend(vec![u8::try_from(*id).unwrap(); count * 12]);
			at += count * 12;
		}

		bytes.extend(records);
		bytes
	}

	/// A body whose declared record count says nothing about how long it is.
	fn ctcb(records: usize, slack: usize) -> Vec<u8> {
		let size = 12 + records * 24 + slack;
		let mut bytes = Vec::from(u32::try_from(size).unwrap().to_le_bytes());
		bytes.resize(size, 0xCC);
		bytes
	}

	fn timeline(items: &[(&[u8; 4], Vec<u8>)]) -> Vec<u8> {
		let length = 12 + items.iter().map(|(_, body)| 8 + body.len()).sum::<usize>();
		let mut bytes = Vec::from(*b"TMLB");
		bytes.extend(u32::try_from(length).unwrap().to_le_bytes());
		bytes.extend(u32::try_from(items.len()).unwrap().to_le_bytes());

		for (magic, body) in items {
			bytes.extend(**magic);
			bytes.extend(u32::try_from(8 + body.len()).unwrap().to_le_bytes());
			bytes.extend(body);
		}

		bytes
	}

	fn header(duration: i16) -> Vec<u8> {
		[1i16, 0, duration, 3]
			.iter()
			.flat_map(|field| field.to_le_bytes())
			.collect()
	}

	#[test]
	fn empty() {
		assert!(matches!(
			Cutscene::read(io::empty()),
			Err(Error::Invalid(..))
		));
	}

	#[test]
	fn reads_every_modelled_node() {
		let file = Cutscene::read(Cursor::new(cutscene(&[
			(b"CTRL", control(&["chara/one.mdl", "chara/two.tex"]), None),
			(b"CTIS", info("quest/023/BanAll110_02382"), None),
			(
				b"CTDS",
				scene(
					"ex1/02_dra_d2/fld/d2f3/level/d2f3",
					[1.0, 2.0, 3.0],
					&[[7, 8]],
					&[],
				),
				None,
			),
			(
				b"CTAL",
				participants(&[
					helper(0xff00_0001, 4, [4.0, 5.0, 6.0], 1_003_223, None),
					helper(0xff00_0002, 8, [0.0; 3], 0, Some("bg/one.mdl")),
				]),
				None,
			),
			(b"CTPA", groups(&[(0, 2), (1, 0)]), None),
			(b"CTTL", timeline(&[(b"TMDH", header(480))]), None),
		])))
		.unwrap();

		let nodes = file.nodes();
		assert_eq!(nodes.len(), 6);

		let Node::Resources(resources) = &nodes[0] else {
			panic!("expected resources, got {:?}", nodes[0]);
		};
		assert_eq!(resources[0].path(), "chara/one.mdl");
		assert_eq!(resources[1].path(), "chara/two.tex");
		assert_eq!(resources[1].unknown_1(), 255);

		assert!(matches!(&nodes[1], Node::Sheet(sheet) if sheet == "quest/023/BanAll110_02382"));

		let Node::Scene(scene) = &nodes[2] else {
			panic!("expected a scene, got {:?}", nodes[2]);
		};
		assert_eq!(scene.level(), "ex1/02_dra_d2/fld/d2f3/level/d2f3");
		assert_eq!(scene.origin(), [1.0, 2.0, 3.0]);
		assert_eq!(scene.entries(), [[7, 8]]);

		let Node::Participants(participants) = &nodes[3] else {
			panic!("expected participants, got {:?}", nodes[3]);
		};
		assert_eq!(participants[0].kind(), InstanceKind::HelperObject);
		assert_eq!(participants[0].id(), 0xff00_0001);
		assert_eq!(participants[0].transform().translation(), [4.0, 5.0, 6.0]);

		let InstanceData::HelperObject(helper) = participants[0].data() else {
			panic!("expected a helper, got {:?}", participants[0].data());
		};
		assert_eq!(helper.kind(), HelperKind::EventNpc);
		assert_eq!(helper.base_id(), 1_003_223);
		assert_eq!(helper.height(), 3);
		assert!(helper.nested().is_none());
		assert!(helper.placement().is_none());

		let InstanceData::HelperObject(helper) = participants[1].data() else {
			panic!("expected a helper, got {:?}", participants[1].data());
		};
		assert_eq!(helper.kind(), HelperKind::BgPart);
		let Some(nested) = helper.nested() else {
			panic!("expected a nested instance");
		};
		assert_eq!(nested.kind(), InstanceKind::BgPart);
		let InstanceData::BgPart(part) = nested.data() else {
			panic!("expected a background part, got {:?}", nested.data());
		};
		assert_eq!(part.asset_path(), "bg/one.mdl");
		let Some(placement) = helper.placement() else {
			panic!("expected a placement");
		};
		assert_eq!(placement.transform().translation(), [7.0, 0.0, 0.0]);
		assert_eq!(placement.flags(), 1);

		let Node::Groups(groups) = &nodes[4] else {
			panic!("expected groups, got {:?}", nodes[4]);
		};
		assert_eq!(groups[0].id(), 0);
		assert_eq!(groups[0].records(), [[0; 12]; 2]);
		assert!(groups[1].records().is_empty());

		let Node::Timeline(timeline) = &nodes[5] else {
			panic!("expected a timeline, got {:?}", nodes[5]);
		};
		assert_eq!(timeline.items().len(), 1);
	}

	/// The strings sit past the entries the offset at `0x48` names, not at it.
	#[test]
	fn a_scene_reaches_its_level_over_the_second_array() {
		let file = Cutscene::read(Cursor::new(cutscene(&[(
			b"CTDS",
			scene(
				"ffxiv/wil_w1/twn/w1t2/level/w1t2",
				[0.0; 3],
				&[[1, 2]],
				&[0xFE80_0000],
			),
			None,
		)])))
		.unwrap();

		let Node::Scene(scene) = &file.nodes()[0] else {
			panic!("expected a scene, got {:?}", file.nodes()[0]);
		};
		assert_eq!(scene.level(), "ffxiv/wil_w1/twn/w1t2/level/w1t2");
	}

	/// The node table counts a `CTCB`'s records, so its size is nowhere near its extent.
	#[test]
	fn a_ctcb_is_bounded_by_its_own_header() {
		let file =
			Cutscene::read(Cursor::new(cutscene(&[(b"CTCB", ctcb(3, 52), Some(3))]))).unwrap();

		let Node::Unknown(node) = &file.nodes()[0] else {
			panic!("expected an unknown node, got {:?}", file.nodes()[0]);
		};
		assert_eq!(node.body().len(), 12 + 3 * 24 + 52);
	}

	#[test]
	fn an_unknown_magic_keeps_its_bytes() {
		let file = Cutscene::read(Cursor::new(cutscene(&[(b"CTEX", vec![9; 24], None)]))).unwrap();

		let Node::Unknown(node) = &file.nodes()[0] else {
			panic!("expected an unknown node, got {:?}", file.nodes()[0]);
		};
		assert_eq!(node.magic(), *b"CTEX");
		assert_eq!(node.body(), [9; 24]);
	}

	/// A timeline is read over its own node, so an offset landing in a later one is out of bounds
	/// rather than a string from somewhere else entirely.
	#[test]
	fn a_timeline_cannot_reach_past_its_node() {
		// `TMPP` reads a path at the offset it holds, written from the item's body. Twelve bytes on
		// lands exactly at the next node's own string.
		let escaping = timeline(&[(b"TMPP", Vec::from(12i32.to_le_bytes()))]);
		let bytes = cutscene(&[
			(b"CTTL", escaping, None),
			(b"CTIS", info("chara/elsewhere"), None),
		]);

		assert!(matches!(
			Cutscene::read(Cursor::new(bytes)),
			Err(Error::Resource(_))
		));
	}

	#[test]
	fn a_node_declaring_more_than_the_file_holds() {
		let mut bytes = cutscene(&[(b"CTIS", info("quest/023/BanAll110_02382"), None)]);
		bytes.truncate(bytes.len() - 4);
		assert!(matches!(
			Cutscene::read(Cursor::new(bytes)),
			Err(Error::Invalid(..))
		));
	}
}
