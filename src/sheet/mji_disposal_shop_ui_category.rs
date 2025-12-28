use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for MJIDisposalShopUICategory {
    fn name() -> String {
        "MJIDisposalShopUICategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIDisposalShopUICategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIDisposalShopUICategory {
    pub r#category: SeString,
}
impl MJIDisposalShopUICategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_string()?,
        })
    }
}
