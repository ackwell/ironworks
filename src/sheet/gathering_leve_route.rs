use crate::utility::read_array;
use crate::metadata::MetadataAdapter;
use std::vec::Vec;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for GatheringLeveRoute {
    fn name() -> String {
        "GatheringLeveRoute".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringLeveRoute::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringLeveRoute_Pon {
    pub r#gathering_point: i32,
    pub r#pop_range: i32,
}
impl GatheringLeveRoute_Pon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#gathering_point: row.field(0usize + offset)?.into_i32()?,
            r#pop_range: row.field(1usize + offset)?.into_i32()?,
        })
    }
}
#[derive(Debug)]
pub struct GatheringLeveRoute {
    pub r#pon: Vec<GatheringLeveRoute_Pon>,
}
impl GatheringLeveRoute {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#pon: read_array(
                offset,
                12usize,
                2usize,
                |offset| { Result::Ok(GatheringLeveRoute_Pon::populate(row, offset)?) },
            )?,
        })
    }
}
