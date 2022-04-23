use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use crate::error::PopulateError;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for MateriaJoinRateGatherCraft {
    fn name() -> String {
        "MateriaJoinRateGatherCraft".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MateriaJoinRateGatherCraft::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MateriaJoinRateGatherCraft {
    pub r#nq_overmeld_percent_slot: Vec<f32>,
    pub r#hq_overmeld_percent_slot: Vec<f32>,
}
impl MateriaJoinRateGatherCraft {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#nq_overmeld_percent_slot: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_f32()?) },
            )?,
            r#hq_overmeld_percent_slot: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(4usize + offset)?.into_f32()?) },
            )?,
        })
    }
}
