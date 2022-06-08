//! Structs and utilities for parsing .eqdp files.

use std::{borrow::Cow, fmt::Debug, io::Cursor};

use binrw::{binread, until_eof, BinRead};
use modular_bitfield::prelude::*;

use crate::error::Result;

use super::file::File;

#[binread]
#[br(little)]
pub struct EquipmentDeformerParameter {
	// todo penumbra merges these two as a "identifier"
	version: u8,
	#[br(temp)]
	_unk1: u8, // "skeleton"?
	block_size: u16,
	#[br(temp)]
	block_count: u16,

	#[br(count = block_count)]
	block_offsets: Vec<u16>,

	#[br(parse_with = until_eof)]
	data: Vec<u8>,
}

impl File for EquipmentDeformerParameter {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut Cursor::new(data.into()))?)
	}
}

impl EquipmentDeformerParameter {
	// u16?
	pub fn entry(&self, set_id: u16) -> Entry {
		let block_index = usize::try_from(set_id / self.block_size).unwrap();
		if block_index > self.block_offsets.len() {
			return Default::default();
		}

		// A block offset of u16::MAX designates the block to be "compressed" - emitted from the file, and equivalent to null bytes.
		let block_offset = self.block_offsets[block_index];
		if block_offset == u16::MAX {
			return Default::default();
		}

		let mut cursor = Cursor::new(&self.data);
		cursor.set_position((block_offset + set_id % self.block_size).into());
		// should this unwrap?
		Entry::read(&mut cursor).unwrap()
	}
}

impl Debug for EquipmentDeformerParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("EquipmentDeformerParameter")
			.field("version", &self.version)
			.field(
				"entry.len",
				&(usize::from(self.block_size / 2) * self.block_offsets.len()),
			)
			.finish()
	}
}

#[bitfield]
#[derive(BinRead, Debug, Default)]
pub struct Entry {
	head_ears: Field,
	body_neck: Field,
	hands_wrists: Field,
	legs_ring_r: Field,
	feet_ring_l: Field,
	#[skip]
	_reserved: B6,
}

// do i want this?
// impl Entry {
// 	fn head(&self) -> Field {
// 		self.head_ears()
// 	}

// 	fn ears(&self) -> Field {
// 		self.head_ears()
// 	}
// }

#[bitfield(bits = 2)]
#[derive(BitfieldSpecifier, Debug)]
pub struct Field {
	material: bool,
	model: bool,
}
