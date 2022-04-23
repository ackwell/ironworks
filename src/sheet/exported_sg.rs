use ironworks::excel::Row;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for ExportedSG {
    fn name() -> String {
        "ExportedSG".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ExportedSG::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ExportedSG {
    pub r#sgb_path: SeString,
}
impl ExportedSG {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#sgb_path: row.field(0usize + offset)?.into_string()?,
        })
    }
}
