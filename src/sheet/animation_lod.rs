use crate::utility::read_array;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use std::vec::Vec;
impl MetadataAdapter for AnimationLOD {
    fn name() -> String {
        "AnimationLOD".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AnimationLOD::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AnimationLOD {
    pub r#camera_distance: f32,
    pub r#sample_interval: f32,
    pub r#bone_lod: i8,
    pub r#animation_enable: Vec<bool>,
}
impl AnimationLOD {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#camera_distance: row.field(0usize + offset)?.into_f32()?,
            r#sample_interval: row.field(1usize + offset)?.into_f32()?,
            r#bone_lod: row.field(2usize + offset)?.into_i8()?,
            r#animation_enable: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_bool()?) },
            )?,
        })
    }
}
