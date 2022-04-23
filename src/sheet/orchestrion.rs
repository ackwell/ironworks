use ironworks::sestring::SeString;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for Orchestrion {
    fn name() -> String {
        "Orchestrion".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Orchestrion::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Orchestrion {
    pub r#name: SeString,
    pub r#description: SeString,
}
impl Orchestrion {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#description: row.field(1usize + offset)?.into_string()?,
        })
    }
}
