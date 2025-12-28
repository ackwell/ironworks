use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for HugeCraftworksRank {
    fn name() -> String {
        "HugeCraftworksRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HugeCraftworksRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HugeCraftworksRank {
    pub r#crafter_level: u8,
    pub r#exp_reward_per_item: u32,
    pub r#unknown2: u8,
}
impl HugeCraftworksRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#crafter_level: row.field(0usize + offset)?.into_u8()?,
            r#exp_reward_per_item: row.field(1usize + offset)?.into_u32()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
