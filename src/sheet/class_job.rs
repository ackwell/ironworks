use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
impl MetadataAdapter for ClassJob {
    fn name() -> String {
        "ClassJob".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ClassJob::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ClassJob {
    pub r#name: SeString,
    pub r#abbreviation: SeString,
    pub r#unknown2: SeString,
    pub r#class_job_category: u8,
    pub r#exp_array_index: i8,
    pub r#battle_class_index: i8,
    pub r#unknown6: u8,
    pub r#job_index: u8,
    pub r#doh_dol_job_index: i8,
    pub r#modifier_hit_points: u16,
    pub r#modifier_mana_points: u16,
    pub r#modifier_strength: u16,
    pub r#modifier_vitality: u16,
    pub r#modifier_dexterity: u16,
    pub r#modifier_intelligence: u16,
    pub r#modifier_mind: u16,
    pub r#modifier_piety: u16,
    pub r#unknown17: u16,
    pub r#unknown18: u16,
    pub r#unknown19: u16,
    pub r#unknown20: u16,
    pub r#unknown21: u16,
    pub r#unknown22: u16,
    pub r#unknown23: u8,
    pub r#pv_p_action_sort_row: u8,
    pub r#unknown25: u8,
    pub r#class_job_parent: u8,
    pub r#name_english: SeString,
    pub r#item_starting_weapon: i32,
    pub r#unknown29: i32,
    pub r#role: u8,
    pub r#starting_town: u8,
    pub r#monster_note: i8,
    pub r#primary_stat: u8,
    pub r#limit_break1: u16,
    pub r#limit_break2: u16,
    pub r#limit_break3: u16,
    pub r#ui_priority: u8,
    pub r#item_soul_crystal: u32,
    pub r#unlock_quest: u32,
    pub r#relic_quest: u32,
    pub r#prerequisite: u32,
    pub r#starting_level: u8,
    pub r#party_bonus: u8,
    pub r#unknown44: u8,
    pub r#is_limited_job: bool,
    pub r#can_queue_for_duty: bool,
}
impl ClassJob {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#abbreviation: row.field(1usize + offset)?.into_string()?,
            r#unknown2: row.field(2usize + offset)?.into_string()?,
            r#class_job_category: row.field(3usize + offset)?.into_u8()?,
            r#exp_array_index: row.field(4usize + offset)?.into_i8()?,
            r#battle_class_index: row.field(5usize + offset)?.into_i8()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#job_index: row.field(7usize + offset)?.into_u8()?,
            r#doh_dol_job_index: row.field(8usize + offset)?.into_i8()?,
            r#modifier_hit_points: row.field(9usize + offset)?.into_u16()?,
            r#modifier_mana_points: row.field(10usize + offset)?.into_u16()?,
            r#modifier_strength: row.field(11usize + offset)?.into_u16()?,
            r#modifier_vitality: row.field(12usize + offset)?.into_u16()?,
            r#modifier_dexterity: row.field(13usize + offset)?.into_u16()?,
            r#modifier_intelligence: row.field(14usize + offset)?.into_u16()?,
            r#modifier_mind: row.field(15usize + offset)?.into_u16()?,
            r#modifier_piety: row.field(16usize + offset)?.into_u16()?,
            r#unknown17: row.field(17usize + offset)?.into_u16()?,
            r#unknown18: row.field(18usize + offset)?.into_u16()?,
            r#unknown19: row.field(19usize + offset)?.into_u16()?,
            r#unknown20: row.field(20usize + offset)?.into_u16()?,
            r#unknown21: row.field(21usize + offset)?.into_u16()?,
            r#unknown22: row.field(22usize + offset)?.into_u16()?,
            r#unknown23: row.field(23usize + offset)?.into_u8()?,
            r#pv_p_action_sort_row: row.field(24usize + offset)?.into_u8()?,
            r#unknown25: row.field(25usize + offset)?.into_u8()?,
            r#class_job_parent: row.field(26usize + offset)?.into_u8()?,
            r#name_english: row.field(27usize + offset)?.into_string()?,
            r#item_starting_weapon: row.field(28usize + offset)?.into_i32()?,
            r#unknown29: row.field(29usize + offset)?.into_i32()?,
            r#role: row.field(30usize + offset)?.into_u8()?,
            r#starting_town: row.field(31usize + offset)?.into_u8()?,
            r#monster_note: row.field(32usize + offset)?.into_i8()?,
            r#primary_stat: row.field(33usize + offset)?.into_u8()?,
            r#limit_break1: row.field(34usize + offset)?.into_u16()?,
            r#limit_break2: row.field(35usize + offset)?.into_u16()?,
            r#limit_break3: row.field(36usize + offset)?.into_u16()?,
            r#ui_priority: row.field(37usize + offset)?.into_u8()?,
            r#item_soul_crystal: row.field(38usize + offset)?.into_u32()?,
            r#unlock_quest: row.field(39usize + offset)?.into_u32()?,
            r#relic_quest: row.field(40usize + offset)?.into_u32()?,
            r#prerequisite: row.field(41usize + offset)?.into_u32()?,
            r#starting_level: row.field(42usize + offset)?.into_u8()?,
            r#party_bonus: row.field(43usize + offset)?.into_u8()?,
            r#unknown44: row.field(44usize + offset)?.into_u8()?,
            r#is_limited_job: row.field(45usize + offset)?.into_bool()?,
            r#can_queue_for_duty: row.field(46usize + offset)?.into_bool()?,
        })
    }
}
