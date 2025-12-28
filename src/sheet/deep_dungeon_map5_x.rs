use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for DeepDungeonMap5X {
    fn name() -> String {
        "DeepDungeonMap5X".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DeepDungeonMap5X::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DeepDungeonMap5X {
    pub r#deep_dungeon_room: Vec<u16>,
}
impl DeepDungeonMap5X {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#deep_dungeon_room: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
