use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for OmikujiGuidance {
    fn name() -> String {
        "OmikujiGuidance".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(OmikujiGuidance::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct OmikujiGuidance {
    pub r#unknown0: bool,
    pub r#unknown1: SeString,
    pub r#unknown2: SeString,
}
impl OmikujiGuidance {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#unknown1: row.field(1usize + offset)?.into_string()?,
            r#unknown2: row.field(2usize + offset)?.into_string()?,
        })
    }
}
