use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for WeeklyBingoOrderData {
    fn name() -> String {
        "WeeklyBingoOrderData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WeeklyBingoOrderData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WeeklyBingoOrderData {
    pub r#type: u32,
    pub r#data: u32,
    pub r#unknown2: u32,
    pub r#text: u8,
    pub r#icon: u8,
    pub r#unknown5: u32,
    pub r#unknown6: u8,
}
impl WeeklyBingoOrderData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u32()?,
            r#data: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u32()?,
            r#text: row.field(3usize + offset)?.into_u8()?,
            r#icon: row.field(4usize + offset)?.into_u8()?,
            r#unknown5: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
        })
    }
}
