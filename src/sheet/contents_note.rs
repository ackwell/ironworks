use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ContentsNote {
    fn name() -> String {
        "ContentsNote".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentsNote::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentsNote {
    pub r#content_type: u8,
    pub r#icon: i32,
    pub r#menu_order: u8,
    pub r#required_amount: i32,
    pub r#reward0: u8,
    pub r#exp_multiplier: i32,
    pub r#reward1: u8,
    pub r#gil_rward: i32,
    pub r#level_unlock: u16,
    pub r#how_to: u16,
    pub r#req_unlock: u32,
    pub r#name: SeString,
    pub r#description: SeString,
    pub r#exp_cap: i32,
}
impl ContentsNote {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#content_type: row.field(0usize + offset)?.into_u8()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#menu_order: row.field(2usize + offset)?.into_u8()?,
            r#required_amount: row.field(3usize + offset)?.into_i32()?,
            r#reward0: row.field(4usize + offset)?.into_u8()?,
            r#exp_multiplier: row.field(5usize + offset)?.into_i32()?,
            r#reward1: row.field(6usize + offset)?.into_u8()?,
            r#gil_rward: row.field(7usize + offset)?.into_i32()?,
            r#level_unlock: row.field(8usize + offset)?.into_u16()?,
            r#how_to: row.field(9usize + offset)?.into_u16()?,
            r#req_unlock: row.field(10usize + offset)?.into_u32()?,
            r#name: row.field(11usize + offset)?.into_string()?,
            r#description: row.field(12usize + offset)?.into_string()?,
            r#exp_cap: row.field(13usize + offset)?.into_i32()?,
        })
    }
}
