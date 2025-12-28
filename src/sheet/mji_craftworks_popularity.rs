use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for MJICraftworksPopularity {
    fn name() -> String {
        "MJICraftworksPopularity".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJICraftworksPopularity::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJICraftworksPopularity {
    pub r#popularity: Vec<u8>,
}
impl MJICraftworksPopularity {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#popularity: read_array(
                offset,
                91usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
