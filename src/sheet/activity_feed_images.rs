use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ActivityFeedImages {
    fn name() -> String {
        "ActivityFeedImages".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActivityFeedImages::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActivityFeedImages {
    pub r#expansion_image: SeString,
    pub r#activity_feed_ja: SeString,
    pub r#activity_feed_en: SeString,
    pub r#activity_feed_de: SeString,
    pub r#activity_feed_fr: SeString,
}
impl ActivityFeedImages {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#expansion_image: row.field(0usize + offset)?.into_string()?,
            r#activity_feed_ja: row.field(1usize + offset)?.into_string()?,
            r#activity_feed_en: row.field(2usize + offset)?.into_string()?,
            r#activity_feed_de: row.field(3usize + offset)?.into_string()?,
            r#activity_feed_fr: row.field(4usize + offset)?.into_string()?,
        })
    }
}
