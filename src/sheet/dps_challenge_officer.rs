use std::vec::Vec;
use crate::error::PopulateError;
use crate::utility::read_array;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for DpsChallengeOfficer {
    fn name() -> String {
        "DpsChallengeOfficer".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(DpsChallengeOfficer::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct DpsChallengeOfficer {
    pub r#unlock_quest: u32,
    pub r#challenge_name: Vec<u16>,
}
impl DpsChallengeOfficer {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unlock_quest: row.field(0usize + offset)?.into_u32()?,
            r#challenge_name: read_array(
                offset,
                25usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
