use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for RecipeLookup {
    fn name() -> String {
        "RecipeLookup".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RecipeLookup::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RecipeLookup {
    pub r#crp: u16,
    pub r#bsm: u16,
    pub r#arm: u16,
    pub r#gsm: u16,
    pub r#ltw: u16,
    pub r#wvr: u16,
    pub r#alc: u16,
    pub r#cul: u16,
}
impl RecipeLookup {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#crp: row.field(0usize + offset)?.into_u16()?,
            r#bsm: row.field(1usize + offset)?.into_u16()?,
            r#arm: row.field(2usize + offset)?.into_u16()?,
            r#gsm: row.field(3usize + offset)?.into_u16()?,
            r#ltw: row.field(4usize + offset)?.into_u16()?,
            r#wvr: row.field(5usize + offset)?.into_u16()?,
            r#alc: row.field(6usize + offset)?.into_u16()?,
            r#cul: row.field(7usize + offset)?.into_u16()?,
        })
    }
}
