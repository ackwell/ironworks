use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for PvPSeriesLevel {
    fn name() -> String {
        "PvPSeriesLevel".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PvPSeriesLevel::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PvPSeriesLevel {}
impl PvPSeriesLevel {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
