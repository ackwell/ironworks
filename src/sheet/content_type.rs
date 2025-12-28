use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
    pub r#unknown3: u8,
    pub r#unknown4: u8,
}
impl ContentType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#icon_duty_finder: row.field(2usize + offset)?.into_u32()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
