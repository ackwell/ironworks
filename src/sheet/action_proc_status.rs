use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for ActionProcStatus {
    fn name() -> String {
        "ActionProcStatus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActionProcStatus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActionProcStatus {
    pub r#status: u16,
}
impl ActionProcStatus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#status: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
