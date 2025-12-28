use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for CollectablesShopRewardScrip {
    fn name() -> String {
        "CollectablesShopRewardScrip".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CollectablesShopRewardScrip::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CollectablesShopRewardScrip {
    pub r#currency: u16,
    pub r#low_reward: u16,
    pub r#mid_reward: u16,
    pub r#high_reward: u16,
    pub r#exp_ratio_low: u16,
    pub r#exp_ratio_mid: u16,
    pub r#exp_ratio_high: u16,
}
impl CollectablesShopRewardScrip {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#currency: row.field(0usize + offset)?.into_u16()?,
            r#low_reward: row.field(1usize + offset)?.into_u16()?,
            r#mid_reward: row.field(2usize + offset)?.into_u16()?,
            r#high_reward: row.field(3usize + offset)?.into_u16()?,
            r#exp_ratio_low: row.field(4usize + offset)?.into_u16()?,
            r#exp_ratio_mid: row.field(5usize + offset)?.into_u16()?,
            r#exp_ratio_high: row.field(6usize + offset)?.into_u16()?,
        })
    }
}
