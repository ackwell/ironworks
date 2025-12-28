use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#unknown2: i16,
    pub r#unknown3: i16,
    pub r#unknown4: i16,
    pub r#unknown5: i16,
}
impl RideShootingTargetType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#e_obj: row.field(0usize + offset)?.into_u32()?,
            r#score: row.field(1usize + offset)?.into_i16()?,
            r#unknown2: row.field(2usize + offset)?.into_i16()?,
            r#unknown3: row.field(3usize + offset)?.into_i16()?,
            r#unknown4: row.field(4usize + offset)?.into_i16()?,
            r#unknown5: row.field(5usize + offset)?.into_i16()?,
        })
    }
}
