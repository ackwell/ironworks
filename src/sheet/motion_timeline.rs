use ironworks::sestring::SeString;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MotionTimeline {
    fn name() -> String {
        "MotionTimeline".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MotionTimeline::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MotionTimeline {
    pub r#filename: SeString,
    pub r#blend_group: u8,
    pub r#is_loop: bool,
    pub r#is_blink_enable: bool,
    pub r#is_lip_enable: bool,
}
impl MotionTimeline {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#filename: row.field(0usize + offset)?.into_string()?,
            r#blend_group: row.field(1usize + offset)?.into_u8()?,
            r#is_loop: row.field(2usize + offset)?.into_bool()?,
            r#is_blink_enable: row.field(3usize + offset)?.into_bool()?,
            r#is_lip_enable: row.field(4usize + offset)?.into_bool()?,
        })
    }
}
