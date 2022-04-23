use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for SkyIsland2RangeType {
    fn name() -> String {
        "SkyIsland2RangeType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SkyIsland2RangeType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SkyIsland2RangeType {
    pub r#type: u8,
}
impl SkyIsland2RangeType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
