use std::vec::Vec;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for DailySupplyItem {
    fn name() -> String {
        "DailySupplyItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DailySupplyItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DailySupplyItem_Unnamed0 {
    pub r#item: i32,
    pub r#quantity: u8,
    pub r#recipe_level: u8,
}
impl DailySupplyItem_Unnamed0 {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_i32()?,
            r#quantity: row.field(1usize + offset)?.into_u8()?,
            r#recipe_level: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct DailySupplyItem {
    pub r#unnamed0: Vec<DailySupplyItem_Unnamed0>,
}
impl DailySupplyItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unnamed0: read_array(
                offset,
                8usize,
                3usize,
                |offset| { Result::Ok(DailySupplyItem_Unnamed0::populate(row, offset)?) },
            )?,
        })
    }
}
