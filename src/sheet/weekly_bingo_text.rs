use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for WeeklyBingoText {
    fn name() -> String {
        "WeeklyBingoText".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(WeeklyBingoText::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct WeeklyBingoText {
    pub r#description: SeString,
}
impl WeeklyBingoText {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#description: row.field(0usize + offset)?.into_string()?,
        })
    }
}
