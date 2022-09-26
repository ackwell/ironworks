use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for VVDNotebookSeries {
    fn name() -> String {
        "VVDNotebookSeries".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(VVDNotebookSeries::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct VVDNotebookSeries {}
impl VVDNotebookSeries {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
