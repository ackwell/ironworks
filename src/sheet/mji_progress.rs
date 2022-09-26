use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for MJIProgress {
    fn name() -> String {
        "MJIProgress".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIProgress::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIProgress {
    pub r#vision: SeString,
    pub r#objective: SeString,
    pub r#previous_objective: SeString,
}
impl MJIProgress {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#vision: row.field(0usize + offset)?.into_string()?,
            r#objective: row.field(1usize + offset)?.into_string()?,
            r#previous_objective: row.field(2usize + offset)?.into_string()?,
        })
    }
}
