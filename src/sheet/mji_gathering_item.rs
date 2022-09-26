use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MJIGatheringItem {
    fn name() -> String {
        "MJIGatheringItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIGatheringItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIGatheringItem {
    pub r#item: u32,
    pub r#sort: u8,
    pub r#unknown2: u8,
    pub r#x: i16,
    pub r#y: i16,
    pub r#radius: u16,
}
impl MJIGatheringItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u32()?,
            r#sort: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#x: row.field(3usize + offset)?.into_i16()?,
            r#y: row.field(4usize + offset)?.into_i16()?,
            r#radius: row.field(5usize + offset)?.into_u16()?,
        })
    }
}
