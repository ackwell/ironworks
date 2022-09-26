use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
impl MetadataAdapter for PvPSelectTrait {
    fn name() -> String {
        "PvPSelectTrait".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PvPSelectTrait::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PvPSelectTrait {
    pub r#effect: SeString,
    pub r#icon: u32,
    pub r#value: i16,
}
impl PvPSelectTrait {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#effect: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_u32()?,
            r#value: row.field(2usize + offset)?.into_i16()?,
        })
    }
}
