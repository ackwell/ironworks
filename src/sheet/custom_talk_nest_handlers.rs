use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for CustomTalkNestHandlers {
    fn name() -> String {
        "CustomTalkNestHandlers".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CustomTalkNestHandlers::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CustomTalkNestHandlers {
    pub r#nest_handler: u32,
}
impl CustomTalkNestHandlers {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#nest_handler: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
