use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for AchievementKind {
    fn name() -> String {
        "AchievementKind".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AchievementKind::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AchievementKind {
    pub r#name: SeString,
    pub r#order: u8,
}
impl AchievementKind {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#order: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
