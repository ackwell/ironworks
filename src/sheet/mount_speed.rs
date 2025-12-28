use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MountSpeed {
    fn name() -> String {
        "MountSpeed".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MountSpeed::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MountSpeed {
    pub r#quest: u32,
    pub r#unknown1: u32,
    pub r#unknown2: u8,
}
impl MountSpeed {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
