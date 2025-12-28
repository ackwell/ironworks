use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for ItemBarterCheck {
    fn name() -> String {
        "ItemBarterCheck".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemBarterCheck::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemBarterCheck {
    pub r#category: u16,
    pub r#question: u32,
    pub r#confirm: u32,
}
impl ItemBarterCheck {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_u16()?,
            r#question: row.field(1usize + offset)?.into_u32()?,
            r#confirm: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
