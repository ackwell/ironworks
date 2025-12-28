use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for AchievementHideCondition {
    fn name() -> String {
        "AchievementHideCondition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AchievementHideCondition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AchievementHideCondition {
    pub r#hide_achievement: bool,
    pub r#hide_name: bool,
    pub r#hide_conditions: bool,
}
impl AchievementHideCondition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#hide_achievement: row.field(0usize + offset)?.into_bool()?,
            r#hide_name: row.field(1usize + offset)?.into_bool()?,
            r#hide_conditions: row.field(2usize + offset)?.into_bool()?,
        })
    }
}
