use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for Description {
    fn name() -> String {
        "Description".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Description::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Description {
    pub r#quest: u32,
    pub r#text_long: SeString,
    pub r#text_short: SeString,
    pub r#text_commentary: SeString,
    pub r#unknown4: bool,
    pub r#section: u32,
}
impl Description {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#quest: row.field(0usize + offset)?.into_u32()?,
            r#text_long: row.field(1usize + offset)?.into_string()?,
            r#text_short: row.field(2usize + offset)?.into_string()?,
            r#text_commentary: row.field(3usize + offset)?.into_string()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#section: row.field(5usize + offset)?.into_u32()?,
        })
    }
}
