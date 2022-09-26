use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for MJIItemPouch {
    fn name() -> String {
        "MJIItemPouch".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIItemPouch::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIItemPouch {
    pub r#item: u32,
    pub r#category: i32,
    pub r#crop: u8,
}
impl MJIItemPouch {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u32()?,
            r#category: row.field(1usize + offset)?.into_i32()?,
            r#crop: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
