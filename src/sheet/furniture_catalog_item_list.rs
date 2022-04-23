use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for FurnitureCatalogItemList {
    fn name() -> String {
        "FurnitureCatalogItemList".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FurnitureCatalogItemList::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FurnitureCatalogItemList {
    pub r#category: u16,
    pub r#item: i32,
    pub r#patch: u16,
}
impl FurnitureCatalogItemList {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_u16()?,
            r#item: row.field(1usize + offset)?.into_i32()?,
            r#patch: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
