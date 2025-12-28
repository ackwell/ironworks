use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for BGMFadeType {
    fn name() -> String {
        "BGMFadeType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BGMFadeType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BGMFadeType {
    pub r#fade_out_time: f32,
    pub r#fade_in_time: f32,
    pub r#fade_in_start_time: f32,
    pub r#resume_fade_in_time: f32,
}
impl BGMFadeType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#fade_out_time: row.field(0usize + offset)?.into_f32()?,
            r#fade_in_time: row.field(1usize + offset)?.into_f32()?,
            r#fade_in_start_time: row.field(2usize + offset)?.into_f32()?,
            r#resume_fade_in_time: row.field(3usize + offset)?.into_f32()?,
        })
    }
}
