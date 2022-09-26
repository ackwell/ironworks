use ironworks::sestring::SeString;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
impl MetadataAdapter for HowToCategory {
    fn name() -> String {
        "HowToCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HowToCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HowToCategory {
    pub r#category: SeString,
}
impl HowToCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#category: row.field(0usize + offset)?.into_string()?,
        })
    }
}
