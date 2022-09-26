use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for ContentGauge {
    fn name() -> String {
        "ContentGauge".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentGauge::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentGauge {
    pub r#unknown0: u8,
    pub r#name: SeString,
    pub r#color: u8,
    pub r#unknown3: bool,
    pub r#text_string: SeString,
}
impl ContentGauge {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#name: row.field(1usize + offset)?.into_string()?,
            r#color: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#text_string: row.field(4usize + offset)?.into_string()?,
        })
    }
}
