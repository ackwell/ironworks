use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for DisposalShop {
    fn name() -> String {
        "DisposalShop".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DisposalShop::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DisposalShop {
    pub r#shop_name: SeString,
}
impl DisposalShop {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#shop_name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
