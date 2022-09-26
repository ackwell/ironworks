use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for InclusionShopCategory {
    fn name() -> String {
        "InclusionShopCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InclusionShopCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InclusionShopCategory {
    pub r#name: SeString,
    pub r#class_job_category: u8,
    pub r#inclusion_shop_series: u16,
}
impl InclusionShopCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#class_job_category: row.field(1usize + offset)?.into_u8()?,
            r#inclusion_shop_series: row.field(2usize + offset)?.into_u16()?,
        })
    }
}
