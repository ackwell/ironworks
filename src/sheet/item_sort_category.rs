use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for ItemSortCategory {
    fn name() -> String {
        "ItemSortCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemSortCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemSortCategory {
    pub r#param: u8,
}
impl ItemSortCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#param: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
