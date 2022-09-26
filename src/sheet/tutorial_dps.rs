use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for TutorialDPS {
    fn name() -> String {
        "TutorialDPS".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(TutorialDPS::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct TutorialDPS {
    pub r#objective: u8,
}
impl TutorialDPS {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#objective: row.field(0usize + offset)?.into_u8()?,
        })
    }
}
