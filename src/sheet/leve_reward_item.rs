use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for LeveRewardItem {
    fn name() -> String {
        "LeveRewardItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(LeveRewardItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct LeveRewardItem_ro {
    pub r#leve_reward_item_group: u16,
    pub r#probability_percent: u8,
}
impl LeveRewardItem_ro {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#leve_reward_item_group: row.field(0usize + offset)?.into_u16()?,
            r#probability_percent: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct LeveRewardItem {
    pub r#ro: Vec<LeveRewardItem_ro>,
}
impl LeveRewardItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#ro: read_array(
                offset,
                8usize,
                2usize,
                |offset| { Result::Ok(LeveRewardItem_ro::populate(row, offset)?) },
            )?,
        })
    }
}
