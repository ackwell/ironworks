use ironworks::excel::Row;
use crate::utility::read_array;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use std::vec::Vec;
use ironworks::sestring::SeString;
impl MetadataAdapter for FateEvent {
    fn name() -> String {
        "FateEvent".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FateEvent::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FateEvent {
    pub r#turn: Vec<u8>,
    pub r#gesture: Vec<u32>,
    pub r#lip_sync: Vec<i32>,
    pub r#facial: Vec<i32>,
    pub r#shape: Vec<i32>,
    pub r#is_auto_shake: Vec<bool>,
    pub r#widget_type: Vec<u8>,
    pub r#text: Vec<SeString>,
}
impl FateEvent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#turn: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u8()?) },
            )?,
            r#gesture: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(8usize + offset)?.into_u32()?) },
            )?,
            r#lip_sync: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(16usize + offset)?.into_i32()?) },
            )?,
            r#facial: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(24usize + offset)?.into_i32()?) },
            )?,
            r#shape: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(32usize + offset)?.into_i32()?) },
            )?,
            r#is_auto_shake: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(40usize + offset)?.into_bool()?) },
            )?,
            r#widget_type: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(48usize + offset)?.into_u8()?) },
            )?,
            r#text: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(56usize + offset)?.into_string()?) },
            )?,
        })
    }
}
