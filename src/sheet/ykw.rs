use std::vec::Vec;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
impl MetadataAdapter for YKW {
    fn name() -> String {
        "YKW".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(YKW::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct YKW {
    pub r#unknown0: u16,
    pub r#item: u32,
    pub r#location: Vec<u16>,
}
impl YKW {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#item: row.field(1usize + offset)?.into_u32()?,
            r#location: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
