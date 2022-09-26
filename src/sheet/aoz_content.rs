use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for AOZContent {
    fn name() -> String {
        "AOZContent".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AOZContent::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AOZContent {
    pub r#standard_finish_time: u16,
    pub r#ideal_finish_time: u16,
    pub r#act1_fight_type: u8,
    pub r#act1: u16,
    pub r#arena_type1: u8,
    pub r#act2_fight_type: u8,
    pub r#act2: u16,
    pub r#arena_type2: u8,
    pub r#act3_fight_type: u8,
    pub r#act3: u16,
    pub r#arena_type3: u8,
    pub r#content_entry: u32,
    pub r#order: u8,
    pub r#gil_reward: u16,
    pub r#allied_seals_reward: u16,
    pub r#tomestones_reward: u16,
}
impl AOZContent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#standard_finish_time: row.field(0usize + offset)?.into_u16()?,
            r#ideal_finish_time: row.field(1usize + offset)?.into_u16()?,
            r#act1_fight_type: row.field(2usize + offset)?.into_u8()?,
            r#act1: row.field(3usize + offset)?.into_u16()?,
            r#arena_type1: row.field(4usize + offset)?.into_u8()?,
            r#act2_fight_type: row.field(5usize + offset)?.into_u8()?,
            r#act2: row.field(6usize + offset)?.into_u16()?,
            r#arena_type2: row.field(7usize + offset)?.into_u8()?,
            r#act3_fight_type: row.field(8usize + offset)?.into_u8()?,
            r#act3: row.field(9usize + offset)?.into_u16()?,
            r#arena_type3: row.field(10usize + offset)?.into_u8()?,
            r#content_entry: row.field(11usize + offset)?.into_u32()?,
            r#order: row.field(12usize + offset)?.into_u8()?,
            r#gil_reward: row.field(13usize + offset)?.into_u16()?,
            r#allied_seals_reward: row.field(14usize + offset)?.into_u16()?,
            r#tomestones_reward: row.field(15usize + offset)?.into_u16()?,
        })
    }
}
