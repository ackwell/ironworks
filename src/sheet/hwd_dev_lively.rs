use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for HWDDevLively {
    fn name() -> String {
        "HWDDevLively".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDDevLively::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDDevLively {
    pub r#enpc: u32,
    pub r#unknown1: u16,
    pub r#unknown2: u8,
}
impl HWDDevLively {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#enpc: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
