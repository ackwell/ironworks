use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
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
    pub r#unknown2: u8,
    pub r#text: u8,
    pub r#icon: u32,
}
impl WeeklyBingoOrderData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u32()?,
            r#data: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#text: row.field(3usize + offset)?.into_u8()?,
            r#icon: row.field(4usize + offset)?.into_u32()?,
        })
    }
}
