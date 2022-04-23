use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for QuestRedo {
    fn name() -> String {
        "QuestRedo".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestRedo::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestRedo {
    pub r#final_quest: u32,
    pub r#unknown1: u32,
    pub r#chapter: u8,
    pub r#quest: Vec<u16>,
}
impl QuestRedo {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#final_quest: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
            r#chapter: row.field(2usize + offset)?.into_u8()?,
            r#quest: read_array(
                offset,
                32usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
