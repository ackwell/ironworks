use std::vec::Vec;
use crate::utility::read_array;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
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
    pub r#magicite_slot: Vec<u8>,
    pub r#name: SeString,
    pub r#content_finder_condition_start: u16,
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
            r#magicite_slot: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(18usize + offset)?.into_u8()?) },
            )?,
            r#name: row.field(22usize + offset)?.into_string()?,
            r#content_finder_condition_start: row.field(23usize + offset)?.into_u16()?,
        })
    }
}
