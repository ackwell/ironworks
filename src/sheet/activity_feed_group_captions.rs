use ironworks::sestring::SeString;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for ActivityFeedGroupCaptions {
    fn name() -> String {
        "ActivityFeedGroupCaptions".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActivityFeedGroupCaptions::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActivityFeedGroupCaptions {
    pub r#ja: SeString,
    pub r#en: SeString,
    pub r#de: SeString,
    pub r#fr: SeString,
}
impl ActivityFeedGroupCaptions {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#ja: row.field(0usize + offset)?.into_string()?,
            r#en: row.field(1usize + offset)?.into_string()?,
            r#de: row.field(2usize + offset)?.into_string()?,
            r#fr: row.field(3usize + offset)?.into_string()?,
        })
    }
}
