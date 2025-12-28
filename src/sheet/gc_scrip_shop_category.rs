use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for GCScripShopCategory {
    fn name() -> String {
        "GCScripShopCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GCScripShopCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GCScripShopCategory {
    pub r#grand_company: i8,
    pub r#tier: i8,
    pub r#sub_category: i8,
}
impl GCScripShopCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#grand_company: row.field(0usize + offset)?.into_i8()?,
            r#tier: row.field(1usize + offset)?.into_i8()?,
            r#sub_category: row.field(2usize + offset)?.into_i8()?,
        })
    }
}
