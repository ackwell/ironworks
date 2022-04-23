use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for EventItemTimeline {
    fn name() -> String {
        "EventItemTimeline".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventItemTimeline::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventItemTimeline {
    pub r#action_timeline: u32,
}
impl EventItemTimeline {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action_timeline: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
