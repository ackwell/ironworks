use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ScreenImage {
    fn name() -> String {
        "ScreenImage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ScreenImage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ScreenImage {
    pub r#image: u32,
    pub r#jingle: i16,
    pub r#type: i8,
    pub r#lang: bool,
}
impl ScreenImage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_u32()?,
            r#jingle: row.field(1usize + offset)?.into_i16()?,
            r#type: row.field(2usize + offset)?.into_i8()?,
            r#lang: row.field(3usize + offset)?.into_bool()?,
        })
    }
}
