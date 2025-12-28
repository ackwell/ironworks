use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for SatisfactionNpc {
    fn name() -> String {
        "SatisfactionNpc".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SatisfactionNpc::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SatisfactionNpc_I {
    pub r#item: Vec<i32>,
    pub r#item_count: Vec<u8>,
    pub r#is_hq: Vec<bool>,
}
impl SatisfactionNpc_I {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(16usize + offset)?.into_i32()?) },
            )?,
            r#item_count: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(22usize + offset)?.into_u8()?) },
            )?,
            r#is_hq: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(28usize + offset)?.into_bool()?) },
            )?,
        })
    }
}
#[derive(Debug)]
pub struct SatisfactionNpc {
    pub r#npc: i32,
    pub r#quest_required: i32,
    pub r#level_unlock: u8,
    pub r#deliveries_per_week: u8,
    pub r#supply_index: Vec<i32>,
    pub r#satisfaction_required: Vec<u16>,
    pub r#i: Vec<SatisfactionNpc_I>,
    pub r#unknown70: i32,
    pub r#unknown71: i32,
    pub r#unknown72: i32,
    pub r#unknown73: i32,
    pub r#unknown74: i32,
    pub r#unknown75: i32,
    pub r#unknown76: i32,
    pub r#unknown77: i32,
    pub r#unknown78: i32,
    pub r#unknown79: i32,
    pub r#unknown80: i32,
    pub r#unknown81: i32,
    pub r#unknown82: i32,
    pub r#unknown83: i32,
    pub r#unknown84: i32,
    pub r#unknown85: i32,
    pub r#unknown86: i32,
    pub r#unknown87: i32,
    pub r#icon: u32,
    pub r#unknown89: i32,
    pub r#unknown90: u8,
    pub r#unknown91: u8,
    pub r#unknown92: u8,
}
impl SatisfactionNpc {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#npc: row.field(0usize + offset)?.into_i32()?,
            r#quest_required: row.field(1usize + offset)?.into_i32()?,
            r#level_unlock: row.field(2usize + offset)?.into_u8()?,
            r#deliveries_per_week: row.field(3usize + offset)?.into_u8()?,
            r#supply_index: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(4usize + offset)?.into_i32()?) },
            )?,
            r#satisfaction_required: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(10usize + offset)?.into_u16()?) },
            )?,
            r#i: read_array(
                offset,
                3usize,
                18usize,
                |offset| { Result::Ok(SatisfactionNpc_I::populate(row, offset)?) },
            )?,
            r#unknown70: row.field(70usize + offset)?.into_i32()?,
            r#unknown71: row.field(71usize + offset)?.into_i32()?,
            r#unknown72: row.field(72usize + offset)?.into_i32()?,
            r#unknown73: row.field(73usize + offset)?.into_i32()?,
            r#unknown74: row.field(74usize + offset)?.into_i32()?,
            r#unknown75: row.field(75usize + offset)?.into_i32()?,
            r#unknown76: row.field(76usize + offset)?.into_i32()?,
            r#unknown77: row.field(77usize + offset)?.into_i32()?,
            r#unknown78: row.field(78usize + offset)?.into_i32()?,
            r#unknown79: row.field(79usize + offset)?.into_i32()?,
            r#unknown80: row.field(80usize + offset)?.into_i32()?,
            r#unknown81: row.field(81usize + offset)?.into_i32()?,
            r#unknown82: row.field(82usize + offset)?.into_i32()?,
            r#unknown83: row.field(83usize + offset)?.into_i32()?,
            r#unknown84: row.field(84usize + offset)?.into_i32()?,
            r#unknown85: row.field(85usize + offset)?.into_i32()?,
            r#unknown86: row.field(86usize + offset)?.into_i32()?,
            r#unknown87: row.field(87usize + offset)?.into_i32()?,
            r#icon: row.field(88usize + offset)?.into_u32()?,
            r#unknown89: row.field(89usize + offset)?.into_i32()?,
            r#unknown90: row.field(90usize + offset)?.into_u8()?,
            r#unknown91: row.field(91usize + offset)?.into_u8()?,
            r#unknown92: row.field(92usize + offset)?.into_u8()?,
        })
    }
}
