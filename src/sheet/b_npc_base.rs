use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for BNpcBase {
    fn name() -> String {
        "BNpcBase".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BNpcBase::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BNpcBase {
    pub r#behavior: u16,
    pub r#battalion: u8,
    pub r#link_race: u8,
    pub r#rank: u8,
    pub r#scale: f32,
    pub r#model_chara: u16,
    pub r#b_npc_customize: u16,
    pub r#npc_equip: u16,
    pub r#special: u16,
    pub r#se_pack: u8,
    pub r#unknown10: bool,
    pub r#array_event_handler: i32,
    pub r#b_npc_parts: u8,
    pub r#unknown13: u8,
    pub r#unknown14: u8,
    pub r#is_target_line: bool,
    pub r#is_display_level: bool,
}
impl BNpcBase {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#behavior: row.field(0usize + offset)?.into_u16()?,
            r#battalion: row.field(1usize + offset)?.into_u8()?,
            r#link_race: row.field(2usize + offset)?.into_u8()?,
            r#rank: row.field(3usize + offset)?.into_u8()?,
            r#scale: row.field(4usize + offset)?.into_f32()?,
            r#model_chara: row.field(5usize + offset)?.into_u16()?,
            r#b_npc_customize: row.field(6usize + offset)?.into_u16()?,
            r#npc_equip: row.field(7usize + offset)?.into_u16()?,
            r#special: row.field(8usize + offset)?.into_u16()?,
            r#se_pack: row.field(9usize + offset)?.into_u8()?,
            r#unknown10: row.field(10usize + offset)?.into_bool()?,
            r#array_event_handler: row.field(11usize + offset)?.into_i32()?,
            r#b_npc_parts: row.field(12usize + offset)?.into_u8()?,
            r#unknown13: row.field(13usize + offset)?.into_u8()?,
            r#unknown14: row.field(14usize + offset)?.into_u8()?,
            r#is_target_line: row.field(15usize + offset)?.into_bool()?,
            r#is_display_level: row.field(16usize + offset)?.into_bool()?,
        })
    }
}
