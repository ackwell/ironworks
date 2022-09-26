use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::result::Result;
impl MetadataAdapter for FateShop {
    fn name() -> String {
        "FateShop".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FateShop::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FateShop {
    pub r#special_shop: Vec<u32>,
    pub r#default_talk: Vec<u32>,
}
impl FateShop {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#special_shop: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
            r#default_talk: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(2usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
