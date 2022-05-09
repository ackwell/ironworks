//! Structs and utilities for parsing .tex files.

use std::io::Cursor;

use binrw::{binread, until_eof, BinRead};
use derivative::Derivative;
use getset::{CopyGetters, Getters};

use crate::error::Result;

use super::file::File;

#[binread]
#[br(little)]
#[derive(Derivative, Getters, CopyGetters)]
#[derivative(Debug)]
pub struct Texture {
	// TODO: enums
	flags: u32, // attribute?
	#[get_copy = "pub"]
	format: u32,

	#[get_copy = "pub"]
	width: u16,
	#[get_copy = "pub"]
	height: u16,
	#[get_copy = "pub"]
	depth: u16,
	mip_levels: u16,

	lod_offsets: [u32; 3],
	surface_offset: [u32; 13],

	#[br(parse_with = until_eof)]
	#[derivative(Debug = "ignore")]
	// TODO: probably shouldn't expose this directly, there's a bunch of stuff around lod/mipmap to consider. check -caustics.tex.
	#[get = "pub"]
	data: Vec<u8>,
}

impl File for Texture {
	fn read(data: Vec<u8>) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut Cursor::new(data))?)
	}
}
