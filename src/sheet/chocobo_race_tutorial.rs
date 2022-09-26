use std::vec::Vec;
use crate::utility::read_array;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for ChocoboRaceTutorial {
    fn name() -> String {
        "ChocoboRaceTutorial".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ChocoboRaceTutorial::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ChocoboRaceTutorial {
    pub r#npc_yell: Vec<i32>,
}
impl ChocoboRaceTutorial {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#npc_yell: read_array(
                offset,
                8usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_i32()?) },
            )?,
        })
    }
}
