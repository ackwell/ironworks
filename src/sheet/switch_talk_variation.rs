use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for SwitchTalkVariation {
    fn name() -> String {
        "SwitchTalkVariation".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SwitchTalkVariation::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SwitchTalkVariation {
    pub r#quest0: u32,
    pub r#quest1: u32,
    pub r#unknown2: u8,
    pub r#default_talk: u32,
}
impl SwitchTalkVariation {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest0: row.field(0usize + offset)?.into_u32()?,
            r#quest1: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#default_talk: row.field(3usize + offset)?.into_u32()?,
        })
    }
}
