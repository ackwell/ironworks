use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ContentRoulette {
    fn name() -> String {
        "ContentRoulette".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentRoulette::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentRoulette {
    pub r#name: SeString,
    pub r#category: SeString,
    pub r#unknown2: SeString,
    pub r#description: SeString,
    pub r#duty_type: SeString,
    pub r#unknown5: u8,
    pub r#unknown6: u32,
    pub r#is_gold_saucer: bool,
    pub r#is_in_duty_finder: bool,
    pub r#open_rule: u8,
    pub r#is_pv_p: bool,
    pub r#required_level: u8,
    pub r#unknown12: u8,
    pub r#item_level_required: u16,
    pub r#unknown14: u16,
    pub r#icon: u32,
    pub r#content_roulette_role_bonus: u8,
    pub r#reward_tome_a: u16,
    pub r#reward_tome_b: u16,
    pub r#reward_tome_c: u16,
    pub r#unknown20: u32,
    pub r#unknown21: u16,
    pub r#sort_key: u8,
    pub r#unknown23: u8,
    pub r#content_member_type: u8,
    pub r#unknown25: u8,
    pub r#unknown26: u8,
    pub r#unknown27: i8,
    pub r#unknown28: u8,
    pub r#unknown29: u8,
    pub r#unknown30: u8,
    pub r#unknown31: u8,
    pub r#unknown32: bool,
    pub r#unknown33: bool,
    pub r#require_all_duties: bool,
    pub r#unknown35: bool,
    pub r#content_roulette_open_rule: u8,
    pub r#instance_content: u16,
}
impl ContentRoulette {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#category: row.field(1usize + offset)?.into_string()?,
            r#unknown2: row.field(2usize + offset)?.into_string()?,
            r#description: row.field(3usize + offset)?.into_string()?,
            r#duty_type: row.field(4usize + offset)?.into_string()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#is_gold_saucer: row.field(7usize + offset)?.into_bool()?,
            r#is_in_duty_finder: row.field(8usize + offset)?.into_bool()?,
            r#open_rule: row.field(9usize + offset)?.into_u8()?,
            r#is_pv_p: row.field(10usize + offset)?.into_bool()?,
            r#required_level: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u8()?,
            r#item_level_required: row.field(13usize + offset)?.into_u16()?,
            r#unknown14: row.field(14usize + offset)?.into_u16()?,
            r#icon: row.field(15usize + offset)?.into_u32()?,
            r#content_roulette_role_bonus: row.field(16usize + offset)?.into_u8()?,
            r#reward_tome_a: row.field(17usize + offset)?.into_u16()?,
            r#reward_tome_b: row.field(18usize + offset)?.into_u16()?,
            r#reward_tome_c: row.field(19usize + offset)?.into_u16()?,
            r#unknown20: row.field(20usize + offset)?.into_u32()?,
            r#unknown21: row.field(21usize + offset)?.into_u16()?,
            r#sort_key: row.field(22usize + offset)?.into_u8()?,
            r#unknown23: row.field(23usize + offset)?.into_u8()?,
            r#content_member_type: row.field(24usize + offset)?.into_u8()?,
            r#unknown25: row.field(25usize + offset)?.into_u8()?,
            r#unknown26: row.field(26usize + offset)?.into_u8()?,
            r#unknown27: row.field(27usize + offset)?.into_i8()?,
            r#unknown28: row.field(28usize + offset)?.into_u8()?,
            r#unknown29: row.field(29usize + offset)?.into_u8()?,
            r#unknown30: row.field(30usize + offset)?.into_u8()?,
            r#unknown31: row.field(31usize + offset)?.into_u8()?,
            r#unknown32: row.field(32usize + offset)?.into_bool()?,
            r#unknown33: row.field(33usize + offset)?.into_bool()?,
            r#require_all_duties: row.field(34usize + offset)?.into_bool()?,
            r#unknown35: row.field(35usize + offset)?.into_bool()?,
            r#content_roulette_open_rule: row.field(36usize + offset)?.into_u8()?,
            r#instance_content: row.field(37usize + offset)?.into_u16()?,
        })
    }
}
