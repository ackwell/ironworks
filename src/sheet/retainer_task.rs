use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for RetainerTask {
    fn name() -> String {
        "RetainerTask".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RetainerTask::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RetainerTask {
    pub r#is_random: bool,
    pub r#class_job_category: u8,
    pub r#retainer_level: u8,
    pub r#unknown3: u16,
    pub r#retainer_task_parameter: u16,
    pub r#venture_cost: u16,
    pub r#max_timemin: u16,
    pub r#experience: i32,
    pub r#required_item_level: u16,
    pub r#condition_param0: u8,
    pub r#condition_param1: u8,
    pub r#required_gathering: u16,
    pub r#unknown12: u16,
    pub r#task: u16,
}
impl RetainerTask {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#is_random: row.field(0usize + offset)?.into_bool()?,
            r#class_job_category: row.field(1usize + offset)?.into_u8()?,
            r#retainer_level: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#retainer_task_parameter: row.field(4usize + offset)?.into_u16()?,
            r#venture_cost: row.field(5usize + offset)?.into_u16()?,
            r#max_timemin: row.field(6usize + offset)?.into_u16()?,
            r#experience: row.field(7usize + offset)?.into_i32()?,
            r#required_item_level: row.field(8usize + offset)?.into_u16()?,
            r#condition_param0: row.field(9usize + offset)?.into_u8()?,
            r#condition_param1: row.field(10usize + offset)?.into_u8()?,
            r#required_gathering: row.field(11usize + offset)?.into_u16()?,
            r#unknown12: row.field(12usize + offset)?.into_u16()?,
            r#task: row.field(13usize + offset)?.into_u16()?,
        })
    }
}
