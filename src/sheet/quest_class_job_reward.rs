use std::vec::Vec;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
impl MetadataAdapter for QuestClassJobReward {
    fn name() -> String {
        "QuestClassJobReward".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuestClassJobReward::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuestClassJobReward {
    pub r#class_job_category: u8,
    pub r#reward_item: Vec<u32>,
    pub r#reward_amount: Vec<u8>,
    pub r#required_item: Vec<u32>,
    pub r#required_amount: Vec<u8>,
}
impl QuestClassJobReward {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#class_job_category: row.field(0usize + offset)?.into_u8()?,
            r#reward_item: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u32()?) },
            )?,
            r#reward_amount: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(5usize + offset)?.into_u8()?) },
            )?,
            r#required_item: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(9usize + offset)?.into_u32()?) },
            )?,
            r#required_amount: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(13usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
