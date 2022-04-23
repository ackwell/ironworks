use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for Leve {
    fn name() -> String {
        "Leve".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Leve::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Leve {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#leve_client: i32,
    pub r#leve_assignment_type: u8,
    pub r#unknown4: i32,
    pub r#town: i32,
    pub r#class_job_level: u16,
    pub r#time_limit: u8,
    pub r#allowance_cost: u8,
    pub r#evaluation: i32,
    pub r#place_name_start: i32,
    pub r#place_name_issued: i32,
    pub r#unknown12: u16,
    pub r#class_job_category: u8,
    pub r#journal_genre: bool,
    pub r#unknown15: u8,
    pub r#place_name_start_zone: i32,
    pub r#icon_city_state: i32,
    pub r#data_id: i32,
    pub r#can_cancel: i32,
    pub r#max_difficulty: i32,
    pub r#exp_factor: bool,
    pub r#exp_reward: u8,
    pub r#gil_reward: f32,
    pub r#leve_reward_item: u32,
    pub r#leve_vfx: u32,
    pub r#leve_vfx_frame: u16,
    pub r#level_levemete: u8,
    pub r#icon_issuer: u8,
    pub r#locked_leve: u32,
    pub r#level_start: i32,
    pub r#bgm: bool,
}
impl Leve {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#leve_client: row.field(2usize + offset)?.into_i32()?,
            r#leve_assignment_type: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_i32()?,
            r#town: row.field(5usize + offset)?.into_i32()?,
            r#class_job_level: row.field(6usize + offset)?.into_u16()?,
            r#time_limit: row.field(7usize + offset)?.into_u8()?,
            r#allowance_cost: row.field(8usize + offset)?.into_u8()?,
            r#evaluation: row.field(9usize + offset)?.into_i32()?,
            r#place_name_start: row.field(10usize + offset)?.into_i32()?,
            r#place_name_issued: row.field(11usize + offset)?.into_i32()?,
            r#unknown12: row.field(12usize + offset)?.into_u16()?,
            r#class_job_category: row.field(13usize + offset)?.into_u8()?,
            r#journal_genre: row.field(14usize + offset)?.into_bool()?,
            r#unknown15: row.field(15usize + offset)?.into_u8()?,
            r#place_name_start_zone: row.field(16usize + offset)?.into_i32()?,
            r#icon_city_state: row.field(17usize + offset)?.into_i32()?,
            r#data_id: row.field(18usize + offset)?.into_i32()?,
            r#can_cancel: row.field(19usize + offset)?.into_i32()?,
            r#max_difficulty: row.field(20usize + offset)?.into_i32()?,
            r#exp_factor: row.field(21usize + offset)?.into_bool()?,
            r#exp_reward: row.field(22usize + offset)?.into_u8()?,
            r#gil_reward: row.field(23usize + offset)?.into_f32()?,
            r#leve_reward_item: row.field(24usize + offset)?.into_u32()?,
            r#leve_vfx: row.field(25usize + offset)?.into_u32()?,
            r#leve_vfx_frame: row.field(26usize + offset)?.into_u16()?,
            r#level_levemete: row.field(27usize + offset)?.into_u8()?,
            r#icon_issuer: row.field(28usize + offset)?.into_u8()?,
            r#locked_leve: row.field(29usize + offset)?.into_u32()?,
            r#level_start: row.field(30usize + offset)?.into_i32()?,
            r#bgm: row.field(31usize + offset)?.into_bool()?,
        })
    }
}
