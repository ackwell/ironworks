use crate::utility::read_array;
use ironworks::excel::Row;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for ItemFood {
    fn name() -> String {
        "ItemFood".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemFood::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemFood_a {
    pub r#base_param: u8,
    pub r#is_relative: bool,
    pub r#value: i8,
    pub r#max: i16,
    pub r#value_hq: i8,
    pub r#max_hq: i16,
}
impl ItemFood_a {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#base_param: row.field(1usize + offset)?.into_u8()?,
            r#is_relative: row.field(2usize + offset)?.into_bool()?,
            r#value: row.field(3usize + offset)?.into_i8()?,
            r#max: row.field(4usize + offset)?.into_i16()?,
            r#value_hq: row.field(5usize + offset)?.into_i8()?,
            r#max_hq: row.field(6usize + offset)?.into_i16()?,
        })
    }
}
#[derive(Debug)]
pub struct ItemFood {
    pub r#exp_bonus_percent: u8,
    pub r#a: Vec<ItemFood_a>,
}
impl ItemFood {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#exp_bonus_percent: row.field(0usize + offset)?.into_u8()?,
            r#a: read_array(
                offset,
                3usize,
                6usize,
                |offset| { Result::Ok(ItemFood_a::populate(row, offset)?) },
            )?,
        })
    }
}
