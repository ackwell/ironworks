use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for GatheringExp {
    fn name() -> String {
        "GatheringExp".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringExp::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringExp {
    pub r#exp: i32,
}
impl GatheringExp {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#exp: row.field(0usize + offset)?.into_i32()?,
        })
    }
}
