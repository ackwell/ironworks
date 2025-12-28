use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
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
    pub r#unknown5: i16,
    pub r#unknown6: i8,
    pub r#unknown7: u8,
    pub r#unknown8: i16,
    pub r#unknown9: i8,
    pub r#unknown10: u8,
    pub r#unknown11: i16,
}
impl ContentGauge {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#name: row.field(1usize + offset)?.into_string()?,
            r#color: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#text_string: row.field(4usize + offset)?.into_string()?,
            r#unknown5: row.field(5usize + offset)?.into_i16()?,
            r#unknown6: row.field(6usize + offset)?.into_i8()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
            r#unknown8: row.field(8usize + offset)?.into_i16()?,
            r#unknown9: row.field(9usize + offset)?.into_i8()?,
            r#unknown10: row.field(10usize + offset)?.into_u8()?,
            r#unknown11: row.field(11usize + offset)?.into_i16()?,
        })
    }
}
