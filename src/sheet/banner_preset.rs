use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for BannerPreset {
    fn name() -> String {
        "BannerPreset".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BannerPreset::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BannerPreset {
    pub r#unknown0: f32,
    pub r#unknown1: f32,
    pub r#unknown2: f32,
    pub r#unknown3: f32,
    pub r#unknown4: f32,
    pub r#unknown5: f32,
    pub r#unknown6: i16,
    pub r#unknown7: u8,
    pub r#unknown8: u16,
    pub r#unknown9: f32,
    pub r#unknown10: i32,
    pub r#unknown11: f32,
    pub r#unknown12: f32,
    pub r#unknown13: f32,
    pub r#unknown14: f32,
    pub r#unknown15: u8,
    pub r#unknown16: u8,
    pub r#unknown17: u8,
    pub r#unknown18: u8,
    pub r#unknown19: u8,
    pub r#unknown20: i16,
    pub r#unknown21: i16,
    pub r#unknown22: u8,
    pub r#unknown23: u8,
    pub r#unknown24: u8,
    pub r#unknown25: u8,
}
impl BannerPreset {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_f32()?,
            r#unknown1: row.field(1usize + offset)?.into_f32()?,
            r#unknown2: row.field(2usize + offset)?.into_f32()?,
            r#unknown3: row.field(3usize + offset)?.into_f32()?,
            r#unknown4: row.field(4usize + offset)?.into_f32()?,
            r#unknown5: row.field(5usize + offset)?.into_f32()?,
            r#unknown6: row.field(6usize + offset)?.into_i16()?,
            r#unknown7: row.field(7usize + offset)?.into_u8()?,
            r#unknown8: row.field(8usize + offset)?.into_u16()?,
            r#unknown9: row.field(9usize + offset)?.into_f32()?,
            r#unknown10: row.field(10usize + offset)?.into_i32()?,
            r#unknown11: row.field(11usize + offset)?.into_f32()?,
            r#unknown12: row.field(12usize + offset)?.into_f32()?,
            r#unknown13: row.field(13usize + offset)?.into_f32()?,
            r#unknown14: row.field(14usize + offset)?.into_f32()?,
            r#unknown15: row.field(15usize + offset)?.into_u8()?,
            r#unknown16: row.field(16usize + offset)?.into_u8()?,
            r#unknown17: row.field(17usize + offset)?.into_u8()?,
            r#unknown18: row.field(18usize + offset)?.into_u8()?,
            r#unknown19: row.field(19usize + offset)?.into_u8()?,
            r#unknown20: row.field(20usize + offset)?.into_i16()?,
            r#unknown21: row.field(21usize + offset)?.into_i16()?,
            r#unknown22: row.field(22usize + offset)?.into_u8()?,
            r#unknown23: row.field(23usize + offset)?.into_u8()?,
            r#unknown24: row.field(24usize + offset)?.into_u8()?,
            r#unknown25: row.field(25usize + offset)?.into_u8()?,
        })
    }
}
