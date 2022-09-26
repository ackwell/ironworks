use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for GatheringItemPoint {
    fn name() -> String {
        "GatheringItemPoint".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringItemPoint::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringItemPoint {
    pub r#gathering_point: u32,
}
impl GatheringItemPoint {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#gathering_point: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
