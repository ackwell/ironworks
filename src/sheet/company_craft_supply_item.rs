use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for CompanyCraftSupplyItem {
    fn name() -> String {
        "CompanyCraftSupplyItem".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanyCraftSupplyItem::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanyCraftSupplyItem {
    pub r#item: u32,
}
impl CompanyCraftSupplyItem {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
