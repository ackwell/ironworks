use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ItemActionTelepo {
    fn name() -> String {
        "ItemActionTelepo".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemActionTelepo::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemActionTelepo {
    pub r#requirement: u32,
    pub r#deny_message: u32,
}
impl ItemActionTelepo {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#requirement: row.field(0usize + offset)?.into_u32()?,
            r#deny_message: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
