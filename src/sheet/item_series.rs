use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for ItemSeries {
    fn name() -> String {
        "ItemSeries".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemSeries::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemSeries {
    pub r#name: SeString,
}
impl ItemSeries {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
