use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for QuestEventAreaEntranceInfo {
    fn name() -> String {
        "QuestEventAreaEntranceInfo".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestEventAreaEntranceInfo::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestEventAreaEntranceInfo {
    pub r#quest: u32,
    pub r#unknown1: u8,
    pub r#location: u32,
}
impl QuestEventAreaEntranceInfo {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#location: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
