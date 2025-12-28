use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for SnipeTalk {
    fn name() -> String {
        "SnipeTalk".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(SnipeTalk::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct SnipeTalk {
    pub r#unknown0: u8,
    pub r#unknown1: u8,
    pub r#name: u16,
    pub r#text: SeString,
    pub r#unknown4: SeString,
    pub r#unknown5: SeString,
}
impl SnipeTalk {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#unknown1: row.field(1usize + offset)?.into_u8()?,
            r#name: row.field(2usize + offset)?.into_u16()?,
            r#text: row.field(3usize + offset)?.into_string()?,
            r#unknown4: row.field(4usize + offset)?.into_string()?,
            r#unknown5: row.field(5usize + offset)?.into_string()?,
        })
    }
}
