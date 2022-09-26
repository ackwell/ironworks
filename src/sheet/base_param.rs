use std::result::Result;
use std::vec::Vec;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for BaseParam {
    fn name() -> String {
        "BaseParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BaseParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BaseParam {
    pub r#packet_index: i8,
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#order_priority: u8,
    pub r#one_h_wpn_percent: u16,
    pub r#oh_percent: u16,
    pub r#head_percent: u16,
    pub r#chest_percent: u16,
    pub r#hands_percent: u16,
    pub r#waist_percent: u16,
    pub r#legs_percent: u16,
    pub r#feet_percent: u16,
    pub r#earring_percent: u16,
    pub r#necklace_percent: u16,
    pub r#bracelet_percent: u16,
    pub r#ring_percent: u16,
    pub r#two_h_wpn_percent: u16,
    pub r#under_armor_percent: u16,
    pub r#chest_head_percent: u16,
    pub r#chest_head_legs_feet_percent: u16,
    pub r#unknown20: u16,
    pub r#legs_feet_percent: u16,
    pub r#head_chest_hands_legs_feet_percent: u16,
    pub r#chest_legs_gloves_percent: u16,
    pub r#chest_legs_feet_percent: u16,
    pub r#meld_param: Vec<u16>,
}
impl BaseParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#packet_index: row.field(0usize + offset)?.into_i8()?,
            r#name: row.field(1usize + offset)?.into_string()?,
            r#description: row.field(2usize + offset)?.into_string()?,
            r#order_priority: row.field(3usize + offset)?.into_u8()?,
            r#one_h_wpn_percent: row.field(4usize + offset)?.into_u16()?,
            r#oh_percent: row.field(5usize + offset)?.into_u16()?,
            r#head_percent: row.field(6usize + offset)?.into_u16()?,
            r#chest_percent: row.field(7usize + offset)?.into_u16()?,
            r#hands_percent: row.field(8usize + offset)?.into_u16()?,
            r#waist_percent: row.field(9usize + offset)?.into_u16()?,
            r#legs_percent: row.field(10usize + offset)?.into_u16()?,
            r#feet_percent: row.field(11usize + offset)?.into_u16()?,
            r#earring_percent: row.field(12usize + offset)?.into_u16()?,
            r#necklace_percent: row.field(13usize + offset)?.into_u16()?,
            r#bracelet_percent: row.field(14usize + offset)?.into_u16()?,
            r#ring_percent: row.field(15usize + offset)?.into_u16()?,
            r#two_h_wpn_percent: row.field(16usize + offset)?.into_u16()?,
            r#under_armor_percent: row.field(17usize + offset)?.into_u16()?,
            r#chest_head_percent: row.field(18usize + offset)?.into_u16()?,
            r#chest_head_legs_feet_percent: row.field(19usize + offset)?.into_u16()?,
            r#unknown20: row.field(20usize + offset)?.into_u16()?,
            r#legs_feet_percent: row.field(21usize + offset)?.into_u16()?,
            r#head_chest_hands_legs_feet_percent: row
                .field(22usize + offset)?
                .into_u16()?,
            r#chest_legs_gloves_percent: row.field(23usize + offset)?.into_u16()?,
            r#chest_legs_feet_percent: row.field(24usize + offset)?.into_u16()?,
            r#meld_param: read_array(
                offset,
                13usize,
                1usize,
                |offset| { Result::Ok(row.field(25usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
