use std::result::Result;
use std::vec::Vec;
use crate::utility::read_array;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
impl MetadataAdapter for MJIVillageAppearanceSG {
    fn name() -> String {
        "MJIVillageAppearanceSG".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIVillageAppearanceSG::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIVillageAppearanceSG {
    pub r#sgb: Vec<u16>,
}
impl MJIVillageAppearanceSG {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#sgb: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
