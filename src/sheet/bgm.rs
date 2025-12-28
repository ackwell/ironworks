use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for BGM {
    fn name() -> String {
        "BGM".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BGM::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BGM {
    pub r#file: SeString,
    pub r#priority: u8,
    pub r#disable_restart_time_out: bool,
    pub r#disable_restart: bool,
    pub r#pass_end: bool,
    pub r#disable_restart_reset_time: f32,
    pub r#special_mode: u8,
}
impl BGM {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#file: row.field(0usize + offset)?.into_string()?,
            r#priority: row.field(1usize + offset)?.into_u8()?,
            r#disable_restart_time_out: row.field(2usize + offset)?.into_bool()?,
            r#disable_restart: row.field(3usize + offset)?.into_bool()?,
            r#pass_end: row.field(4usize + offset)?.into_bool()?,
            r#disable_restart_reset_time: row.field(5usize + offset)?.into_f32()?,
            r#special_mode: row.field(6usize + offset)?.into_u8()?,
        })
    }
}
