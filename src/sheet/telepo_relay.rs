use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for TelepoRelay {
    fn name() -> String {
        "TelepoRelay".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TelepoRelay::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TelepoRelay {}
impl TelepoRelay {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
