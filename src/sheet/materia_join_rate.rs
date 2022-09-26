use std::vec::Vec;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use crate::utility::read_array;
use crate::error::PopulateError;
impl MetadataAdapter for MateriaJoinRate {
    fn name() -> String {
        "MateriaJoinRate".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MateriaJoinRate::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MateriaJoinRate {
    pub r#nq_overmeld_percent_slot: Vec<f32>,
    pub r#hq_overmeld_percent_slot: Vec<f32>,
}
impl MateriaJoinRate {
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
