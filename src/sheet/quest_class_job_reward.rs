use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
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
    pub r#required_amount: Vec<u32>,
    pub r#unknown17: u8,
    pub r#unknown18: u8,
    pub r#unknown19: u8,
    pub r#unknown20: u8,
    pub r#unknown21: bool,
    pub r#unknown22: bool,
    pub r#unknown23: bool,
    pub r#unknown24: bool,
    pub r#unknown25: bool,
    pub r#unknown26: bool,
    pub r#unknown27: bool,
    pub r#unknown28: bool,
    pub r#unknown29: bool,
    pub r#unknown30: bool,
    pub r#unknown31: bool,
    pub r#unknown32: bool,
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
                |offset| { Result::Ok(row.field(13usize + offset)?.into_u32()?) },
            )?,
            r#unknown17: row.field(17usize + offset)?.into_u8()?,
            r#unknown18: row.field(18usize + offset)?.into_u8()?,
            r#unknown19: row.field(19usize + offset)?.into_u8()?,
            r#unknown20: row.field(20usize + offset)?.into_u8()?,
            r#unknown21: row.field(21usize + offset)?.into_bool()?,
            r#unknown22: row.field(22usize + offset)?.into_bool()?,
            r#unknown23: row.field(23usize + offset)?.into_bool()?,
            r#unknown24: row.field(24usize + offset)?.into_bool()?,
            r#unknown25: row.field(25usize + offset)?.into_bool()?,
            r#unknown26: row.field(26usize + offset)?.into_bool()?,
            r#unknown27: row.field(27usize + offset)?.into_bool()?,
            r#unknown28: row.field(28usize + offset)?.into_bool()?,
            r#unknown29: row.field(29usize + offset)?.into_bool()?,
            r#unknown30: row.field(30usize + offset)?.into_bool()?,
            r#unknown31: row.field(31usize + offset)?.into_bool()?,
            r#unknown32: row.field(32usize + offset)?.into_bool()?,
        })
    }
}
