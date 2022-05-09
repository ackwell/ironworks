//! Structs and utilities for parsing .tex files.

use std::io::Cursor;

use binrw::{binread, until_eof, BinRead};
use derivative::Derivative;
use getset::{CopyGetters, Getters};
use num_enum::{IntoPrimitive, TryFromPrimitive};

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
	format: Format,

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

/// TODO: docs
#[allow(missing_docs)]
#[binread]
#[derive(Copy, Clone, Debug, IntoPrimitive)]
#[br(repr = u32)]
#[repr(u32)]
pub enum Format {
	Unknown = 0x0,

	L8 = 0x1130,
	A8 = 0x1131,

	Rgba4 = 0x1440,
	Rgb5a1 = 0x1441,
	Argb8 = 0x1450,
	Rgbx8 = 0x1451,
	Argb82 = 0x1452,

	R32F = 0x2150,
	Rg16F = 0x2250,
	Rgba16F = 0x2460,
	Rgba32F = 0x2470,

	Dxt1 = 0x3420,
	Dxt3 = 0x3430,
	Dxt5 = 0x3431,

	D16 = 0x4140,
	D24S8 = 0x4250,
	Rgba8 = 0x4401, // Zero BPP?

	Null = 0x5100,
	Shadow16 = 0x5140,
	Shadow24 = 0x5150,
}

impl Format {
	pub fn kind(&self) -> FormatKind {
		FormatKind::try_from(u32::from(*self) & 0xF000).unwrap()
	}

	pub fn components(&self) -> u8 {
		u8::try_from((u32::from(*self) & 0x0F00) >> 8).unwrap()
	}

	pub fn bits_per_pixel(&self) -> u8 {
		1 << ((u32::from(*self) & 0x00F0) >> 4)
	}
}

/// TODO: docs
#[allow(missing_docs)]
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum FormatKind {
	Integer = 0x1000,
	Float = 0x2000,
	Dxt = 0x3000,
	DepthStencil = 0x4000,
	Special = 0x5000,
}
