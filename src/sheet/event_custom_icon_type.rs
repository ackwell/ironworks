use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for EventCustomIconType {
    fn name() -> String {
        "EventCustomIconType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventCustomIconType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventCustomIconType {
    pub r#announce_quest: Vec<u32>,
    pub r#announce_quest_locked: Vec<u32>,
    pub r#map_announce_quest0: Vec<u32>,
    pub r#map_announce_quest_locked: Vec<u32>,
    pub r#map_announce_quest1: Vec<u32>,
    pub r#unknown50: u8,
}
impl EventCustomIconType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#announce_quest: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
            r#announce_quest_locked: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(row.field(10usize + offset)?.into_u32()?) },
            )?,
            r#map_announce_quest0: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(row.field(20usize + offset)?.into_u32()?) },
            )?,
            r#map_announce_quest_locked: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(row.field(30usize + offset)?.into_u32()?) },
            )?,
            r#map_announce_quest1: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(row.field(40usize + offset)?.into_u32()?) },
            )?,
            r#unknown50: row.field(50usize + offset)?.into_u8()?,
        })
    }
}
