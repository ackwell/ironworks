use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
    pub r#unknown1: bool,
}
impl EventItemHelp {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#description: row.field(0usize + offset)?.into_string()?,
            r#unknown1: row.field(1usize + offset)?.into_bool()?,
        })
    }
}
