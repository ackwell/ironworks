use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for RideShootingTargetType {
    fn name() -> String {
        "RideShootingTargetType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RideShootingTargetType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RideShootingTargetType {
    pub r#e_obj: u32,
    pub r#score: i16,
}
impl RideShootingTargetType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#e_obj: row.field(0usize + offset)?.into_u32()?,
            r#score: row.field(1usize + offset)?.into_i16()?,
        })
    }
}
