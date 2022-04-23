use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
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
}
impl HWDAnnounce {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#enpc: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
