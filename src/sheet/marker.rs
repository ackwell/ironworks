use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
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
    pub r#name: SeString,
}
impl Marker {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
            r#name: row.field(1usize + offset)?.into_string()?,
        })
    }
}
