use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MJIGatheringObject {
    fn name() -> String {
        "MJIGatheringObject".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIGatheringObject::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIGatheringObject {
    pub r#sgb: u16,
    pub r#map_icon: u32,
    pub r#unknown2: u32,
    pub r#name: u32,
    pub r#unknown4: u16,
}
impl MJIGatheringObject {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#sgb: row.field(0usize + offset)?.into_u16()?,
            r#map_icon: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u32()?,
            r#name: row.field(3usize + offset)?.into_u32()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
        })
    }
}
