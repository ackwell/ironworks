use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for BGMFade {
    fn name() -> String {
        "BGMFade".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BGMFade::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BGMFade {
    pub r#scene_out: i32,
    pub r#scene_in: i32,
    pub r#bgm_fade_type: i32,
}
impl BGMFade {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#scene_out: row.field(0usize + offset)?.into_i32()?,
            r#scene_in: row.field(1usize + offset)?.into_i32()?,
            r#bgm_fade_type: row.field(2usize + offset)?.into_i32()?,
        })
    }
}
