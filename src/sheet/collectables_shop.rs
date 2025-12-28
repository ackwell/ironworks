use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for CollectablesShop {
    fn name() -> String {
        "CollectablesShop".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CollectablesShop::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CollectablesShop {
    pub r#name: SeString,
    pub r#quest: u32,
    pub r#reward_type: u8,
    pub r#shop_items: Vec<u16>,
}
impl CollectablesShop {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#quest: row.field(1usize + offset)?.into_u32()?,
            r#reward_type: row.field(2usize + offset)?.into_u8()?,
            r#shop_items: read_array(
                offset,
                11usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
