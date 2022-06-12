//! Structs and utilities for parsing .pbd files.

use std::{
	borrow::Cow,
	fmt::Debug,
	io::{Cursor, Read, Seek, SeekFrom},
};

use binrw::{binread, BinRead, BinResult, NullString, ReadOptions};

use crate::error::Result;

use super::file::File;

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct PreBoneDeformer {
	#[br(temp)]
	data_count: u32,

	#[br(count = data_count)]
	deformers: Vec<Deformer>,

	#[br(count = data_count)]
	nodes: Vec<Node>,
}

impl File for PreBoneDeformer {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut Cursor::new(data.into()))?)
	}
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Deformer {
	id: u16,
	node_index: u16,

	#[br(temp)]
	data_offset: i32,

	#[br(
    if(data_offset > 0),
    seek_before = SeekFrom::Start(data_offset.try_into().unwrap()),
    restore_position,
  )]
	data: Option<DeformerData>,

	// TODO: apparently 2.x pbds don't include this?
	unk: f32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct Node {
	super_index: u16,
	sub_top_index: u16,
	next_index: u16,
	header_index: u16,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct DeformerData {
	#[br(temp, parse_with = current_position)]
	deformer_offset: u64,

	#[br(temp)]
	bone_count: u32,

	#[br(args {
    count: bone_count.try_into().unwrap(),
    inner: (deformer_offset,)
  })]
	bone_names: Vec<BoneName>,

	// NOTE: this is 1:1 with the bone names above - should probably expose as tuples or a struct
	#[br(align_before = 4, count = bone_count)]
	matrices: Vec<[[f32; 4]; 3]>,
}

#[binread]
#[br(little, import(base_offset: u64))]
#[derive(Debug)]
struct BoneName {
	#[br(temp)]
	offset: i16,

	#[br(
    seek_before = SeekFrom::Start(base_offset + u64::try_from(offset).unwrap()),
    restore_position,
  )]
	bone_name: NullString,
}

fn current_position<R: Read + Seek>(reader: &mut R, _: &ReadOptions, _: ()) -> BinResult<u64> {
	Ok(reader.stream_position()?)
}
