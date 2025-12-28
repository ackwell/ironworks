use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for FashionCheckThemeCategory {
    fn name() -> String {
        "FashionCheckThemeCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FashionCheckThemeCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FashionCheckThemeCategory {
    pub r#name: SeString,
}
impl FashionCheckThemeCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
