use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for GatheringLeveRule {
    fn name() -> String {
        "GatheringLeveRule".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GatheringLeveRule::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GatheringLeveRule {
    pub r#rule: SeString,
}
impl GatheringLeveRule {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#rule: row.field(0usize + offset)?.into_string()?,
        })
    }
}
