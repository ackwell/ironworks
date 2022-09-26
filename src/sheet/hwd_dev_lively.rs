use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
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
}
impl HWDDevLively {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#enpc: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
