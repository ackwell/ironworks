use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for Tomestones {
    fn name() -> String {
        "Tomestones".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Tomestones::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Tomestones {
    pub r#weekly_limit: u16,
}
impl Tomestones {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#weekly_limit: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
