use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#friendly_vfx: u16,
    pub r#stack_vfx1_trigger: u8,
    pub r#stack_vfx1: u16,
    pub r#stack_vfx2_trigger: u8,
    pub r#stack_vfx2: u16,
    pub r#hostile_vfx: u16,
    pub r#unknown6: u8,
    pub r#unknown7: u8,
    pub r#unknown8: bool,
    pub r#unknown9: bool,
    pub r#unknown10: bool,
}
impl StatusLoopVFX {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#friendly_vfx: row.field(0usize + offset)?.into_u16()?,
            r#stack_vfx1_trigger: row.field(1usize + offset)?.into_u8()?,
            r#stack_vfx1: row.field(2usize + offset)?.into_u16()?,
            r#stack_vfx2_trigger: row.field(3usize + offset)?.into_u8()?,
            r#stack_vfx2: row.field(4usize + offset)?.into_u16()?,
            r#hostile_vfx: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
            r#unknown10: row.field(10usize + offset)?.into_bool()?,
        })
    }
}
