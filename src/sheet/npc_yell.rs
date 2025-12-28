use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for NpcYell {
    fn name() -> String {
        "NpcYell".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(NpcYell::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct NpcYell {
    pub r#unknown0: u32,
    pub r#unknown1: bool,
    pub r#unknown2: bool,
    pub r#unknown3: bool,
    pub r#output_type: u8,
    pub r#unknown5: u8,
    pub r#balloon_time: u32,
    pub r#is_balloon_slow: f32,
    pub r#battle_talk_time: f32,
    pub r#unknown9: bool,
    pub r#unknown10: bool,
    pub r#text: u8,
    pub r#unknown12: u8,
    pub r#unknown13: SeString,
    pub r#unknown14: u16,
}
impl NpcYell {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#output_type: row.field(4usize + offset)?.into_u8()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#balloon_time: row.field(6usize + offset)?.into_u32()?,
            r#is_balloon_slow: row.field(7usize + offset)?.into_f32()?,
            r#battle_talk_time: row.field(8usize + offset)?.into_f32()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
            r#unknown10: row.field(10usize + offset)?.into_bool()?,
            r#text: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
            r#unknown13: row.field(13usize + offset)?.into_string()?,
            r#unknown14: row.field(14usize + offset)?.into_u16()?,
        })
    }
}
