use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for HWDAnnounce {
    fn name() -> String {
        "HWDAnnounce".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDAnnounce::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDAnnounce {
    pub r#name: SeString,
    pub r#enpc: u32,
    pub r#unknown2: u8,
    pub r#unknown3: u8,
}
impl HWDAnnounce {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#enpc: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
