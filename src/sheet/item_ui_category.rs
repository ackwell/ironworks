use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
impl MetadataAdapter for ItemUICategory {
    fn name() -> String {
        "ItemUICategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemUICategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemUICategory {
    pub r#name: SeString,
    pub r#icon: i32,
    pub r#order_minor: u8,
    pub r#order_major: u8,
}
impl ItemUICategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#order_minor: row.field(2usize + offset)?.into_u8()?,
            r#order_major: row.field(3usize + offset)?.into_u8()?,
        })
    }
}
