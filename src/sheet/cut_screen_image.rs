use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for CutScreenImage {
    fn name() -> String {
        "CutScreenImage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CutScreenImage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CutScreenImage {
    pub r#type: i16,
    pub r#image: i32,
    pub r#unknown2: i16,
}
impl CutScreenImage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_i16()?,
            r#image: row.field(1usize + offset)?.into_i32()?,
            r#unknown2: row.field(2usize + offset)?.into_i16()?,
        })
    }
}
