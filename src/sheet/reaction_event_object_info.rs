use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for ReactionEventObjectInfo {
    fn name() -> String {
        "ReactionEventObjectInfo".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ReactionEventObjectInfo::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ReactionEventObjectInfo {}
impl ReactionEventObjectInfo {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
