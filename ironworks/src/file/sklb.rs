//! Structs and utilities for parsing .sklb files.

use std::{
	borrow::Cow,
	io::{Cursor, Read, Seek, SeekFrom},
};

use binrw::{binread, count, until_eof, BinRead, BinResult, ReadOptions};

use crate::error::Result;

use super::file::File;

#[binread]
#[br(little, magic = b"blks")]
#[derive(Debug)]
pub struct SkeletonBinary {
	// File header.
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

	#[br(args {
		count: layer_count.into(),
		inner: (header.layer_offset().into(),)
	})]
	animation_layers: Vec<AnimationLayer>,

	// TODO: read this properly?
	#[br(
		seek_before = SeekFrom::Start(header.skeleton_offset().into()),
		parse_with = until_eof,
	)]
	skeleton: Vec<u8>,
}

impl File for SkeletonBinary {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut Cursor::new(data.into()))?)
	}
}

#[binread]
#[br(little)]
#[derive(Clone, Copy, Debug)]
enum Version {
	#[br(magic = b"0011")]
	V1100,

	#[br(magic = b"0111")]
	V1110,

	#[br(magic = b"0021")]
	V1200,

	#[br(magic = b"0031")]
	V1300,
}

// #[binread]
#[derive(Debug)]
enum Header {
	V1(HeaderV1),
	V2(HeaderV2),
}

// TODO: These might make more sense on the main struct, as they'll probably need to be exposed there for public interface anyway
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

#[derive(Debug)]
struct AnimationLayer {
	layer: u32,
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
