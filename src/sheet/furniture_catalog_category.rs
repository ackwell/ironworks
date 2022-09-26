use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for FurnitureCatalogCategory {
    fn name() -> String {
        "FurnitureCatalogCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FurnitureCatalogCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FurnitureCatalogCategory {
    pub r#category: SeString,
}
impl FurnitureCatalogCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_string()?,
        })
    }
}
