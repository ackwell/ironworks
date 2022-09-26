use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for PresetCameraAdjust {
    fn name() -> String {
        "PresetCameraAdjust".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PresetCameraAdjust::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PresetCameraAdjust {
    pub r#hyur_m: f32,
    pub r#hyur_f: f32,
    pub r#elezen_m: f32,
    pub r#elezen_f: f32,
    pub r#lalafell_m: f32,
    pub r#lalafell_f: f32,
    pub r#miqote_m: f32,
    pub r#miqote_f: f32,
    pub r#roe_m: f32,
    pub r#roe_f: f32,
    pub r#hrothgar_m: f32,
    pub r#hrothgar_f: f32,
    pub r#viera_m: f32,
    pub r#viera_f: f32,
}
impl PresetCameraAdjust {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#hyur_m: row.field(0usize + offset)?.into_f32()?,
            r#hyur_f: row.field(1usize + offset)?.into_f32()?,
            r#elezen_m: row.field(2usize + offset)?.into_f32()?,
            r#elezen_f: row.field(3usize + offset)?.into_f32()?,
            r#lalafell_m: row.field(4usize + offset)?.into_f32()?,
            r#lalafell_f: row.field(5usize + offset)?.into_f32()?,
            r#miqote_m: row.field(6usize + offset)?.into_f32()?,
            r#miqote_f: row.field(7usize + offset)?.into_f32()?,
            r#roe_m: row.field(8usize + offset)?.into_f32()?,
            r#roe_f: row.field(9usize + offset)?.into_f32()?,
            r#hrothgar_m: row.field(10usize + offset)?.into_f32()?,
            r#hrothgar_f: row.field(11usize + offset)?.into_f32()?,
            r#viera_m: row.field(12usize + offset)?.into_f32()?,
            r#viera_f: row.field(13usize + offset)?.into_f32()?,
        })
    }
}
