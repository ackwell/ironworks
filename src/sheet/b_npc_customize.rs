use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for BNpcCustomize {
    fn name() -> String {
        "BNpcCustomize".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BNpcCustomize::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BNpcCustomize {
    pub r#race: u8,
    pub r#gender: u8,
    pub r#body_type: u8,
    pub r#height: u8,
    pub r#tribe: u8,
    pub r#face: u8,
    pub r#hair_style: u8,
    pub r#hair_highlight: u8,
    pub r#skin_color: u8,
    pub r#eye_heterochromia: u8,
    pub r#hair_color: u8,
    pub r#hair_highlight_color: u8,
    pub r#facial_feature: u8,
    pub r#facial_feature_color: u8,
    pub r#eyebrows: u8,
    pub r#eye_color: u8,
    pub r#eye_shape: u8,
    pub r#nose: u8,
    pub r#jaw: u8,
    pub r#mouth: u8,
    pub r#lip_color: u8,
    pub r#bust_or_tone1: u8,
    pub r#extra_feature1: u8,
    pub r#extra_feature2_or_bust: u8,
    pub r#face_paint: u8,
    pub r#face_paint_color: u8,
}
impl BNpcCustomize {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#race: row.field(0usize + offset)?.into_u8()?,
            r#gender: row.field(1usize + offset)?.into_u8()?,
            r#body_type: row.field(2usize + offset)?.into_u8()?,
            r#height: row.field(3usize + offset)?.into_u8()?,
            r#tribe: row.field(4usize + offset)?.into_u8()?,
            r#face: row.field(5usize + offset)?.into_u8()?,
            r#hair_style: row.field(6usize + offset)?.into_u8()?,
            r#hair_highlight: row.field(7usize + offset)?.into_u8()?,
            r#skin_color: row.field(8usize + offset)?.into_u8()?,
            r#eye_heterochromia: row.field(9usize + offset)?.into_u8()?,
            r#hair_color: row.field(10usize + offset)?.into_u8()?,
            r#hair_highlight_color: row.field(11usize + offset)?.into_u8()?,
            r#facial_feature: row.field(12usize + offset)?.into_u8()?,
            r#facial_feature_color: row.field(13usize + offset)?.into_u8()?,
            r#eyebrows: row.field(14usize + offset)?.into_u8()?,
            r#eye_color: row.field(15usize + offset)?.into_u8()?,
            r#eye_shape: row.field(16usize + offset)?.into_u8()?,
            r#nose: row.field(17usize + offset)?.into_u8()?,
            r#jaw: row.field(18usize + offset)?.into_u8()?,
            r#mouth: row.field(19usize + offset)?.into_u8()?,
            r#lip_color: row.field(20usize + offset)?.into_u8()?,
            r#bust_or_tone1: row.field(21usize + offset)?.into_u8()?,
            r#extra_feature1: row.field(22usize + offset)?.into_u8()?,
            r#extra_feature2_or_bust: row.field(23usize + offset)?.into_u8()?,
            r#face_paint: row.field(24usize + offset)?.into_u8()?,
            r#face_paint_color: row.field(25usize + offset)?.into_u8()?,
        })
    }
}
