use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for IKDSpot {
    fn name() -> String {
        "IKDSpot".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(IKDSpot::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct IKDSpot {
    pub r#spot_main: u32,
    pub r#spot_sub: u32,
    pub r#place_name: u32,
}
impl IKDSpot {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#spot_main: row.field(0usize + offset)?.into_u32()?,
            r#spot_sub: row.field(1usize + offset)?.into_u32()?,
            r#place_name: row.field(2usize + offset)?.into_u32()?,
        })
    }
}
