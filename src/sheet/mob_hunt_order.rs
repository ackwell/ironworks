use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MobHuntOrder {
    fn name() -> String {
        "MobHuntOrder".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MobHuntOrder::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MobHuntOrder {
    pub r#target: u16,
    pub r#needed_kills: u8,
    pub r#type: u8,
    pub r#rank: u8,
    pub r#mob_hunt_reward: u8,
}
impl MobHuntOrder {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#target: row.field(0usize + offset)?.into_u16()?,
            r#needed_kills: row.field(1usize + offset)?.into_u8()?,
            r#type: row.field(2usize + offset)?.into_u8()?,
            r#rank: row.field(3usize + offset)?.into_u8()?,
            r#mob_hunt_reward: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
