use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MobHuntRewardCap {
    fn name() -> String {
        "MobHuntRewardCap".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MobHuntRewardCap::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MobHuntRewardCap {
    pub r#exp_cap: u32,
}
impl MobHuntRewardCap {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#exp_cap: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
