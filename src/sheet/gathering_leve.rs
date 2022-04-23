use std::vec::Vec;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use crate::utility::read_array;
use ironworks::excel::Row;
impl MetadataAdapter for GatheringLeve {
    fn name() -> String {
        "GatheringLeve".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringLeve::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringLeve {
    pub r#route: Vec<i32>,
    pub r#required_item0: i32,
    pub r#required_item_qty0: u8,
    pub r#required_item1: i32,
    pub r#required_item_qty1: u8,
    pub r#required_item2: i32,
    pub r#required_item_qty2: u8,
    pub r#required_item3: i32,
    pub r#required_item_qty3: u8,
    pub r#item_number: u8,
    pub r#rule: i32,
    pub r#varient: u8,
    pub r#objective0: u16,
    pub r#objective1: u16,
    pub r#b_npc_entry: i32,
    pub r#use_secondary_tool: bool,
}
impl GatheringLeve {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#route: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_i32()?) },
            )?,
            r#required_item0: row.field(4usize + offset)?.into_i32()?,
            r#required_item_qty0: row.field(5usize + offset)?.into_u8()?,
            r#required_item1: row.field(6usize + offset)?.into_i32()?,
            r#required_item_qty1: row.field(7usize + offset)?.into_u8()?,
            r#required_item2: row.field(8usize + offset)?.into_i32()?,
            r#required_item_qty2: row.field(9usize + offset)?.into_u8()?,
            r#required_item3: row.field(10usize + offset)?.into_i32()?,
            r#required_item_qty3: row.field(11usize + offset)?.into_u8()?,
            r#item_number: row.field(12usize + offset)?.into_u8()?,
            r#rule: row.field(13usize + offset)?.into_i32()?,
            r#varient: row.field(14usize + offset)?.into_u8()?,
            r#objective0: row.field(15usize + offset)?.into_u16()?,
            r#objective1: row.field(16usize + offset)?.into_u16()?,
            r#b_npc_entry: row.field(17usize + offset)?.into_i32()?,
            r#use_secondary_tool: row.field(18usize + offset)?.into_bool()?,
        })
    }
}
