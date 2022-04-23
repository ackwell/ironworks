use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for SwitchTalk {
    fn name() -> String {
        "SwitchTalk".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SwitchTalk::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SwitchTalk {}
impl SwitchTalk {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
