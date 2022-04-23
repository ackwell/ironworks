use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
use crate::error::PopulateError;
use std::convert::Infallible;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
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
    pub r#member: u32,
    pub r#select_image: Vec<u32>,
    pub r#portrait_image: Vec<u32>,
    pub r#class: Option<Infallible>,
}
impl DawnGrowMember {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#member: row.field(0usize + offset)?.into_u32()?,
            r#select_image: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u32()?) },
            )?,
            r#portrait_image: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(4usize + offset)?.into_u32()?) },
            )?,
            r#class: None,
        })
    }
}
