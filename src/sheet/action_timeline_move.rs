use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ActionTimelineMove {
    fn name() -> String {
        "ActionTimelineMove".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActionTimelineMove::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActionTimelineMove {}
impl ActionTimelineMove {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
