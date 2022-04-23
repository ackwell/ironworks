use ironworks::sestring::SeString;
use std::result::Result;
use ironworks::excel::Row;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for UDS_Property {
    fn name() -> String {
        "UDS_Property".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(UDS_Property::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct UDS_Property {
    pub r#text: SeString,
    pub r#type: SeString,
}
impl UDS_Property {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
            r#type: row.field(1usize + offset)?.into_string()?,
        })
    }
}
