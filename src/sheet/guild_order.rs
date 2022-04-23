use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for GuildOrder {
    fn name() -> String {
        "GuildOrder".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GuildOrder::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GuildOrder {
    pub r#e_npc_name: u32,
    pub r#objective: SeString,
    pub r#description1: SeString,
    pub r#description2: SeString,
    pub r#description3: SeString,
    pub r#completion_bonus_exp: u32,
    pub r#reward_exp: u32,
    pub r#completion_bonus_gil: u32,
    pub r#reward_gil: u32,
}
impl GuildOrder {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#e_npc_name: row.field(0usize + offset)?.into_u32()?,
            r#objective: row.field(1usize + offset)?.into_string()?,
            r#description1: row.field(2usize + offset)?.into_string()?,
            r#description2: row.field(3usize + offset)?.into_string()?,
            r#description3: row.field(4usize + offset)?.into_string()?,
            r#completion_bonus_exp: row.field(5usize + offset)?.into_u32()?,
            r#reward_exp: row.field(6usize + offset)?.into_u32()?,
            r#completion_bonus_gil: row.field(7usize + offset)?.into_u32()?,
            r#reward_gil: row.field(8usize + offset)?.into_u32()?,
        })
    }
}
