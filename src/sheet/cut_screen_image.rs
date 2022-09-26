use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
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
}
impl CutScreenImage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_i16()?,
            r#image: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
