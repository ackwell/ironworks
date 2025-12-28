use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for WebGuidance {
    fn name() -> String {
        "WebGuidance".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WebGuidance::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WebGuidance {
    pub r#image: i32,
    pub r#url: u8,
    pub r#name: SeString,
    pub r#unknown3: SeString,
    pub r#description: SeString,
}
impl WebGuidance {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#image: row.field(0usize + offset)?.into_i32()?,
            r#url: row.field(1usize + offset)?.into_u8()?,
            r#name: row.field(2usize + offset)?.into_string()?,
            r#unknown3: row.field(3usize + offset)?.into_string()?,
            r#description: row.field(4usize + offset)?.into_string()?,
        })
    }
}
