use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
