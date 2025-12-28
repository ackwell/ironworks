use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Marker {
    fn name() -> String {
        "Marker".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Marker::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Marker {
    pub r#icon: i32,
    pub r#unknown1: u8,
    pub r#name: SeString,
}
impl Marker {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#name: row.field(2usize + offset)?.into_string()?,
        })
    }
}
