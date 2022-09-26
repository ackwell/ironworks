use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MateriaGrade {
    fn name() -> String {
        "MateriaGrade".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MateriaGrade::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MateriaGrade {}
impl MateriaGrade {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
