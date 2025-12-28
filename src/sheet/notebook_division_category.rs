use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for NotebookDivisionCategory {
    fn name() -> String {
        "NotebookDivisionCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(NotebookDivisionCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct NotebookDivisionCategory {
    pub r#name: SeString,
    pub r#index: u8,
}
impl NotebookDivisionCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#index: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
