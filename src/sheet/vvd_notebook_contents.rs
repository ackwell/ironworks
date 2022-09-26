use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for VVDNotebookContents {
    fn name() -> String {
        "VVDNotebookContents".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(VVDNotebookContents::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct VVDNotebookContents {}
impl VVDNotebookContents {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
