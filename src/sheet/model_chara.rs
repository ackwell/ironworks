use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ModelChara {
    fn name() -> String {
        "ModelChara".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ModelChara::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ModelChara {
    pub r#type: u8,
    pub r#model: u16,
    pub r#base: u8,
    pub r#variant: u8,
    pub r#se_pack: u16,
    pub r#unknown5: u8,
    pub r#unknown6: bool,
    pub r#pap_variation: bool,
    pub r#unknown8: u8,
    pub r#unknown9: i8,
    pub r#unknown10: bool,
    pub r#unknown11: bool,
    pub r#unknown12: bool,
    pub r#unknown13: bool,
    pub r#unknown14: bool,
    pub r#unknown15: u8,
    pub r#unknown16: bool,
    pub r#unknown17: u8,
    pub r#unknown18: u8,
    pub r#unknown19: f32,
    pub r#unknown20: f32,
    pub r#unknown21: u8,
    pub r#unknown22: f32,
}
impl ModelChara {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#model: row.field(1usize + offset)?.into_u16()?,
            r#base: row.field(2usize + offset)?.into_u8()?,
            r#variant: row.field(3usize + offset)?.into_u8()?,
            r#se_pack: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
            r#pap_variation: row.field(7usize + offset)?.into_bool()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#unknown9: row.field(9usize + offset)?.into_i8()?,
            r#unknown10: row.field(10usize + offset)?.into_bool()?,
            r#unknown11: row.field(11usize + offset)?.into_bool()?,
            r#unknown12: row.field(12usize + offset)?.into_bool()?,
            r#unknown13: row.field(13usize + offset)?.into_bool()?,
            r#unknown14: row.field(14usize + offset)?.into_bool()?,
            r#unknown15: row.field(15usize + offset)?.into_u8()?,
            r#unknown16: row.field(16usize + offset)?.into_bool()?,
            r#unknown17: row.field(17usize + offset)?.into_u8()?,
            r#unknown18: row.field(18usize + offset)?.into_u8()?,
            r#unknown19: row.field(19usize + offset)?.into_f32()?,
            r#unknown20: row.field(20usize + offset)?.into_f32()?,
            r#unknown21: row.field(21usize + offset)?.into_u8()?,
            r#unknown22: row.field(22usize + offset)?.into_f32()?,
        })
    }
}
