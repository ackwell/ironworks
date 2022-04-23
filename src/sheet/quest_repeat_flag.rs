use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for QuestRepeatFlag {
    fn name() -> String {
        "QuestRepeatFlag".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestRepeatFlag::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestRepeatFlag {
    pub r#quest: u32,
}
impl QuestRepeatFlag {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
