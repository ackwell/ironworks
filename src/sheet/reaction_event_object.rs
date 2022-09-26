use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ReactionEventObject {
    fn name() -> String {
        "ReactionEventObject".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ReactionEventObject::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ReactionEventObject {}
impl ReactionEventObject {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
