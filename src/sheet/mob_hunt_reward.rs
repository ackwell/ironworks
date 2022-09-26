use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MobHuntReward {
    fn name() -> String {
        "MobHuntReward".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MobHuntReward::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MobHuntReward {
    pub r#exp_reward: u32,
    pub r#gil_reward: u16,
    pub r#expansion: u8,
    pub r#currency_reward: u16,
}
impl MobHuntReward {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#exp_reward: row.field(0usize + offset)?.into_u32()?,
            r#gil_reward: row.field(1usize + offset)?.into_u16()?,
            r#expansion: row.field(2usize + offset)?.into_u8()?,
            r#currency_reward: row.field(3usize + offset)?.into_u16()?,
        })
    }
}
