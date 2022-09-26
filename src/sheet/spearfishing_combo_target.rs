use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for SpearfishingComboTarget {
    fn name() -> String {
        "SpearfishingComboTarget".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SpearfishingComboTarget::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SpearfishingComboTarget {}
impl SpearfishingComboTarget {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
