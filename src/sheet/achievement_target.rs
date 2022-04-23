use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for AchievementTarget {
    fn name() -> String {
        "AchievementTarget".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AchievementTarget::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AchievementTarget {
    pub r#type: u8,
    pub r#value: u32,
}
impl AchievementTarget {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#value: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
