use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MJIGardenscaping {
    fn name() -> String {
        "MJIGardenscaping".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIGardenscaping::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIGardenscaping {
    pub r#level: u8,
    pub r#unknown1: i32,
    pub r#unknown2: i32,
    pub r#unknown3: i32,
    pub r#unknown4: u16,
    pub r#item: i32,
    pub r#unknown6: u16,
    pub r#unknown7: i32,
    pub r#unknown8: u16,
}
impl MJIGardenscaping {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#level: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_i32()?,
            r#unknown2: row.field(2usize + offset)?.into_i32()?,
            r#unknown3: row.field(3usize + offset)?.into_i32()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#item: row.field(5usize + offset)?.into_i32()?,
            r#unknown6: row.field(6usize + offset)?.into_u16()?,
            r#unknown7: row.field(7usize + offset)?.into_i32()?,
            r#unknown8: row.field(8usize + offset)?.into_u16()?,
        })
    }
}
