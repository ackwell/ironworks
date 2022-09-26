use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for QuestRedoChapterUITab {
    fn name() -> String {
        "QuestRedoChapterUITab".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestRedoChapterUITab::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestRedoChapterUITab {
    pub r#unknown0: u8,
    pub r#icon1: u32,
    pub r#icon2: u32,
    pub r#text: SeString,
}
impl QuestRedoChapterUITab {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#icon1: row.field(1usize + offset)?.into_u32()?,
            r#icon2: row.field(2usize + offset)?.into_u32()?,
            r#text: row.field(3usize + offset)?.into_string()?,
        })
    }
}
