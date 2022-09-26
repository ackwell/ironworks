use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for ContentGaugeColor {
    fn name() -> String {
        "ContentGaugeColor".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentGaugeColor::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentGaugeColor {
    pub r#android_color1: u32,
    pub r#android_color2: u32,
    pub r#android_color3: u32,
}
impl ContentGaugeColor {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#android_color1: row.field(0usize + offset)?.into_u32()?,
            r#android_color2: row.field(1usize + offset)?.into_u32()?,
            r#android_color3: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
