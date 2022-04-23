use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use std::vec::Vec;
use crate::utility::read_array;
use crate::error::PopulateError;
impl MetadataAdapter for AnimaWeapon5TradeItem {
    fn name() -> String {
        "AnimaWeapon5TradeItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AnimaWeapon5TradeItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AnimaWeapon5TradeItem_Unnamed3 {
    pub r#item_name: u32,
    pub r#is_hq: bool,
    pub r#quantity: u8,
}
impl AnimaWeapon5TradeItem_Unnamed3 {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item_name: row.field(3usize + offset)?.into_u32()?,
            r#is_hq: row.field(4usize + offset)?.into_bool()?,
            r#quantity: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct AnimaWeapon5TradeItem {
    pub r#unknown0: u8,
    pub r#crystal_sand: u32,
    pub r#qty: u8,
    pub r#unnamed3: Vec<AnimaWeapon5TradeItem_Unnamed3>,
    pub r#category: u8,
}
impl AnimaWeapon5TradeItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#crystal_sand: row.field(1usize + offset)?.into_u32()?,
            r#qty: row.field(2usize + offset)?.into_u8()?,
            r#unnamed3: read_array(
                offset,
                8usize,
                3usize,
                |offset| {
                    Result::Ok(AnimaWeapon5TradeItem_Unnamed3::populate(row, offset)?)
                },
            )?,
            r#category: row.field(27usize + offset)?.into_u8()?,
        })
    }
}
