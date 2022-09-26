use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for Carry {
    fn name() -> String {
        "Carry".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Carry::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Carry {
    pub r#model: u64,
    pub r#timeline: u8,
}
impl Carry {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#model: row.field(0usize + offset)?.into_u64()?,
            r#timeline: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
