//! Structs and utilities for parsing .eqdp files.

use std::{fmt::Debug, io::Cursor};

use binrw::{binread, until_eof, BinRead};

use crate::{error::Result, FileStream};

use super::file::File;

/// Metadata for equipment and accessory sets on a per-race basis.
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
	fn read(mut stream: impl FileStream) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut stream)?)
	}
}

impl EquipmentDeformerParameter {
	/// Get metadata for for specified set ID.
	pub fn set(&self, id: u16) -> Set {
		// Sets are laid out sequentially - grab the index of the block it should reside in.
		let block_index = usize::try_from(id / self.block_size).unwrap();
		if block_index >= self.block_offsets.len() {
			return Default::default();
		}

		// A block offset of u16::MAX designates the block to be "compressed" - omitted from the file, and equivalent to null bytes.
		let block_offset = self.block_offsets[block_index];
		if block_offset == u16::MAX {
			return Default::default();
		}

		// Read the entry from the data block.
		let mut cursor = Cursor::new(&self.data);
		cursor.set_position(((block_offset + id % self.block_size) * 2).into());
		Set(bitfield::Set::read(&mut cursor).unwrap())
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

/// Metadata for a specific set.
#[derive(Debug, Default)]
pub struct Set(bitfield::Set);

macro_rules! set_slot {
	($name:ident, $field:ident) => {
		#[doc = concat!("Metadata for the ", stringify!($name), " slot.")]
		pub fn $name(&self) -> Slot {
			Slot(self.0.$field())
		}
	};
}

impl Set {
	set_slot!(head, head_ears);
	set_slot!(body, body_neck);
	set_slot!(hands, hands_wrists);
	set_slot!(legs, legs_ring_r);
	set_slot!(feet, feet_ring_l);

	set_slot!(ears, head_ears);
	set_slot!(neck, body_neck);
	set_slot!(wrists, hands_wrists);
	set_slot!(ring_right, legs_ring_r);
	set_slot!(ring_left, feet_ring_l);
}

/// Metadata for a slot within a set.
#[derive(Debug)]
pub struct Slot(bitfield::Slot);

impl Slot {
	/// Whether there is a dedicated material for this (Race, Set, Slot).
	pub fn material(&self) -> bool {
		self.0.material()
	}

	/// Whether there is a dedicated model for this (Race, Set, Slot).
	pub fn model(&self) -> bool {
		self.0.model()
	}
}

#[allow(dead_code, clippy::identity_op, clippy::unnecessary_cast)]
mod bitfield {
	use binrw::BinRead;
	use modular_bitfield::prelude::*;

	#[bitfield]
	#[derive(BinRead, Debug, Default)]
	#[br(map = Self::from_bytes)]
	pub struct Set {
		pub head_ears: Slot,
		pub body_neck: Slot,
		pub hands_wrists: Slot,
		pub legs_ring_r: Slot,
		pub feet_ring_l: Slot,
		#[skip]
		_reserved: B6,
	}

	#[bitfield(bits = 2)]
	#[derive(BitfieldSpecifier, Debug)]
	pub struct Slot {
		pub material: bool,
		pub model: bool,
	}
}
