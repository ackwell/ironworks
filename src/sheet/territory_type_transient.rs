use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for TerritoryTypeTransient {
    fn name() -> String {
        "TerritoryTypeTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TerritoryTypeTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TerritoryTypeTransient {
    pub r#offset_z: i16,
}
impl TerritoryTypeTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#offset_z: row.field(0usize + offset)?.into_i16()?,
        })
    }
}
