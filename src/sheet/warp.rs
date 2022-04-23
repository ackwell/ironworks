use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Warp {
    fn name() -> String {
        "Warp".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Warp::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Warp {
    pub r#pop_range: u32,
    pub r#territory_type: u16,
    pub r#condition_success_event: u32,
    pub r#condition_fail_event: u32,
    pub r#confirm_event: u32,
    pub r#warp_condition: u16,
    pub r#warp_logic: u16,
    pub r#start_cutscene: u16,
    pub r#end_cutscene: u16,
    pub r#can_skip_cutscene: bool,
    pub r#name: SeString,
    pub r#question: SeString,
}
impl Warp {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#pop_range: row.field(0usize + offset)?.into_u32()?,
            r#territory_type: row.field(1usize + offset)?.into_u16()?,
            r#condition_success_event: row.field(2usize + offset)?.into_u32()?,
            r#condition_fail_event: row.field(3usize + offset)?.into_u32()?,
            r#confirm_event: row.field(4usize + offset)?.into_u32()?,
            r#warp_condition: row.field(5usize + offset)?.into_u16()?,
            r#warp_logic: row.field(6usize + offset)?.into_u16()?,
            r#start_cutscene: row.field(7usize + offset)?.into_u16()?,
            r#end_cutscene: row.field(8usize + offset)?.into_u16()?,
            r#can_skip_cutscene: row.field(9usize + offset)?.into_bool()?,
            r#name: row.field(10usize + offset)?.into_string()?,
            r#question: row.field(11usize + offset)?.into_string()?,
        })
    }
}
