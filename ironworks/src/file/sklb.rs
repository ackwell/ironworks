//! Structs and utilities for parsing .sklb files.

use std::{borrow::Cow, io::Cursor};

use binrw::{binread, BinRead};

use crate::error::Result;

use super::file::File;

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct SkeletonBinary {}

impl File for SkeletonBinary {
	fn read<'a>(data: impl Into<Cow<'a, [u8]>>) -> Result<Self> {
		Ok(<Self as BinRead>::read(&mut Cursor::new(data.into()))?)
	}
}
