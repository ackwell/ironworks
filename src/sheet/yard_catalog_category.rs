use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
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
    pub r#unknown1: u16,
    pub r#unknown2: u8,
}
impl YardCatalogCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#unknown2: row.field(2usize + offset)?.into_u8()?,
        })
    }
}
