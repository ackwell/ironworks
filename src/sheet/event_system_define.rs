use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for EventSystemDefine {
    fn name() -> String {
        "EventSystemDefine".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EventSystemDefine::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EventSystemDefine {
    pub r#text: SeString,
    pub r#define_value: u32,
}
impl EventSystemDefine {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
            r#define_value: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
