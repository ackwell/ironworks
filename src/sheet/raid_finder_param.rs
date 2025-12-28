use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for RaidFinderParam {
    fn name() -> String {
        "RaidFinderParam".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(RaidFinderParam::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct RaidFinderParam {
    pub r#unknown0: bool,
}
impl RaidFinderParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
        })
    }
}
