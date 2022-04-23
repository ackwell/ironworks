use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for MoveVfx {
    fn name() -> String {
        "MoveVfx".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MoveVfx::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MoveVfx {
    pub r#vfx_normal: u16,
    pub r#vfx_walking: u16,
}
impl MoveVfx {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#vfx_normal: row.field(0usize + offset)?.into_u16()?,
            r#vfx_walking: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
