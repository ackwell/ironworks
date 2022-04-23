use std::result::Result;
use crate::utility::read_array;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
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
    pub r#icon: i32,
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
            r#icon: row.field(70usize + offset)?.into_i32()?,
        })
    }
}
