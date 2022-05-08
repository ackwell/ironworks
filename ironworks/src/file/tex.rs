//! Structs and utilities for parsing .tex files.

use std::io::Cursor;

use binrw::{binread, until_eof, BinRead};
use derivative::Derivative;

use crate::error::Result;

use super::file::File;

#[binread]
#[br(little)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Texture {
	// TODO: enums
	flags: u32, // attribute?
	format: u32,

	width: u16,
	height: u16,
	depth: u16,
	mip_levels: u16,

	lod_offsets: [u32; 3],
	surface_offset: [u32; 13],

	#[br(parse_with = until_eof)]
	#[derivative(Debug = "ignore")]
	data: Vec<u8>,
}

impl File for Texture {
	fn read(data: Vec<u8>) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut Cursor::new(data))?)
	}
}
