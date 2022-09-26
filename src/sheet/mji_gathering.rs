use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MJIGathering {
    fn name() -> String {
        "MJIGathering".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIGathering::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIGathering {
    pub r#gathering_object: u8,
}
impl MJIGathering {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#gathering_object: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
