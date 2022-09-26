use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for ContentType {
    fn name() -> String {
        "ContentType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentType {
    pub r#name: SeString,
    pub r#icon: u32,
    pub r#icon_duty_finder: u32,
}
impl ContentType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#icon_duty_finder: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
