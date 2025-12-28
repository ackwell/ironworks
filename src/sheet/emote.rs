use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for Emote {
    fn name() -> String {
        "Emote".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Emote::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Emote {
    pub r#name: SeString,
    pub r#action_timeline: Vec<u16>,
    pub r#unknown8: bool,
    pub r#unknown9: bool,
    pub r#unknown10: bool,
    pub r#emote_category: u8,
    pub r#emote_mode: u8,
    pub r#unknown13: bool,
    pub r#unknown14: bool,
    pub r#has_cancel_emote: bool,
    pub r#draws_weapon: bool,
    pub r#unknown17: bool,
    pub r#order: u16,
    pub r#text_command: i32,
    pub r#icon: u32,
    pub r#log_message_targeted: u16,
    pub r#log_message_untargeted: u16,
    pub r#unlock_link: u32,
    pub r#unknown24: u16,
}
impl Emote {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#action_timeline: read_array(
                offset,
                7usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u16()?) },
            )?,
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
            r#unknown10: row.field(10usize + offset)?.into_bool()?,
            r#emote_category: row.field(11usize + offset)?.into_u8()?,
            r#emote_mode: row.field(12usize + offset)?.into_u8()?,
            r#unknown13: row.field(13usize + offset)?.into_bool()?,
            r#unknown14: row.field(14usize + offset)?.into_bool()?,
            r#has_cancel_emote: row.field(15usize + offset)?.into_bool()?,
            r#draws_weapon: row.field(16usize + offset)?.into_bool()?,
            r#unknown17: row.field(17usize + offset)?.into_bool()?,
            r#order: row.field(18usize + offset)?.into_u16()?,
            r#text_command: row.field(19usize + offset)?.into_i32()?,
            r#icon: row.field(20usize + offset)?.into_u32()?,
            r#log_message_targeted: row.field(21usize + offset)?.into_u16()?,
            r#log_message_untargeted: row.field(22usize + offset)?.into_u16()?,
            r#unlock_link: row.field(23usize + offset)?.into_u32()?,
            r#unknown24: row.field(24usize + offset)?.into_u16()?,
        })
    }
}
