use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::vec::Vec;
use crate::error::PopulateError;
use crate::utility::read_array;
impl MetadataAdapter for MJIFarmPastureRank {
    fn name() -> String {
        "MJIFarmPastureRank".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIFarmPastureRank::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIFarmPastureRank_SGB {
    pub r#sgb: Vec<u32>,
}
impl MJIFarmPastureRank_SGB {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#sgb: read_array(
                offset,
                4usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
#[derive(Debug)]
pub struct MJIFarmPastureRank {
    pub r#sgb: Vec<MJIFarmPastureRank_SGB>,
}
impl MJIFarmPastureRank {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#sgb: read_array(
                offset,
                4usize,
                4usize,
                |offset| { Result::Ok(MJIFarmPastureRank_SGB::populate(row, offset)?) },
            )?,
        })
    }
}
