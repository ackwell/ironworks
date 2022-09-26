use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for StatusLoopVFX {
    fn name() -> String {
        "StatusLoopVFX".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(StatusLoopVFX::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct StatusLoopVFX {
    pub r#vfx: u16,
    pub r#unknown1: u8,
    pub r#vfx2: u16,
    pub r#unknown3: u8,
    pub r#vfx3: u16,
}
impl StatusLoopVFX {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#vfx: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#vfx2: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#vfx3: row.field(4usize + offset)?.into_u16()?,
        })
    }
}
