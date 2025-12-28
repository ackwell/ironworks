use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
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
    pub r#unknown129: i32,
    pub r#unknown130: i32,
    pub r#unknown131: i32,
    pub r#unknown132: i32,
    pub r#unknown133: i32,
    pub r#unknown134: i32,
    pub r#unknown135: i32,
    pub r#unknown136: i32,
    pub r#unknown137: i32,
    pub r#unknown138: i32,
    pub r#unknown139: i32,
    pub r#unknown140: i32,
    pub r#unknown141: i32,
    pub r#unknown142: i32,
    pub r#unknown143: i32,
    pub r#unknown144: i32,
    pub r#unknown145: i32,
    pub r#unknown146: i32,
    pub r#unknown147: i32,
    pub r#unknown148: i32,
    pub r#unknown149: i32,
    pub r#unknown150: i32,
    pub r#unknown151: i32,
    pub r#unknown152: i32,
    pub r#unknown153: i32,
    pub r#unknown154: i32,
    pub r#unknown155: i32,
    pub r#unknown156: i32,
    pub r#unknown157: i32,
    pub r#unknown158: i32,
    pub r#unknown159: i32,
    pub r#unknown160: i32,
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
            r#unknown129: row.field(129usize + offset)?.into_i32()?,
            r#unknown130: row.field(130usize + offset)?.into_i32()?,
            r#unknown131: row.field(131usize + offset)?.into_i32()?,
            r#unknown132: row.field(132usize + offset)?.into_i32()?,
            r#unknown133: row.field(133usize + offset)?.into_i32()?,
            r#unknown134: row.field(134usize + offset)?.into_i32()?,
            r#unknown135: row.field(135usize + offset)?.into_i32()?,
            r#unknown136: row.field(136usize + offset)?.into_i32()?,
            r#unknown137: row.field(137usize + offset)?.into_i32()?,
            r#unknown138: row.field(138usize + offset)?.into_i32()?,
            r#unknown139: row.field(139usize + offset)?.into_i32()?,
            r#unknown140: row.field(140usize + offset)?.into_i32()?,
            r#unknown141: row.field(141usize + offset)?.into_i32()?,
            r#unknown142: row.field(142usize + offset)?.into_i32()?,
            r#unknown143: row.field(143usize + offset)?.into_i32()?,
            r#unknown144: row.field(144usize + offset)?.into_i32()?,
            r#unknown145: row.field(145usize + offset)?.into_i32()?,
            r#unknown146: row.field(146usize + offset)?.into_i32()?,
            r#unknown147: row.field(147usize + offset)?.into_i32()?,
            r#unknown148: row.field(148usize + offset)?.into_i32()?,
            r#unknown149: row.field(149usize + offset)?.into_i32()?,
            r#unknown150: row.field(150usize + offset)?.into_i32()?,
            r#unknown151: row.field(151usize + offset)?.into_i32()?,
            r#unknown152: row.field(152usize + offset)?.into_i32()?,
            r#unknown153: row.field(153usize + offset)?.into_i32()?,
            r#unknown154: row.field(154usize + offset)?.into_i32()?,
            r#unknown155: row.field(155usize + offset)?.into_i32()?,
            r#unknown156: row.field(156usize + offset)?.into_i32()?,
            r#unknown157: row.field(157usize + offset)?.into_i32()?,
            r#unknown158: row.field(158usize + offset)?.into_i32()?,
            r#unknown159: row.field(159usize + offset)?.into_i32()?,
            r#unknown160: row.field(160usize + offset)?.into_i32()?,
        })
    }
}
