use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for EventItemCastTimeline {
    fn name() -> String {
        "EventItemCastTimeline".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventItemCastTimeline::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventItemCastTimeline {
    pub r#action_timeline: u32,
}
impl EventItemCastTimeline {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action_timeline: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
