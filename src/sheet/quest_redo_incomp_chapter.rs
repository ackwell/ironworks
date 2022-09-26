use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for QuestRedoIncompChapter {
    fn name() -> String {
        "QuestRedoIncompChapter".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestRedoIncompChapter::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestRedoIncompChapter {
    pub r#chapter: u16,
}
impl QuestRedoIncompChapter {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#chapter: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
