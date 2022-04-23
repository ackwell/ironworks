use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
impl MetadataAdapter for HWDGathereInspectTerm {
    fn name() -> String {
        "HWDGathereInspectTerm".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDGathereInspectTerm::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDGathereInspectTerm {
    pub r#name: SeString,
}
impl HWDGathereInspectTerm {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
        })
    }
}
