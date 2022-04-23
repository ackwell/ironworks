use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
impl MetadataAdapter for RetainerTaskRandom {
    fn name() -> String {
        "RetainerTaskRandom".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RetainerTaskRandom::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RetainerTaskRandom {
    pub r#name: SeString,
    pub r#requirement: i16,
}
impl RetainerTaskRandom {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#requirement: row.field(1usize + offset)?.into_i16()?,
        })
    }
}
