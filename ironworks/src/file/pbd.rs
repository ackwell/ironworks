//! Structs and utilities for parsing .pbd files.

use std::{
	borrow::Cow,
	collections::HashMap,
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
	// this is the character / race ID (cXXXX)
	id: u16,
	node_index: u16,

	#[br(temp)]
	data_offset: i32,

	#[br(
		if(data_offset > 0),
		seek_before = SeekFrom::Start(data_offset.try_into().unwrap()),
		restore_position,
	)]
	bone_matrices: Option<BoneMatrices>,

	// TODO: apparently 2.x pbds don't include this?
	unknown: f32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
struct BoneMatrices {
	#[br(temp, parse_with = current_position)]
	base_offset: u64,

	#[br(temp)]
	bone_count: u32,

	#[br(temp, args {
		count: bone_count.try_into().unwrap(),
		inner: (base_offset,)
	})]
	bone_names: Vec<BoneName>,

	#[br(temp, align_before = 4, count = bone_count)]
	matrices: Vec<[[f32; 4]; 3]>,

	#[br(calc = (0..bone_count)
		.map(|index| {
			let i = usize::try_from(index).unwrap();
			(bone_names[i].bone_name.to_string(), matrices[i])
		})
		.collect()
	)]
	bone_matrices: HashMap<String, [[f32; 4]; 3]>,
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

#[binread]
#[br(little)]
#[derive(Debug)]
struct Node {
	super_index: u16,
	first_child_index: u16,
	next_index: u16,
	header_index: u16,
}

fn current_position<R: Read + Seek>(reader: &mut R, _: &ReadOptions, _: ()) -> BinResult<u64> {
	Ok(reader.stream_position()?)
}
