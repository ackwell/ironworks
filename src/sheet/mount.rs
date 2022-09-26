use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for Mount {
    fn name() -> String {
        "Mount".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Mount::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Mount {
    pub r#singular: SeString,
    pub r#adjective: i8,
    pub r#plural: SeString,
    pub r#possessive_pronoun: i8,
    pub r#starts_with_vowel: i8,
    pub r#unknown5: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
    pub r#model_chara: i32,
    pub r#unknown9: u16,
    pub r#flying_condition: u8,
    pub r#unknown11: u8,
    pub r#unknown12: u8,
    pub r#unknown13: u8,
    pub r#is_flying: u8,
    pub r#unknown15: u8,
    pub r#mount_customize: u8,
    pub r#ride_bgm: u16,
    pub r#unknown18: SeString,
    pub r#unknown19: SeString,
    pub r#unknown20: SeString,
    pub r#exit_move_dist: u8,
    pub r#exit_move_speed: u8,
    pub r#unknown23: bool,
    pub r#is_emote: bool,
    pub r#equip_head: i32,
    pub r#equip_body: i32,
    pub r#equip_leg: i32,
    pub r#equip_foot: i32,
    pub r#order: i16,
    pub r#icon: u16,
    pub r#ui_priority: u8,
    pub r#radius_rate: u8,
    pub r#base_motion_speed_run: u8,
    pub r#base_motion_speed_walk: u8,
    pub r#unknown35: u8,
    pub r#extra_seats: u8,
    pub r#mount_action: u16,
    pub r#is_airborne: bool,
    pub r#ex_hotbar_enable_config: bool,
    pub r#use_ep: bool,
    pub r#unknown41: bool,
    pub r#is_immobile: bool,
}
impl Mount {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#singular: row.field(0usize + offset)?.into_string()?,
            r#adjective: row.field(1usize + offset)?.into_i8()?,
            r#plural: row.field(2usize + offset)?.into_string()?,
            r#possessive_pronoun: row.field(3usize + offset)?.into_i8()?,
            r#starts_with_vowel: row.field(4usize + offset)?.into_i8()?,
            r#unknown5: row.field(5usize + offset)?.into_i8()?,
            r#pronoun: row.field(6usize + offset)?.into_i8()?,
            r#article: row.field(7usize + offset)?.into_i8()?,
            r#model_chara: row.field(8usize + offset)?.into_i32()?,
            r#unknown9: row.field(9usize + offset)?.into_u16()?,
            r#flying_condition: row.field(10usize + offset)?.into_u8()?,
            r#unknown11: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
            r#unknown13: row.field(13usize + offset)?.into_u8()?,
            r#is_flying: row.field(14usize + offset)?.into_u8()?,
            r#unknown15: row.field(15usize + offset)?.into_u8()?,
            r#mount_customize: row.field(16usize + offset)?.into_u8()?,
            r#ride_bgm: row.field(17usize + offset)?.into_u16()?,
            r#unknown18: row.field(18usize + offset)?.into_string()?,
            r#unknown19: row.field(19usize + offset)?.into_string()?,
            r#unknown20: row.field(20usize + offset)?.into_string()?,
            r#exit_move_dist: row.field(21usize + offset)?.into_u8()?,
            r#exit_move_speed: row.field(22usize + offset)?.into_u8()?,
            r#unknown23: row.field(23usize + offset)?.into_bool()?,
            r#is_emote: row.field(24usize + offset)?.into_bool()?,
            r#equip_head: row.field(25usize + offset)?.into_i32()?,
            r#equip_body: row.field(26usize + offset)?.into_i32()?,
            r#equip_leg: row.field(27usize + offset)?.into_i32()?,
            r#equip_foot: row.field(28usize + offset)?.into_i32()?,
            r#order: row.field(29usize + offset)?.into_i16()?,
            r#icon: row.field(30usize + offset)?.into_u16()?,
            r#ui_priority: row.field(31usize + offset)?.into_u8()?,
            r#radius_rate: row.field(32usize + offset)?.into_u8()?,
            r#base_motion_speed_run: row.field(33usize + offset)?.into_u8()?,
            r#base_motion_speed_walk: row.field(34usize + offset)?.into_u8()?,
            r#unknown35: row.field(35usize + offset)?.into_u8()?,
            r#extra_seats: row.field(36usize + offset)?.into_u8()?,
            r#mount_action: row.field(37usize + offset)?.into_u16()?,
            r#is_airborne: row.field(38usize + offset)?.into_bool()?,
            r#ex_hotbar_enable_config: row.field(39usize + offset)?.into_bool()?,
            r#use_ep: row.field(40usize + offset)?.into_bool()?,
            r#unknown41: row.field(41usize + offset)?.into_bool()?,
            r#is_immobile: row.field(42usize + offset)?.into_bool()?,
        })
    }
}
