use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for PetMirage {
    fn name() -> String {
        "PetMirage".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PetMirage::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PetMirage {
    pub r#unknown0: f32,
    pub r#unknown1: u16,
    pub r#name: SeString,
}
impl PetMirage {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_f32()?,
            r#unknown1: row.field(1usize + offset)?.into_u16()?,
            r#name: row.field(2usize + offset)?.into_string()?,
        })
    }
}
