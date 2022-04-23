use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for StatusHitEffect {
    fn name() -> String {
        "StatusHitEffect".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(StatusHitEffect::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct StatusHitEffect {
    pub r#location: u16,
}
impl StatusHitEffect {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#location: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
