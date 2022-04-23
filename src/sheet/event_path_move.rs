use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for EventPathMove {
    fn name() -> String {
        "EventPathMove".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventPathMove::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventPathMove {}
impl EventPathMove {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
