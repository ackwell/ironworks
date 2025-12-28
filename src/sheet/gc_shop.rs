use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for GCShop {
    fn name() -> String {
        "GCShop".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GCShop::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GCShop {
    pub r#grand_company: i8,
}
impl GCShop {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#grand_company: row.field(0usize + offset)?.into_i8()?,
        })
    }
}
