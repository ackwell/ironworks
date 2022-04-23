use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for ModelState {
    fn name() -> String {
        "ModelState".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ModelState::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ModelState {
    pub r#unknown0: u8,
    pub r#start: u16,
}
impl ModelState {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#start: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
