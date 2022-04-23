use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use crate::utility::read_array;
impl MetadataAdapter for PlantPotFlowerSeed {
    fn name() -> String {
        "PlantPotFlowerSeed".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PlantPotFlowerSeed::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PlantPotFlowerSeed {
    pub r#seed_icon: Vec<u32>,
}
impl PlantPotFlowerSeed {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#seed_icon: read_array(
                offset,
                9usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
