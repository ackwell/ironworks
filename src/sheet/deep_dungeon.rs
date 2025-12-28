use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for DeepDungeon {
    fn name() -> String {
        "DeepDungeon".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DeepDungeon::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DeepDungeon {
    pub r#aetherpool_arm: u8,
    pub r#aetherpool_armor: u8,
    pub r#pomander_slot: Vec<u8>,
    pub r#deep_dungeon_type: u8,
    pub r#stone_slot: Vec<u8>,
    pub r#name: u8,
    pub r#content_finder_condition_start: u8,
    pub r#unknown25: u32,
    pub r#unknown26: u32,
    pub r#unknown27: u8,
    pub r#unknown28: u8,
    pub r#unknown29: u32,
    pub r#unknown30: u32,
    pub r#unknown31: u8,
    pub r#unknown32: u8,
    pub r#unknown33: u32,
    pub r#unknown34: u32,
    pub r#unknown35: SeString,
    pub r#unknown36: u32,
}
impl DeepDungeon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#aetherpool_arm: row.field(0usize + offset)?.into_u8()?,
            r#aetherpool_armor: row.field(1usize + offset)?.into_u8()?,
            r#pomander_slot: read_array(
                offset,
                16usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_u8()?) },
            )?,
            r#deep_dungeon_type: row.field(18usize + offset)?.into_u8()?,
            r#stone_slot: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(19usize + offset)?.into_u8()?) },
            )?,
            r#name: row.field(23usize + offset)?.into_u8()?,
            r#content_finder_condition_start: row.field(24usize + offset)?.into_u8()?,
            r#unknown25: row.field(25usize + offset)?.into_u32()?,
            r#unknown26: row.field(26usize + offset)?.into_u32()?,
            r#unknown27: row.field(27usize + offset)?.into_u8()?,
            r#unknown28: row.field(28usize + offset)?.into_u8()?,
            r#unknown29: row.field(29usize + offset)?.into_u32()?,
            r#unknown30: row.field(30usize + offset)?.into_u32()?,
            r#unknown31: row.field(31usize + offset)?.into_u8()?,
            r#unknown32: row.field(32usize + offset)?.into_u8()?,
            r#unknown33: row.field(33usize + offset)?.into_u32()?,
            r#unknown34: row.field(34usize + offset)?.into_u32()?,
            r#unknown35: row.field(35usize + offset)?.into_string()?,
            r#unknown36: row.field(36usize + offset)?.into_u32()?,
        })
    }
}
