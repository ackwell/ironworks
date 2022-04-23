use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for YardCatalogItemList {
    fn name() -> String {
        "YardCatalogItemList".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(YardCatalogItemList::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct YardCatalogItemList {
    pub r#category: u16,
    pub r#item: i32,
    pub r#patch: u16,
}
impl YardCatalogItemList {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_u16()?,
            r#item: row.field(1usize + offset)?.into_i32()?,
            r#patch: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
