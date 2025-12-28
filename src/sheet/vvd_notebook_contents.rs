use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
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
pub struct VVDNotebookContents {
    pub r#icon: i32,
    pub r#image: i32,
    pub r#name: SeString,
    pub r#description: SeString,
}
impl VVDNotebookContents {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_i32()?,
            r#image: row.field(1usize + offset)?.into_i32()?,
            r#name: row.field(2usize + offset)?.into_string()?,
            r#description: row.field(3usize + offset)?.into_string()?,
        })
    }
}
