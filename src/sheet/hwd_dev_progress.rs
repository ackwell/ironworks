use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for HWDDevProgress {
    fn name() -> String {
        "HWDDevProgress".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDDevProgress::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDDevProgress {
    pub r#can_go_next: bool,
}
impl HWDDevProgress {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#can_go_next: row.field(0usize + offset)?.into_bool()?,
        })
    }
}
