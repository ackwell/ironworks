use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for OmikujiGuidance {
    fn name() -> String {
        "OmikujiGuidance".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(OmikujiGuidance::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct OmikujiGuidance {}
impl OmikujiGuidance {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
