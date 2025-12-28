use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
pub struct ReactionEventObject {
    pub r#unknown0: u32,
    pub r#unknown1: u32,
}
impl ReactionEventObject {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
