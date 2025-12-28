use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for QuestRedoChapterUI {
    fn name() -> String {
        "QuestRedoChapterUI".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestRedoChapterUI::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestRedoChapterUI {
    pub r#quest: u32,
    pub r#unknown1: u32,
    pub r#ui_tab: u8,
    pub r#category: u8,
    pub r#unknown4: u8,
    pub r#quest_redo_ui_small: u32,
    pub r#quest_redo_ui_large: u32,
    pub r#quest_redo_ui_wide: u32,
    pub r#chapter_name: SeString,
    pub r#chapter_part: SeString,
    pub r#transient: SeString,
}
impl QuestRedoChapterUI {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
            r#ui_tab: row.field(2usize + offset)?.into_u8()?,
            r#category: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#quest_redo_ui_small: row.field(5usize + offset)?.into_u32()?,
            r#quest_redo_ui_large: row.field(6usize + offset)?.into_u32()?,
            r#quest_redo_ui_wide: row.field(7usize + offset)?.into_u32()?,
            r#chapter_name: row.field(8usize + offset)?.into_string()?,
            r#chapter_part: row.field(9usize + offset)?.into_string()?,
            r#transient: row.field(10usize + offset)?.into_string()?,
        })
    }
}
