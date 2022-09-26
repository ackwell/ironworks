use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for VVDVariantAction {
    fn name() -> String {
        "VVDVariantAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(VVDVariantAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct VVDVariantAction {}
impl VVDVariantAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
