use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for DawnContentParticipable {
    fn name() -> String {
        "DawnContentParticipable".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DawnContentParticipable::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DawnContentParticipable {}
impl DawnContentParticipable {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
