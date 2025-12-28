use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for EventIconPriorityPair {
    fn name() -> String {
        "EventIconPriorityPair".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventIconPriorityPair::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventIconPriorityPair {
    pub r#icon: u32,
}
impl EventIconPriorityPair {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
