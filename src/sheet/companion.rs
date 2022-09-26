use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for Companion {
    fn name() -> String {
        "Companion".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Companion::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Companion {
    pub r#singular: SeString,
    pub r#adjective: i8,
    pub r#plural: SeString,
    pub r#possessive_pronoun: i8,
    pub r#starts_with_vowel: i8,
    pub r#unknown5: i8,
    pub r#pronoun: i8,
    pub r#article: i8,
    pub r#model: u16,
    pub r#scale: u8,
    pub r#inactive_idle0: u8,
    pub r#inactive_idle1: u8,
    pub r#inactive_battle: u8,
    pub r#inactive_wandering: u8,
    pub r#behavior: u8,
    pub r#special: u8,
    pub r#wandering_wait: u8,
    pub r#priority: u16,
    pub r#unknown18: bool,
    pub r#unknown19: bool,
    pub r#unknown20: bool,
    pub r#unknown21: bool,
    pub r#unknown22: bool,
    pub r#enemy: u16,
    pub r#battle: bool,
    pub r#roulette: bool,
    pub r#icon: u16,
    pub r#order: u16,
    pub r#idle_animation: bool,
    pub r#unknown29: u8,
    pub r#cost: u8,
    pub r#hp: u16,
    pub r#unknown32: u8,
    pub r#skill_angle: u16,
    pub r#skill_cost: u8,
    pub r#unknown35: u8,
    pub r#unknown36: u16,
    pub r#minion_race: u8,
}
impl Companion {
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
            r#model: row.field(8usize + offset)?.into_u16()?,
            r#scale: row.field(9usize + offset)?.into_u8()?,
            r#inactive_idle0: row.field(10usize + offset)?.into_u8()?,
            r#inactive_idle1: row.field(11usize + offset)?.into_u8()?,
            r#inactive_battle: row.field(12usize + offset)?.into_u8()?,
            r#inactive_wandering: row.field(13usize + offset)?.into_u8()?,
            r#behavior: row.field(14usize + offset)?.into_u8()?,
            r#special: row.field(15usize + offset)?.into_u8()?,
            r#wandering_wait: row.field(16usize + offset)?.into_u8()?,
            r#priority: row.field(17usize + offset)?.into_u16()?,
            r#unknown18: row.field(18usize + offset)?.into_bool()?,
            r#unknown19: row.field(19usize + offset)?.into_bool()?,
            r#unknown20: row.field(20usize + offset)?.into_bool()?,
            r#unknown21: row.field(21usize + offset)?.into_bool()?,
            r#unknown22: row.field(22usize + offset)?.into_bool()?,
            r#enemy: row.field(23usize + offset)?.into_u16()?,
            r#battle: row.field(24usize + offset)?.into_bool()?,
            r#roulette: row.field(25usize + offset)?.into_bool()?,
            r#icon: row.field(26usize + offset)?.into_u16()?,
            r#order: row.field(27usize + offset)?.into_u16()?,
            r#idle_animation: row.field(28usize + offset)?.into_bool()?,
            r#unknown29: row.field(29usize + offset)?.into_u8()?,
            r#cost: row.field(30usize + offset)?.into_u8()?,
            r#hp: row.field(31usize + offset)?.into_u16()?,
            r#unknown32: row.field(32usize + offset)?.into_u8()?,
            r#skill_angle: row.field(33usize + offset)?.into_u16()?,
            r#skill_cost: row.field(34usize + offset)?.into_u8()?,
            r#unknown35: row.field(35usize + offset)?.into_u8()?,
            r#unknown36: row.field(36usize + offset)?.into_u16()?,
            r#minion_race: row.field(37usize + offset)?.into_u8()?,
        })
    }
}
