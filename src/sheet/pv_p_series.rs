use std::result::Result;
use crate::error::PopulateError;
use std::vec::Vec;
use ironworks::excel::Row;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for PvPSeries {
    fn name() -> String {
        "PvPSeries".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PvPSeries::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PvPSeries_LevelRewardItem {
    pub r#level_reward_item: Vec<i32>,
}
impl PvPSeries_LevelRewardItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#level_reward_item: read_array(
                offset,
                32usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_i32()?) },
            )?,
        })
    }
}
#[derive(Debug)]
pub struct PvPSeries_LevelRewardCount {
    pub r#level_reward_count: Vec<u16>,
}
impl PvPSeries_LevelRewardCount {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#level_reward_count: read_array(
                offset,
                32usize,
                1usize,
                |offset| { Result::Ok(row.field(65usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
#[derive(Debug)]
pub struct PvPSeries {
    pub r#unknown0: u8,
    pub r#level_reward_item: Vec<PvPSeries_LevelRewardItem>,
    pub r#level_reward_count: Vec<PvPSeries_LevelRewardCount>,
}
impl PvPSeries {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#level_reward_item: read_array(
                offset,
                2usize,
                32usize,
                |offset| {
                    Result::Ok(PvPSeries_LevelRewardItem::populate(row, offset)?)
                },
            )?,
            r#level_reward_count: read_array(
                offset,
                2usize,
                32usize,
                |offset| {
                    Result::Ok(PvPSeries_LevelRewardCount::populate(row, offset)?)
                },
            )?,
        })
    }
}
