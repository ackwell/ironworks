use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for CollectablesShopRewardItem {
    fn name() -> String {
        "CollectablesShopRewardItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CollectablesShopRewardItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CollectablesShopRewardItem {
    pub r#item: u32,
    pub r#unknown1: bool,
    pub r#reward_low: u8,
    pub r#reward_mid: u8,
    pub r#reward_high: u8,
}
impl CollectablesShopRewardItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
            r#reward_low: row.field(2usize + offset)?.into_u8()?,
            r#reward_mid: row.field(3usize + offset)?.into_u8()?,
            r#reward_high: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
