use ironworks::excel::Row;
use std::vec::Vec;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for CraftLeve {
    fn name() -> String {
        "CraftLeve".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CraftLeve::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CraftLeve_Item {
    pub r#item: i32,
    pub r#item_count: u16,
}
impl CraftLeve_Item {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(3usize + offset)?.into_i32()?,
            r#item_count: row.field(4usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct CraftLeve {
    pub r#leve: i32,
    pub r#craft_leve_talk: i32,
    pub r#repeats: u8,
    pub r#item: Vec<CraftLeve_Item>,
}
impl CraftLeve {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#leve: row.field(0usize + offset)?.into_i32()?,
            r#craft_leve_talk: row.field(1usize + offset)?.into_i32()?,
            r#repeats: row.field(2usize + offset)?.into_u8()?,
            r#item: read_array(
                offset,
                4usize,
                2usize,
                |offset| { Result::Ok(CraftLeve_Item::populate(row, offset)?) },
            )?,
        })
    }
}
