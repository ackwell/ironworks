use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
impl MetadataAdapter for ItemSpecialBonus {
    fn name() -> String {
        "ItemSpecialBonus".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemSpecialBonus::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemSpecialBonus {
    pub r#name: SeString,
}
impl ItemSpecialBonus {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
