//! Structs and utilities for parsing .tex files.

use binrw::{binread, until_eof, BinRead};
use derivative::Derivative;
use getset::{CopyGetters, Getters};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{error::Result, FileStream};

use super::file::File;

/// A texture and associated metadata.
#[binread]
#[br(little)]
#[derive(Derivative, Getters, CopyGetters)]
#[derivative(Debug)]
pub struct Texture {
	attributes: u32,

	/// Pixel data format.
	#[get_copy = "pub"]
	format: Format,

	/// Width in pixels.
	#[get_copy = "pub"]
	width: u16,

	/// Height in pixels.
	#[get_copy = "pub"]
	height: u16,

	/// Depth. Unknown interpretation.
	#[get_copy = "pub"]
	depth: u16,

	/// Mipmap level count.
	#[get_copy = "pub"]
	mip_levels: u8,

	/// Texture array size. Only used by D2Array texture kinds.
	#[get_copy = "pub"]
	array_size: u8,

	// TODO: work out how these should be exposed.
	lod_surfaces: [u32; 3],
	surface_offsets: [u32; 13],

	/// Byte array of pixel data.
	#[br(parse_with = until_eof)]
	#[derivative(Debug = "ignore")]
	#[get = "pub"]
	data: Vec<u8>,
}

impl Texture {
	/// Kind of texture represented by this file.
	pub fn kind(&self) -> TextureKind {
		match (self.attributes & TextureKind::MASK) >> TextureKind::SHIFT {
			0b0000001 => TextureKind::D1,
			0b0000010 => TextureKind::D2,
			0b0000100 => TextureKind::D3,
			0b0001000 => TextureKind::Cube,
			0b1000000 => TextureKind::D2Array,
			_ => TextureKind::Unknown,
		}
	}
}

impl File for Texture {
	fn read(mut stream: impl FileStream) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut stream)?)
	}
}

/// The kind of a texture, or resource. This value implies the semantics of the
/// rest of the texture metadata.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum TextureKind {
	Unknown,
	D1,
	D2,
	D3,
	Cube,
	D2Array,
}

impl TextureKind {
	const SHIFT: u32 = 22;
	const MASK: u32 = 0x13C00000;
}

/// Pixel format of a texture.
#[allow(missing_docs)]
#[binread]
#[derive(Copy, Clone, Debug, PartialEq, Eq, IntoPrimitive)]
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
	/// Texture format kind.
	pub fn kind(&self) -> FormatKind {
		FormatKind::try_from(u32::from(*self) & 0xF000).unwrap()
	}

	/// Channel or component count.
	pub fn components(&self) -> u8 {
		u8::try_from((u32::from(*self) & 0x0F00) >> 8).unwrap()
	}

	/// Bits per pixel.
	pub fn bits_per_pixel(&self) -> u8 {
		1 << ((u32::from(*self) & 0x00F0) >> 4)
	}
}

/// The overarching kind of a texture format.
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
