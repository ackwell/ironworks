use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for QuestRedoChapterUICategory {
    fn name() -> String {
        "QuestRedoChapterUICategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestRedoChapterUICategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestRedoChapterUICategory {
    pub r#unknown0: u8,
    pub r#expac: SeString,
}
impl QuestRedoChapterUICategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#expac: row.field(1usize + offset)?.into_string()?,
        })
    }
}
