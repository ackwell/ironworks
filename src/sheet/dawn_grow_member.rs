use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use crate::utility::read_array;
impl MetadataAdapter for DawnGrowMember {
    fn name() -> String {
        "DawnGrowMember".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DawnGrowMember::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DawnGrowMember {
    pub r#select_image: Vec<u32>,
    pub r#portrait_image: Vec<u32>,
    pub r#class: u8,
}
impl DawnGrowMember {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#select_image: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
            r#portrait_image: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_u32()?) },
            )?,
            r#class: row.field(6usize + offset)?.into_u8()?,
        })
    }
}
