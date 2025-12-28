use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Lobby {
    fn name() -> String {
        "Lobby".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Lobby::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Lobby {
    pub r#type: u32,
    pub r#param: u32,
    pub r#link: u32,
    pub r#text: SeString,
    pub r#unknown4: SeString,
    pub r#unknown5: SeString,
}
impl Lobby {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u32()?,
            r#param: row.field(1usize + offset)?.into_u32()?,
            r#link: row.field(2usize + offset)?.into_u32()?,
            r#text: row.field(3usize + offset)?.into_string()?,
            r#unknown4: row.field(4usize + offset)?.into_string()?,
            r#unknown5: row.field(5usize + offset)?.into_string()?,
        })
    }
}
