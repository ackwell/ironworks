use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for Status {
    fn name() -> String {
        "Status".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Status::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Status {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#icon: u16,
    pub r#unknown3: u8,
    pub r#max_stacks: u8,
    pub r#unknown5: u8,
    pub r#category: u8,
    pub r#hit_effect: u8,
    pub r#vfx: u16,
    pub r#lock_movement: bool,
    pub r#unknown10: bool,
    pub r#lock_actions: bool,
    pub r#lock_control: bool,
    pub r#transfiguration: bool,
    pub r#unknown14: bool,
    pub r#can_dispel: bool,
    pub r#inflicted_by_actor: bool,
    pub r#is_permanent: bool,
    pub r#party_list_priority: u8,
    pub r#unknown19: u8,
    pub r#unknown20: bool,
    pub r#unknown21: bool,
    pub r#unknown22: i16,
    pub r#unknown23: u8,
    pub r#unknown24: bool,
    pub r#log: u16,
    pub r#is_fc_buff: bool,
    pub r#invisibility: bool,
}
impl Status {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#icon: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#max_stacks: row.field(4usize + offset)?.into_u8()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#category: row.field(6usize + offset)?.into_u8()?,
            r#hit_effect: row.field(7usize + offset)?.into_u8()?,
            r#vfx: row.field(8usize + offset)?.into_u16()?,
            r#lock_movement: row.field(9usize + offset)?.into_bool()?,
            r#unknown10: row.field(10usize + offset)?.into_bool()?,
            r#lock_actions: row.field(11usize + offset)?.into_bool()?,
            r#lock_control: row.field(12usize + offset)?.into_bool()?,
            r#transfiguration: row.field(13usize + offset)?.into_bool()?,
            r#unknown14: row.field(14usize + offset)?.into_bool()?,
            r#can_dispel: row.field(15usize + offset)?.into_bool()?,
            r#inflicted_by_actor: row.field(16usize + offset)?.into_bool()?,
            r#is_permanent: row.field(17usize + offset)?.into_bool()?,
            r#party_list_priority: row.field(18usize + offset)?.into_u8()?,
            r#unknown19: row.field(19usize + offset)?.into_u8()?,
            r#unknown20: row.field(20usize + offset)?.into_bool()?,
            r#unknown21: row.field(21usize + offset)?.into_bool()?,
            r#unknown22: row.field(22usize + offset)?.into_i16()?,
            r#unknown23: row.field(23usize + offset)?.into_u8()?,
            r#unknown24: row.field(24usize + offset)?.into_bool()?,
            r#log: row.field(25usize + offset)?.into_u16()?,
            r#is_fc_buff: row.field(26usize + offset)?.into_bool()?,
            r#invisibility: row.field(27usize + offset)?.into_bool()?,
        })
    }
}
