use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for PresetCamera {
    fn name() -> String {
        "PresetCamera".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PresetCamera::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PresetCamera {
    pub r#eid: u16,
    pub r#pos_x: f32,
    pub r#pos_y: f32,
    pub r#pos_z: f32,
    pub r#elezen: f32,
    pub r#lalafell: f32,
    pub r#miqote: f32,
    pub r#roe: f32,
    pub r#hrothgar: f32,
    pub r#viera: f32,
    pub r#unknown10: f32,
    pub r#hyur_f: f32,
    pub r#elezen_f: f32,
    pub r#lalafell_f: f32,
    pub r#miqote_f: f32,
    pub r#roe_f: f32,
    pub r#hrothgar_f: f32,
    pub r#viera_f: f32,
}
impl PresetCamera {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#eid: row.field(0usize + offset)?.into_u16()?,
            r#pos_x: row.field(1usize + offset)?.into_f32()?,
            r#pos_y: row.field(2usize + offset)?.into_f32()?,
            r#pos_z: row.field(3usize + offset)?.into_f32()?,
            r#elezen: row.field(4usize + offset)?.into_f32()?,
            r#lalafell: row.field(5usize + offset)?.into_f32()?,
            r#miqote: row.field(6usize + offset)?.into_f32()?,
            r#roe: row.field(7usize + offset)?.into_f32()?,
            r#hrothgar: row.field(8usize + offset)?.into_f32()?,
            r#viera: row.field(9usize + offset)?.into_f32()?,
            r#unknown10: row.field(10usize + offset)?.into_f32()?,
            r#hyur_f: row.field(11usize + offset)?.into_f32()?,
            r#elezen_f: row.field(12usize + offset)?.into_f32()?,
            r#lalafell_f: row.field(13usize + offset)?.into_f32()?,
            r#miqote_f: row.field(14usize + offset)?.into_f32()?,
            r#roe_f: row.field(15usize + offset)?.into_f32()?,
            r#hrothgar_f: row.field(16usize + offset)?.into_f32()?,
            r#viera_f: row.field(17usize + offset)?.into_f32()?,
        })
    }
}
