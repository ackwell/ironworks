use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for HWDDevLevelUI {
    fn name() -> String {
        "HWDDevLevelUI".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(HWDDevLevelUI::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct HWDDevLevelUI {}
impl HWDDevLevelUI {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {})
    }
}
