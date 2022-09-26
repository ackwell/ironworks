use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for QuestChapter {
    fn name() -> String {
        "QuestChapter".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestChapter::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestChapter {
    pub r#quest: u32,
    pub r#redo: u16,
}
impl QuestChapter {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
            r#redo: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
