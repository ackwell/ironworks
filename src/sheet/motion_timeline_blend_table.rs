use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for MotionTimelineBlendTable {
    fn name() -> String {
        "MotionTimelineBlendTable".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MotionTimelineBlendTable::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MotionTimelineBlendTable {
    pub r#dest_blend_group: u8,
    pub r#src_blend_group: u8,
    pub r#blend_frame_pc: u8,
    pub r#blend_fram_type_a: u8,
    pub r#blend_fram_type_b: u8,
    pub r#blend_fram_type_c: u8,
}
impl MotionTimelineBlendTable {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#dest_blend_group: row.field(0usize + offset)?.into_u8()?,
            r#src_blend_group: row.field(1usize + offset)?.into_u8()?,
            r#blend_frame_pc: row.field(2usize + offset)?.into_u8()?,
            r#blend_fram_type_a: row.field(3usize + offset)?.into_u8()?,
            r#blend_fram_type_b: row.field(4usize + offset)?.into_u8()?,
            r#blend_fram_type_c: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
