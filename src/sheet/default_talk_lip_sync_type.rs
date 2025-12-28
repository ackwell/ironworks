use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for DefaultTalkLipSyncType {
    fn name() -> String {
        "DefaultTalkLipSyncType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DefaultTalkLipSyncType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DefaultTalkLipSyncType {
    pub r#action_timeline: i32,
}
impl DefaultTalkLipSyncType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action_timeline: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
