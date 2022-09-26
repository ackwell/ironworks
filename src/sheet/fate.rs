use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::vec::Vec;
use crate::utility::read_array;
impl MetadataAdapter for Fate {
    fn name() -> String {
        "Fate".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Fate::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Fate {
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#objective: SeString,
    pub r#status_text: Vec<SeString>,
    pub r#eureka_fate: u8,
    pub r#rule: u8,
    pub r#fate_rule_ex: u16,
    pub r#location: u32,
    pub r#class_job_level: u8,
    pub r#class_job_level_max: u8,
    pub r#event_item: u32,
    pub r#type_to_do_value: Vec<u8>,
    pub r#icon_objective: u32,
    pub r#icon_map: u32,
    pub r#icon_inactive_map: u32,
    pub r#music: i32,
    pub r#lgb_guard_npc_location: u32,
    pub r#screen_image_accept: u16,
    pub r#screen_image_complete: u16,
    pub r#screen_image_failed: u16,
    pub r#unknown24: u8,
    pub r#required_quest: u32,
    pub r#special_fate: bool,
    pub r#unknown27: bool,
    pub r#given_status: u16,
    pub r#unknown29: u16,
    pub r#advent_event: bool,
    pub r#moon_faire_event: bool,
    pub r#unknown32: bool,
    pub r#fate_chain: u32,
    pub r#unknown34: u8,
    pub r#unknown35: u16,
    pub r#array_index: u32,
    pub r#unknown37: u32,
    pub r#req_event_item: u32,
    pub r#turn_in_event_item: u32,
    pub r#unknown40: u32,
    pub r#unknown41: u32,
    pub r#unknown42: u32,
    pub r#objective_icon: Vec<u32>,
}
impl Fate {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
            r#objective: row.field(2usize + offset)?.into_string()?,
            r#status_text: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_string()?) },
            )?,
            r#eureka_fate: row.field(6usize + offset)?.into_u8()?,
            r#rule: row.field(7usize + offset)?.into_u8()?,
            r#fate_rule_ex: row.field(8usize + offset)?.into_u16()?,
            r#location: row.field(9usize + offset)?.into_u32()?,
            r#class_job_level: row.field(10usize + offset)?.into_u8()?,
            r#class_job_level_max: row.field(11usize + offset)?.into_u8()?,
            r#event_item: row.field(12usize + offset)?.into_u32()?,
            r#type_to_do_value: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(13usize + offset)?.into_u8()?) },
            )?,
            r#icon_objective: row.field(16usize + offset)?.into_u32()?,
            r#icon_map: row.field(17usize + offset)?.into_u32()?,
            r#icon_inactive_map: row.field(18usize + offset)?.into_u32()?,
            r#music: row.field(19usize + offset)?.into_i32()?,
            r#lgb_guard_npc_location: row.field(20usize + offset)?.into_u32()?,
            r#screen_image_accept: row.field(21usize + offset)?.into_u16()?,
            r#screen_image_complete: row.field(22usize + offset)?.into_u16()?,
            r#screen_image_failed: row.field(23usize + offset)?.into_u16()?,
            r#unknown24: row.field(24usize + offset)?.into_u8()?,
            r#required_quest: row.field(25usize + offset)?.into_u32()?,
            r#special_fate: row.field(26usize + offset)?.into_bool()?,
            r#unknown27: row.field(27usize + offset)?.into_bool()?,
            r#given_status: row.field(28usize + offset)?.into_u16()?,
            r#unknown29: row.field(29usize + offset)?.into_u16()?,
            r#advent_event: row.field(30usize + offset)?.into_bool()?,
            r#moon_faire_event: row.field(31usize + offset)?.into_bool()?,
            r#unknown32: row.field(32usize + offset)?.into_bool()?,
            r#fate_chain: row.field(33usize + offset)?.into_u32()?,
            r#unknown34: row.field(34usize + offset)?.into_u8()?,
            r#unknown35: row.field(35usize + offset)?.into_u16()?,
            r#array_index: row.field(36usize + offset)?.into_u32()?,
            r#unknown37: row.field(37usize + offset)?.into_u32()?,
            r#req_event_item: row.field(38usize + offset)?.into_u32()?,
            r#turn_in_event_item: row.field(39usize + offset)?.into_u32()?,
            r#unknown40: row.field(40usize + offset)?.into_u32()?,
            r#unknown41: row.field(41usize + offset)?.into_u32()?,
            r#unknown42: row.field(42usize + offset)?.into_u32()?,
            r#objective_icon: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(43usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
