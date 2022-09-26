use std::result::Result;
use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::vec::Vec;
use ironworks::excel::Row;
impl MetadataAdapter for JobHudManualPriority {
    fn name() -> String {
        "JobHudManualPriority".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(JobHudManualPriority::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct JobHudManualPriority {
    pub r#job_hud_manual: Vec<u8>,
}
impl JobHudManualPriority {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#job_hud_manual: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
