//! Structs and utilities for parsing .tex files.

use std::{borrow::Cow, io::Cursor};

use binrw::{binread, until_eof, BinRead};
use derivative::Derivative;
use getset::{CopyGetters, Getters};
use modular_bitfield::BitfieldSpecifier;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::error::Result;

use super::file::File;

/// A texture and associated metadata.
#[binread]
#[br(little)]
#[derive(Derivative, Getters, CopyGetters)]
#[derivative(Debug)]
pub struct Texture {
	attributes: bitfield::Attributes,
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
	mip_levels: u16,

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
	/// Dimension kind.
	pub fn dimension(&self) -> Dimension {
		self.attributes.dimension()
	}
}

impl File for Texture {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut Cursor::new(data.into()))?)
	}
}

// Isolating bitfield in a module so modular_bitfield lint disables don't pollute the entire file.
#[allow(dead_code, clippy::identity_op)]
mod bitfield {
	use binrw::binread;
	use modular_bitfield::prelude::*;

	use super::Dimension;

	#[bitfield]
	#[binread]
	#[derive(Debug)]
	#[br(map = Self::from_bytes)]
	pub struct Attributes {
		discard_per_frame: bool,
		discard_per_map: bool,
		managed: bool,
		user_managed: bool,
		cpu_read: bool,
		location_main: bool,
		no_gpu_read: bool,
		aligned_size: bool,
		edge_culling: bool,
		location_onion: bool,
		read_write: bool,
		immutable: bool,
		// 0x1000,
		// 0x2000,
		// 0x4000,
		// 0x8000,
		// 0x10000,
		// 0x20000,
		// 0x40000,
		// 0x80000,
		#[skip]
		unknown1: B8,
		texture_render_target: bool,
		texture_depth_stencil: bool,
		pub dimension: Dimension,
		texture_swizzle: bool,
		texture_no_tiled: bool,
		// 0x10000000
		// 0x20000000
		// 0x40000000
		#[skip]
		unknown2: B3,
		texture_no_swizzle: bool,
	}
}

/// The dimension kind of a texture.
#[allow(missing_docs)]
#[derive(BitfieldSpecifier, Debug)]
#[bits = 4]
pub enum Dimension {
	D1 = 1,
	D2 = 2,
	D3 = 4,
	Cube = 8,
}

/// Pixel format of a texture.
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
