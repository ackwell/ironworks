use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ActionParam {
    fn name() -> String {
        "ActionParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActionParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActionParam {
    pub r#name: i16,
}
impl ActionParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_i16()?,
        })
    }
}
