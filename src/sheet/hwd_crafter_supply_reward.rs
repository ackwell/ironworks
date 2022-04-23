use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for HWDCrafterSupplyReward {
    fn name() -> String {
        "HWDCrafterSupplyReward".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDCrafterSupplyReward::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDCrafterSupplyReward {
    pub r#script_reward_amount: u16,
    pub r#exp_reward: u32,
    pub r#points: u16,
}
impl HWDCrafterSupplyReward {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#script_reward_amount: row.field(0usize + offset)?.into_u16()?,
            r#exp_reward: row.field(1usize + offset)?.into_u32()?,
            r#points: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
