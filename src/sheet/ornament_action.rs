use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for OrnamentAction {
    fn name() -> String {
        "OrnamentAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(OrnamentAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct OrnamentAction {}
impl OrnamentAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
