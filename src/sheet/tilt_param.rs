use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for TiltParam {
    fn name() -> String {
        "TiltParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TiltParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TiltParam {
    pub r#rotation_origin_offset: u8,
    pub r#max_angle: u8,
    pub r#reverse_rotation: bool,
    pub r#tilt_rate: f32,
    pub r#unknown4: u8,
    pub r#unknown5: u8,
}
impl TiltParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#rotation_origin_offset: row.field(0usize + offset)?.into_u8()?,
            r#max_angle: row.field(1usize + offset)?.into_u8()?,
            r#reverse_rotation: row.field(2usize + offset)?.into_bool()?,
            r#tilt_rate: row.field(3usize + offset)?.into_f32()?,
            r#unknown4: row.field(4usize + offset)?.into_u8()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
