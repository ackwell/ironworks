use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::convert::Infallible;
use std::result::Result;
use std::vec::Vec;
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
    pub r#default_talk: Vec<Option<Infallible>>,
}
impl FateShop {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#special_shop: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
            r#default_talk: read_array(
                offset,
                10usize,
                1usize,
                |offset| { Result::Ok(None) },
            )?,
        })
    }
}
