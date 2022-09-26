use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
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
