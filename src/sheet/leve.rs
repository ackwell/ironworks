use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use ironworks::sestring::SeString;
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
    pub r#unknown13: u8,
    pub r#unknown14: bool,
    pub r#class_job_category: u8,
    pub r#journal_genre: i32,
    pub r#unknown17: i32,
    pub r#place_name_start_zone: i32,
    pub r#icon_city_state: i32,
    pub r#data_id: i32,
    pub r#can_cancel: bool,
    pub r#max_difficulty: u8,
    pub r#exp_factor: f32,
    pub r#exp_reward: u32,
    pub r#gil_reward: u32,
    pub r#leve_reward_item: u16,
    pub r#leve_vfx: u8,
    pub r#leve_vfx_frame: u8,
    pub r#level_levemete: u32,
    pub r#icon_issuer: i32,
    pub r#locked_leve: bool,
    pub r#level_start: u32,
    pub r#bgm: u16,
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
            r#unknown13: row.field(13usize + offset)?.into_u8()?,
            r#unknown14: row.field(14usize + offset)?.into_bool()?,
            r#class_job_category: row.field(15usize + offset)?.into_u8()?,
            r#journal_genre: row.field(16usize + offset)?.into_i32()?,
            r#unknown17: row.field(17usize + offset)?.into_i32()?,
            r#place_name_start_zone: row.field(18usize + offset)?.into_i32()?,
            r#icon_city_state: row.field(19usize + offset)?.into_i32()?,
            r#data_id: row.field(20usize + offset)?.into_i32()?,
            r#can_cancel: row.field(21usize + offset)?.into_bool()?,
            r#max_difficulty: row.field(22usize + offset)?.into_u8()?,
            r#exp_factor: row.field(23usize + offset)?.into_f32()?,
            r#exp_reward: row.field(24usize + offset)?.into_u32()?,
            r#gil_reward: row.field(25usize + offset)?.into_u32()?,
            r#leve_reward_item: row.field(26usize + offset)?.into_u16()?,
            r#leve_vfx: row.field(27usize + offset)?.into_u8()?,
            r#leve_vfx_frame: row.field(28usize + offset)?.into_u8()?,
            r#level_levemete: row.field(29usize + offset)?.into_u32()?,
            r#icon_issuer: row.field(30usize + offset)?.into_i32()?,
            r#locked_leve: row.field(31usize + offset)?.into_bool()?,
            r#level_start: row.field(32usize + offset)?.into_u32()?,
            r#bgm: row.field(33usize + offset)?.into_u16()?,
        })
    }
}
