use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::utility::read_array;
impl MetadataAdapter for InclusionShop {
    fn name() -> String {
        "InclusionShop".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InclusionShop::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InclusionShop {
    pub r#unknown0: u32,
    pub r#unknown1: u8,
    pub r#unknown2: SeString,
    pub r#category: Vec<u16>,
}
impl InclusionShop {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u32()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#unknown2: row.field(2usize + offset)?.into_string()?,
            r#category: read_array(
                offset,
                30usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
