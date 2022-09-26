use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for MJIVillageDevelopment {
    fn name() -> String {
        "MJIVillageDevelopment".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIVillageDevelopment::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIVillageDevelopment {
    pub r#enpc: u32,
    pub r#unknown1: u8,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
    pub r#unknown4: u32,
    pub r#unknown5: u8,
    pub r#unknown6: u32,
    pub r#unknown7: u8,
    pub r#unknown8: u32,
    pub r#behavior0: u16,
    pub r#unknown10: u32,
    pub r#behavior1: u16,
}
impl MJIVillageDevelopment {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#enpc: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_u32()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
            r#unknown8: row.field(8usize + offset)?.into_u32()?,
            r#behavior0: row.field(9usize + offset)?.into_u16()?,
            r#unknown10: row.field(10usize + offset)?.into_u32()?,
            r#behavior1: row.field(11usize + offset)?.into_u16()?,
        })
    }
}
