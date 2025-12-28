use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for InclusionShopWelcomText {
    fn name() -> String {
        "InclusionShopWelcomText".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(InclusionShopWelcomText::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct InclusionShopWelcomText {
    pub r#unknown0: SeString,
}
impl InclusionShopWelcomText {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_string()?,
        })
    }
}
