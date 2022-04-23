use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for AchievementCategory {
    fn name() -> String {
        "AchievementCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AchievementCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AchievementCategory {
    pub r#name: SeString,
    pub r#achievement_kind: u8,
    pub r#show_complete: bool,
    pub r#hide_category: bool,
    pub r#order: u8,
}
impl AchievementCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#achievement_kind: row.field(1usize + offset)?.into_u8()?,
            r#show_complete: row.field(2usize + offset)?.into_bool()?,
            r#hide_category: row.field(3usize + offset)?.into_bool()?,
            r#order: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
