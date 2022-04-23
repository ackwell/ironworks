use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for SpearfishingSilhouette {
    fn name() -> String {
        "SpearfishingSilhouette".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SpearfishingSilhouette::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SpearfishingSilhouette {}
impl SpearfishingSilhouette {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
