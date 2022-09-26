use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for ParamGrow {
    fn name() -> String {
        "ParamGrow".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ParamGrow::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ParamGrow {
    pub r#exp_to_next: i32,
    pub r#additional_actions: u8,
    pub r#apply_action: u8,
    pub r#scaled_quest_xp: u16,
    pub r#mp_modifier: i32,
    pub r#base_speed: i32,
    pub r#level_modifier: i32,
    pub r#quest_exp_modifier: u8,
    pub r#hp_modifier: u16,
    pub r#hunting_log_exp_reward: i32,
    pub r#monster_note_seals: i32,
    pub r#item_level_sync: u16,
    pub r#proper_dungeon: u16,
    pub r#proper_guild_order: u16,
    pub r#crafting_level: u16,
}
impl ParamGrow {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#exp_to_next: row.field(0usize + offset)?.into_i32()?,
            r#additional_actions: row.field(1usize + offset)?.into_u8()?,
            r#apply_action: row.field(2usize + offset)?.into_u8()?,
            r#scaled_quest_xp: row.field(3usize + offset)?.into_u16()?,
            r#mp_modifier: row.field(4usize + offset)?.into_i32()?,
            r#base_speed: row.field(5usize + offset)?.into_i32()?,
            r#level_modifier: row.field(6usize + offset)?.into_i32()?,
            r#quest_exp_modifier: row.field(7usize + offset)?.into_u8()?,
            r#hp_modifier: row.field(8usize + offset)?.into_u16()?,
            r#hunting_log_exp_reward: row.field(9usize + offset)?.into_i32()?,
            r#monster_note_seals: row.field(10usize + offset)?.into_i32()?,
            r#item_level_sync: row.field(11usize + offset)?.into_u16()?,
            r#proper_dungeon: row.field(12usize + offset)?.into_u16()?,
            r#proper_guild_order: row.field(13usize + offset)?.into_u16()?,
            r#crafting_level: row.field(14usize + offset)?.into_u16()?,
        })
    }
}
