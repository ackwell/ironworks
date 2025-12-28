use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for RetainerTaskLvRange {
    fn name() -> String {
        "RetainerTaskLvRange".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RetainerTaskLvRange::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RetainerTaskLvRange {
    pub r#min: u8,
    pub r#max: u8,
}
impl RetainerTaskLvRange {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#min: row.field(0usize + offset)?.into_u8()?,
            r#max: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
