use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
impl MetadataAdapter for GatheringPointBonusType {
    fn name() -> String {
        "GatheringPointBonusType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringPointBonusType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringPointBonusType {
    pub r#text: SeString,
}
impl GatheringPointBonusType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
        })
    }
}
