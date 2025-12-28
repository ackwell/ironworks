use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for CSBonusContent {
    fn name() -> String {
        "CSBonusContent".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CSBonusContent::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CSBonusContent {
    pub r#content_type: u8,
    pub r#content: Vec<u16>,
    pub r#score: Vec<u16>,
    pub r#reward_count: Vec<u8>,
}
impl CSBonusContent {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content_type: row.field(0usize + offset)?.into_u8()?,
            r#content: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u16()?) },
            )?,
            r#score: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_u16()?) },
            )?,
            r#reward_count: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(9usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
