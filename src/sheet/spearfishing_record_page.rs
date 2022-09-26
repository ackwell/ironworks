use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for SpearfishingRecordPage {
    fn name() -> String {
        "SpearfishingRecordPage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SpearfishingRecordPage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SpearfishingRecordPage {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#unknown2: u8,
    pub r#place_name: i32,
    pub r#image: i32,
}
impl SpearfishingRecordPage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#place_name: row.field(3usize + offset)?.into_i32()?,
            r#image: row.field(4usize + offset)?.into_i32()?,
        })
    }
}
