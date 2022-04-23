use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::error::PopulateError;
impl MetadataAdapter for TutorialTank {
    fn name() -> String {
        "TutorialTank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TutorialTank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TutorialTank {
    pub r#objective: u8,
}
impl TutorialTank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#objective: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
