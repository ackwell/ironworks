use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for ConfigKey {
    fn name() -> String {
        "ConfigKey".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ConfigKey::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ConfigKey {
    pub r#label: SeString,
    pub r#param: u8,
    pub r#platform: u8,
    pub r#required: bool,
    pub r#category: u8,
    pub r#unknown5: u16,
    pub r#unknown6: u8,
    pub r#text: SeString,
}
impl ConfigKey {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#label: row.field(0usize + offset)?.into_string()?,
            r#param: row.field(1usize + offset)?.into_u8()?,
            r#platform: row.field(2usize + offset)?.into_u8()?,
            r#required: row.field(3usize + offset)?.into_bool()?,
            r#category: row.field(4usize + offset)?.into_u8()?,
            r#unknown5: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#text: row.field(7usize + offset)?.into_string()?,
        })
    }
}
