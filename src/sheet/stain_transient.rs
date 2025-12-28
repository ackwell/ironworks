use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for StainTransient {
    fn name() -> String {
        "StainTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(StainTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct StainTransient {
    pub r#item1: u32,
    pub r#item2: u32,
}
impl StainTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item1: row.field(0usize + offset)?.into_u32()?,
            r#item2: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
