use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::utility::read_array;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for MJIAnimals {
    fn name() -> String {
        "MJIAnimals".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIAnimals::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIAnimals {
    pub r#b_npc_base: u32,
    pub r#size: u8,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#reward: Vec<u32>,
    pub r#icon: i32,
}
impl MJIAnimals {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#b_npc_base: row.field(0usize + offset)?.into_u32()?,
            r#size: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#reward: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(4usize + offset)?.into_u32()?) },
            )?,
            r#icon: row.field(6usize + offset)?.into_i32()?,
        })
    }
}
