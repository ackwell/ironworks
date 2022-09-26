use crate::error::PopulateError;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for YardCatalogCategory {
    fn name() -> String {
        "YardCatalogCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(YardCatalogCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct YardCatalogCategory {
    pub r#category: SeString,
}
impl YardCatalogCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_string()?,
        })
    }
}
