//! Structs and utilities for parsing .imc files.

use std::{borrow::Cow, io::Cursor};

use binrw::{binread, BinRead};

use crate::error::Result;

use super::file::File;

// image change data? IMageChange?
#[binread]
#[br(little)]
#[derive(Debug)]
pub struct ImageChange {
	// Header.
	variant_count: u16,
	#[br(pad_after = 1)]
	part_mask: u8,
	#[br(calc = part_mask.count_ones() * u32::from(variant_count + 1))]
	entry_count: u32,
	// 0 is "default", 1+ is "variant".
	#[br(count = entry_count)]
	entries: Vec<ImageChangeEntry>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct ImageChangeEntry {
	material_id: u8,
	decal_id: u8,
	attribute_and_sound: u16,
	vfx_id: u8,
	material_animation_id_mask: u8,
}

impl File for ImageChange {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut Cursor::new(data.into()))?)
	}
}
