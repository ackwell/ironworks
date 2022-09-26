use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ActivityFeedButtons {
    fn name() -> String {
        "ActivityFeedButtons".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActivityFeedButtons::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActivityFeedButtons {
    pub r#unknown0: u8,
    pub r#banner_url: SeString,
    pub r#description: SeString,
    pub r#language: SeString,
    pub r#picture_url: SeString,
}
impl ActivityFeedButtons {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#banner_url: row.field(1usize + offset)?.into_string()?,
            r#description: row.field(2usize + offset)?.into_string()?,
            r#language: row.field(3usize + offset)?.into_string()?,
            r#picture_url: row.field(4usize + offset)?.into_string()?,
        })
    }
}
