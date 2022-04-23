use crate::utility::read_array;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
impl MetadataAdapter for GatheringRarePopTimeTable {
    fn name() -> String {
        "GatheringRarePopTimeTable".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringRarePopTimeTable::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringRarePopTimeTable_rtim {
    pub r#start_time: u16,
    pub r#durationm: u16,
}
impl GatheringRarePopTimeTable_rtim {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#start_time: row.field(0usize + offset)?.into_u16()?,
            r#durationm: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct GatheringRarePopTimeTable {
    pub r#rtim: Vec<GatheringRarePopTimeTable_rtim>,
}
impl GatheringRarePopTimeTable {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#rtim: read_array(
                offset,
                3usize,
                2usize,
                |offset| {
                    Result::Ok(GatheringRarePopTimeTable_rtim::populate(row, offset)?)
                },
            )?,
        })
    }
}
