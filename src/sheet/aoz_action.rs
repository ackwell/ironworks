use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for AozAction {
    fn name() -> String {
        "AozAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(AozAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct AozAction {
    pub r#action: u32,
    pub r#rank: u8,
}
impl AozAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#action: row.field(0usize + offset)?.into_u32()?,
            r#rank: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
