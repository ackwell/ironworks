use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for ContentRouletteRoleBonus {
    fn name() -> String {
        "ContentRouletteRoleBonus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentRouletteRoleBonus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentRouletteRoleBonus {
    pub r#unknown0: u16,
    pub r#unknown1: u16,
    pub r#unknown2: u16,
    pub r#unknown3: u16,
    pub r#unknown4: u16,
    pub r#unknown5: u16,
    pub r#item_reward_type: u32,
    pub r#reward_amount: u8,
}
impl ContentRouletteRoleBonus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_u16()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u16()?,
            r#item_reward_type: row.field(6usize + offset)?.into_u32()?,
            r#reward_amount: row.field(7usize + offset)?.into_u8()?,
        })
    }
}
