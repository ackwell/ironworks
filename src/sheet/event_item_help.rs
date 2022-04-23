use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for EventItemHelp {
    fn name() -> String {
        "EventItemHelp".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventItemHelp::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventItemHelp {
    pub r#description: SeString,
}
impl EventItemHelp {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#description: row.field(0usize + offset)?.into_string()?,
        })
    }
}
