use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for MYCTemporaryItem {
    fn name() -> String {
        "MYCTemporaryItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MYCTemporaryItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MYCTemporaryItem {
    pub r#category: u8,
    pub r#type: u8,
    pub r#action: u32,
    pub r#max: u8,
    pub r#weight: u8,
    pub r#order: u8,
}
impl MYCTemporaryItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_u8()?,
            r#type: row.field(1usize + offset)?.into_u8()?,
            r#action: row.field(2usize + offset)?.into_u32()?,
            r#max: row.field(3usize + offset)?.into_u8()?,
            r#weight: row.field(4usize + offset)?.into_u8()?,
            r#order: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
