use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
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
    pub r#unknown9: u32,
    pub r#unknown10: u32,
    pub r#unknown11: u32,
    pub r#unknown12: u32,
    pub r#unknown13: u16,
    pub r#unknown14: u16,
    pub r#unknown15: bool,
    pub r#unknown16: bool,
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
            r#unknown9: row.field(9usize + offset)?.into_u32()?,
            r#unknown10: row.field(10usize + offset)?.into_u32()?,
            r#unknown11: row.field(11usize + offset)?.into_u32()?,
            r#unknown12: row.field(12usize + offset)?.into_u32()?,
            r#unknown13: row.field(13usize + offset)?.into_u16()?,
            r#unknown14: row.field(14usize + offset)?.into_u16()?,
            r#unknown15: row.field(15usize + offset)?.into_bool()?,
            r#unknown16: row.field(16usize + offset)?.into_bool()?,
        })
    }
}
