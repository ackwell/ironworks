use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for DawnMember {
    fn name() -> String {
        "DawnMember".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DawnMember::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DawnMember {}
impl DawnMember {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
