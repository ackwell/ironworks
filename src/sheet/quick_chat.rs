use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for QuickChat {
    fn name() -> String {
        "QuickChat".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(QuickChat::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct QuickChat {
    pub r#name_action: SeString,
    pub r#icon: i32,
    pub r#addon: i32,
    pub r#quick_chat_transient: i8,
    pub r#unknown4: u16,
}
impl QuickChat {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name_action: row.field(0usize + offset)?.into_string()?,
            r#icon: row.field(1usize + offset)?.into_i32()?,
            r#addon: row.field(2usize + offset)?.into_i32()?,
            r#quick_chat_transient: row.field(3usize + offset)?.into_i8()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
        })
    }
}
