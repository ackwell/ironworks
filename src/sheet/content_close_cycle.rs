use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ContentCloseCycle {
    fn name() -> String {
        "ContentCloseCycle".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentCloseCycle::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentCloseCycle {
    pub r#unixtime: u32,
    pub r#time_seconds: u32,
}
impl ContentCloseCycle {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unixtime: row.field(0usize + offset)?.into_u32()?,
            r#time_seconds: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
