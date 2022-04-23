use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::vec::Vec;
impl MetadataAdapter for EventAction {
    fn name() -> String {
        "EventAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventAction {
    pub r#name: SeString,
    pub r#icon: u16,
    pub r#cast_time: u8,
    pub r#animation: Vec<u16>,
}
impl EventAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_u16()?,
            r#cast_time: row.field(2usize + offset)?.into_u8()?,
            r#animation: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
