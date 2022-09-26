use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
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
