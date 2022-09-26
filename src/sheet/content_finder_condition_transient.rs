use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
impl MetadataAdapter for ContentFinderConditionTransient {
    fn name() -> String {
        "ContentFinderConditionTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ContentFinderConditionTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ContentFinderConditionTransient {
    pub r#description: SeString,
}
impl ContentFinderConditionTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#description: row.field(0usize + offset)?.into_string()?,
        })
    }
}
