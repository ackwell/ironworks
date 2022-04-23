use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for ExportedGatheringPoint {
    fn name() -> String {
        "ExportedGatheringPoint".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ExportedGatheringPoint::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ExportedGatheringPoint {
    pub r#x: f32,
    pub r#y: f32,
    pub r#gathering_type: u8,
    pub r#gathering_point_type: u8,
    pub r#radius: u16,
}
impl ExportedGatheringPoint {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#x: row.field(0usize + offset)?.into_f32()?,
            r#y: row.field(1usize + offset)?.into_f32()?,
            r#gathering_type: row.field(2usize + offset)?.into_u8()?,
            r#gathering_point_type: row.field(3usize + offset)?.into_u8()?,
            r#radius: row.field(4usize + offset)?.into_u16()?,
        })
    }
}
