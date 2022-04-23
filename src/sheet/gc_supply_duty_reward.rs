use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for GCSupplyDutyReward {
    fn name() -> String {
        "GCSupplyDutyReward".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GCSupplyDutyReward::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GCSupplyDutyReward {
    pub r#experience_supply: u32,
    pub r#experience_provisioning: u32,
    pub r#seals_expert_delivery: u32,
    pub r#seals_supply: u32,
    pub r#seals_provisioning: u32,
}
impl GCSupplyDutyReward {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#experience_supply: row.field(0usize + offset)?.into_u32()?,
            r#experience_provisioning: row.field(1usize + offset)?.into_u32()?,
            r#seals_expert_delivery: row.field(2usize + offset)?.into_u32()?,
            r#seals_supply: row.field(3usize + offset)?.into_u32()?,
            r#seals_provisioning: row.field(4usize + offset)?.into_u32()?,
        })
    }
}
