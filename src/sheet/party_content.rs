use std::vec::Vec;
use std::result::Result;
use crate::error::PopulateError;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for PartyContent {
    fn name() -> String {
        "PartyContent".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PartyContent::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PartyContent {
    pub r#key: u8,
    pub r#time_limit: u16,
    pub r#name: bool,
    pub r#text_data_start: u32,
    pub r#text_data_end: u32,
    pub r#lgb_event_object0: Vec<u32>,
    pub r#lgb_event_range: Vec<u32>,
    pub r#lgb_event_object1: Vec<u32>,
    pub r#unknown32: u16,
    pub r#content_finder_condition: u16,
    pub r#image: u32,
}
impl PartyContent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#key: row.field(0usize + offset)?.into_u8()?,
            r#time_limit: row.field(1usize + offset)?.into_u16()?,
            r#name: row.field(2usize + offset)?.into_bool()?,
            r#text_data_start: row.field(3usize + offset)?.into_u32()?,
            r#text_data_end: row.field(4usize + offset)?.into_u32()?,
            r#lgb_event_object0: read_array(
                offset,
                9usize,
                1usize,
                |offset| { Result::Ok(row.field(5usize + offset)?.into_u32()?) },
            )?,
            r#lgb_event_range: read_array(
                offset,
                9usize,
                1usize,
                |offset| { Result::Ok(row.field(14usize + offset)?.into_u32()?) },
            )?,
            r#lgb_event_object1: read_array(
                offset,
                9usize,
                1usize,
                |offset| { Result::Ok(row.field(23usize + offset)?.into_u32()?) },
            )?,
            r#unknown32: row.field(32usize + offset)?.into_u16()?,
            r#content_finder_condition: row.field(33usize + offset)?.into_u16()?,
            r#image: row.field(34usize + offset)?.into_u32()?,
        })
    }
}
