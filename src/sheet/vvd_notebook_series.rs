use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use std::vec::Vec;
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
pub struct VVDNotebookSeries {
    pub r#name: SeString,
    pub r#contents: Vec<i32>,
    pub r#unknown13: i32,
}
impl VVDNotebookSeries {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#contents: read_array(
                offset,
                12usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_i32()?) },
            )?,
            r#unknown13: row.field(13usize + offset)?.into_i32()?,
        })
    }
}
