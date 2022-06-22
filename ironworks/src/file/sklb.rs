//! Structs and utilities for parsing .sklb files.

use std::{
	borrow::Cow,
	io::{Cursor, Read, Seek, SeekFrom},
};

use binrw::{binread, count, until_eof, BinRead, BinResult, ReadOptions};
use getset::{CopyGetters, Getters};

use crate::error::Result;

use super::file::File;

/// Skeleton data and related mappings.
#[binread]
#[br(little, magic = b"blks")]
#[derive(Debug, Getters, CopyGetters)]
pub struct SkeletonBinary {
	// File header.
	/// Version of this skelton file. This is a XIV-specific version tag, and does
	/// not directly correlate with the version of the embedded tagfile.
	#[get_copy = "pub"]
	version: Version,

	#[br(args(version))]
	header: Header,

	// Animation Layers.
	#[br(
		seek_before = SeekFrom::Start(header.layer_offset().into()),
		temp,
		assert(&alph_magic == b"hpla")
	)]
	alph_magic: [u8; 4],

	#[br(temp)]
	layer_count: u16,

	///
	#[br(args {
		count: layer_count.into(),
		inner: (header.layer_offset().into(),)
	})]
	#[get = "pub"]
	animation_layers: Vec<AnimationLayer>,

	/// Skeleton data, in Havok binary tagfile format.
	#[br(
		seek_before = SeekFrom::Start(header.skeleton_offset().into()),
		parse_with = until_eof,
	)]
	#[get = "pub"]
	skeleton: Vec<u8>,
}

impl SkeletonBinary {
	/// ID of the character associated with this skeleton.
	pub fn character_id(&self) -> u32 {
		match &self.header {
			Header::V1(header) => header.character_id,
			Header::V2(header) => header.character_id,
		}
	}

	///
	pub fn mapper_character_id(&self) -> [u32; 4] {
		match &self.header {
			Header::V1(header) => header.mapper_character_id,
			Header::V2(header) => header.mapper_character_id,
		}
	}

	///
	pub fn connect_bones(&self) -> Vec<i16> {
		match &self.header {
			Header::V1(header) => header.connect_bones.to_vec(),
			Header::V2(header) => vec![header.connect_bone_index],
		}
	}

	///
	pub fn lod_sample_bone_count(&self) -> Option<[i16; 3]> {
		match &self.header {
			Header::V1(header) => Some(header.lod_sample_bone_count),
			Header::V2(_) => None,
		}
	}
}

impl File for SkeletonBinary {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut Cursor::new(data.into()))?)
	}
}

/// XIV skeleton file version.
#[allow(missing_docs)]
#[binread]
#[br(little)]
#[derive(Clone, Copy, Debug)]
pub enum Version {
	#[br(magic = b"0011")]
	V1100,

	#[br(magic = b"0111")]
	V1110,

	#[br(magic = b"0021")]
	V1200,

	#[br(magic = b"0031")]
	V1300,
}

#[derive(Debug)]
enum Header {
	V1(HeaderV1),
	V2(HeaderV2),
}

impl Header {
	fn layer_offset(&self) -> u32 {
		match self {
			Self::V1(header) => header.layer_offset.into(),
			Self::V2(header) => header.layer_offset,
		}
	}

	fn skeleton_offset(&self) -> u32 {
		match self {
			Self::V1(header) => header.skeleton_offset.into(),
			Self::V2(header) => header.skeleton_offset,
		}
	}
}

impl BinRead for Header {
	type Args = (Version,);

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		_options: &ReadOptions,
		(version,): Self::Args,
	) -> BinResult<Self> {
		match version {
			Version::V1100 | Version::V1110 | Version::V1200 => {
				Ok(Self::V1(HeaderV1::read(reader)?))
			}
			Version::V1300 => Ok(Self::V2(HeaderV2::read(reader)?)),
		}
	}
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct HeaderV1 {
	layer_offset: u16,
	skeleton_offset: u16,
	character_id: u32,
	mapper_character_id: [u32; 4],
	lod_sample_bone_count: [i16; 3],
	connect_bones: [i16; 4],
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct HeaderV2 {
	layer_offset: u32,
	skeleton_offset: u32,
	connect_bone_index: i16,
	#[br(pad_before = 2)]
	character_id: u32,
	mapper_character_id: [u32; 4],
}

///
#[derive(Debug, Getters, CopyGetters)]
pub struct AnimationLayer {
	///
	#[get_copy = "pub"]
	layer: u32,

	///
	#[get = "pub"]
	bone_indices: Vec<i16>,
}

impl BinRead for AnimationLayer {
	type Args = (u64,);

	fn read_options<R: Read + Seek>(
		reader: &mut R,
		options: &ReadOptions,
		(base_offset,): Self::Args,
	) -> BinResult<Self> {
		let offset = u16::read(reader)?;
		let position = reader.stream_position()?;

		reader.seek(SeekFrom::Start(base_offset + u64::from(offset)))?;

		let layer = u32::read(reader)?;
		let bone_count = u16::read(reader)?;
		let bone_indices = count(bone_count.into())(reader, options, ())?;

		let result = Self {
			layer,
			bone_indices,
		};

		reader.seek(SeekFrom::Start(position))?;

		Ok(result)
	}
}
