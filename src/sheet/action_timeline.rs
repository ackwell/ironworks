use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for ActionTimeline {
    fn name() -> String {
        "ActionTimeline".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActionTimeline::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActionTimeline {
    pub r#type: u8,
    pub r#priority: u8,
    pub r#pause: bool,
    pub r#stance: u8,
    pub r#slot: u8,
    pub r#look_at_mode: u8,
    pub r#key: SeString,
    pub r#action_timeline_id_mode: u8,
    pub r#weapon_timeline: u16,
    pub r#load_type: u8,
    pub r#start_attach: u8,
    pub r#resident_pap: u8,
    pub r#unknown12: u16,
    pub r#kill_upper: bool,
    pub r#is_motion_canceled_by_moving: bool,
    pub r#unknown15: u8,
    pub r#unknown16: bool,
    pub r#unknown17: bool,
    pub r#is_loop: bool,
    pub r#unknown19: u8,
    pub r#unknown20: u8,
    pub r#unknown21: bool,
}
impl ActionTimeline {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#priority: row.field(1usize + offset)?.into_u8()?,
            r#pause: row.field(2usize + offset)?.into_bool()?,
            r#stance: row.field(3usize + offset)?.into_u8()?,
            r#slot: row.field(4usize + offset)?.into_u8()?,
            r#look_at_mode: row.field(5usize + offset)?.into_u8()?,
            r#key: row.field(6usize + offset)?.into_string()?,
            r#action_timeline_id_mode: row.field(7usize + offset)?.into_u8()?,
            r#weapon_timeline: row.field(8usize + offset)?.into_u16()?,
            r#load_type: row.field(9usize + offset)?.into_u8()?,
            r#start_attach: row.field(10usize + offset)?.into_u8()?,
            r#resident_pap: row.field(11usize + offset)?.into_u8()?,
            r#unknown12: row.field(12usize + offset)?.into_u16()?,
            r#kill_upper: row.field(13usize + offset)?.into_bool()?,
            r#is_motion_canceled_by_moving: row.field(14usize + offset)?.into_bool()?,
            r#unknown15: row.field(15usize + offset)?.into_u8()?,
            r#unknown16: row.field(16usize + offset)?.into_bool()?,
            r#unknown17: row.field(17usize + offset)?.into_bool()?,
            r#is_loop: row.field(18usize + offset)?.into_bool()?,
            r#unknown19: row.field(19usize + offset)?.into_u8()?,
            r#unknown20: row.field(20usize + offset)?.into_u8()?,
            r#unknown21: row.field(21usize + offset)?.into_bool()?,
        })
    }
}
