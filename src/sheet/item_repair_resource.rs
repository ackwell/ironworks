use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ItemRepairResource {
    fn name() -> String {
        "ItemRepairResource".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemRepairResource::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemRepairResource {
    pub r#item: u32,
}
impl ItemRepairResource {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
