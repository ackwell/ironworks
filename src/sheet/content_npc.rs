use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ContentNpc {
    fn name() -> String {
        "ContentNpc".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentNpc::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentNpc {
    pub r#unknown0: bool,
    pub r#unknown1: bool,
}
impl ContentNpc {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
        })
    }
}
