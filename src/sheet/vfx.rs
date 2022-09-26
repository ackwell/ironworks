use crate::error::PopulateError;
use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for VFX {
    fn name() -> String {
        "VFX".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(VFX::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct VFX {
    pub r#location: SeString,
}
impl VFX {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#location: row.field(0usize + offset)?.into_string()?,
        })
    }
}
