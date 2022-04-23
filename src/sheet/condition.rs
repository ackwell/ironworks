use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Condition {
    fn name() -> String {
        "Condition".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Condition::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Condition {
    pub r#unknown0: bool,
    pub r#unknown1: u8,
    pub r#log_message: u32,
}
impl Condition {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#log_message: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
