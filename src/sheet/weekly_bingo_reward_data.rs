use std::vec::Vec;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for WeeklyBingoRewardData {
    fn name() -> String {
        "WeeklyBingoRewardData".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WeeklyBingoRewardData::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WeeklyBingoRewardData_Reward {
    pub r#reward_type: u8,
    pub r#reward_item: u32,
    pub r#reward_hq: bool,
    pub r#reward_quantity: u16,
    pub r#reward_option: u8,
}
impl WeeklyBingoRewardData_Reward {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#reward_type: row.field(0usize + offset)?.into_u8()?,
            r#reward_item: row.field(1usize + offset)?.into_u32()?,
            r#reward_hq: row.field(2usize + offset)?.into_bool()?,
            r#reward_quantity: row.field(3usize + offset)?.into_u16()?,
            r#reward_option: row.field(4usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct WeeklyBingoRewardData {
    pub r#reward: Vec<WeeklyBingoRewardData_Reward>,
    pub r#reward_item2: u32,
    pub r#reward_hq2: bool,
    pub r#reward_quantity2: u16,
}
impl WeeklyBingoRewardData {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#reward: read_array(
                offset,
                2usize,
                5usize,
                |offset| {
                    Result::Ok(WeeklyBingoRewardData_Reward::populate(row, offset)?)
                },
            )?,
            r#reward_item2: row.field(10usize + offset)?.into_u32()?,
            r#reward_hq2: row.field(11usize + offset)?.into_bool()?,
            r#reward_quantity2: row.field(12usize + offset)?.into_u16()?,
        })
    }
}
