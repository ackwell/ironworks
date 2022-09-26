use crate::utility::read_array;
use std::vec::Vec;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for CustomTalk {
    fn name() -> String {
        "CustomTalk".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CustomTalk::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CustomTalk {
    pub r#icon_actor: u32,
    pub r#icon_map: u32,
    pub r#name: SeString,
    pub r#script_instruction: Vec<SeString>,
    pub r#script_arg: Vec<u32>,
    pub r#unknown63: bool,
    pub r#main_option: SeString,
    pub r#sub_option: SeString,
    pub r#unknown66: bool,
    pub r#unknown67: bool,
    pub r#unknown68: bool,
    pub r#unknown69: bool,
    pub r#unknown70: bool,
    pub r#unknown71: bool,
    pub r#unknown72: bool,
    pub r#unknown73: bool,
    pub r#unknown74: bool,
    pub r#special_links: u32,
}
impl CustomTalk {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon_actor: row.field(0usize + offset)?.into_u32()?,
            r#icon_map: row.field(1usize + offset)?.into_u32()?,
            r#name: row.field(2usize + offset)?.into_string()?,
            r#script_instruction: read_array(
                offset,
                30usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_string()?) },
            )?,
            r#script_arg: read_array(
                offset,
                30usize,
                1usize,
                |offset| { Result::Ok(row.field(33usize + offset)?.into_u32()?) },
            )?,
            r#unknown63: row.field(63usize + offset)?.into_bool()?,
            r#main_option: row.field(64usize + offset)?.into_string()?,
            r#sub_option: row.field(65usize + offset)?.into_string()?,
            r#unknown66: row.field(66usize + offset)?.into_bool()?,
            r#unknown67: row.field(67usize + offset)?.into_bool()?,
            r#unknown68: row.field(68usize + offset)?.into_bool()?,
            r#unknown69: row.field(69usize + offset)?.into_bool()?,
            r#unknown70: row.field(70usize + offset)?.into_bool()?,
            r#unknown71: row.field(71usize + offset)?.into_bool()?,
            r#unknown72: row.field(72usize + offset)?.into_bool()?,
            r#unknown73: row.field(73usize + offset)?.into_bool()?,
            r#unknown74: row.field(74usize + offset)?.into_bool()?,
            r#special_links: row.field(75usize + offset)?.into_u32()?,
        })
    }
}
