use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for HWDDevLayerControl {
    fn name() -> String {
        "HWDDevLayerControl".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDDevLayerControl::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDDevLayerControl {}
impl HWDDevLayerControl {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
