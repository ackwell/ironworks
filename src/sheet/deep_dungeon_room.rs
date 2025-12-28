use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for DeepDungeonRoom {
    fn name() -> String {
        "DeepDungeonRoom".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DeepDungeonRoom::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DeepDungeonRoom {
    pub r#level: Vec<u32>,
}
impl DeepDungeonRoom {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#level: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
