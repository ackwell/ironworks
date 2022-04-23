use std::vec::Vec;
use std::result::Result;
use crate::error::PopulateError;
use crate::utility::read_array;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for HowTo {
    fn name() -> String {
        "HowTo".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HowTo::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HowTo {
    pub r#name: SeString,
    pub r#announce: bool,
    pub r#how_to_page_pc: Vec<i16>,
    pub r#how_to_page_controller: Vec<i16>,
    pub r#category: i8,
    pub r#sort: u8,
}
impl HowTo {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#announce: row.field(1usize + offset)?.into_bool()?,
            r#how_to_page_pc: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_i16()?) },
            )?,
            r#how_to_page_controller: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(7usize + offset)?.into_i16()?) },
            )?,
            r#category: row.field(12usize + offset)?.into_i8()?,
            r#sort: row.field(13usize + offset)?.into_u8()?,
        })
    }
}
