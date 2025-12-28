use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for DpsChallengeTransient {
    fn name() -> String {
        "DpsChallengeTransient".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DpsChallengeTransient::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DpsChallengeTransient {
    pub r#instance_content: u16,
}
impl DpsChallengeTransient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#instance_content: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
