use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
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
    pub r#balloon_time: f32,
    pub r#is_balloon_slow: bool,
    pub r#battle_talk_time: bool,
    pub r#unknown8: u8,
    pub r#text: u8,
}
impl NpcYell {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#unknown2: row.field(2usize + offset)?.into_bool()?,
            r#unknown3: row.field(3usize + offset)?.into_bool()?,
            r#output_type: row.field(4usize + offset)?.into_u8()?,
            r#balloon_time: row.field(5usize + offset)?.into_f32()?,
            r#is_balloon_slow: row.field(6usize + offset)?.into_bool()?,
            r#battle_talk_time: row.field(7usize + offset)?.into_bool()?,
            r#unknown8: row.field(8usize + offset)?.into_u8()?,
            r#text: row.field(9usize + offset)?.into_u8()?,
        })
    }
}
